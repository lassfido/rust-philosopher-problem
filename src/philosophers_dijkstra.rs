use crate::general_functions:: {
    PhilosopherArguments,
    State,
    StateMessage,
};

use std::sync::mpsc::Receiver;
use std::thread::{self, JoinHandle};
use std::sync::{Arc, Mutex, mpsc, MutexGuard};
use std::time::{Duration};
use std::rc::Rc;
use rand::Rng;

type Sender = mpsc::SyncSender<State>;

pub fn run(args: Option<PhilosopherArguments>) -> () {

    let mut args = args.unwrap_or_default();
    let should_process: bool;
    if args.state_sender.is_none() {
        should_process = true;
    }
    else
    {
        should_process = false;
    }
    let (philosophers, table, rx) = setup(&mut args);
    let mut philo_workers = Option::Some(PhilosopherPool::new(philosophers, table));


    let now = std::time::Instant::now();
    loop {
        if should_process{
            let received = rx.recv();
            match received {
                Ok(message) => println!("{}", message),
                Err(_) => break,
            }
        }
        match args.duration {
            Some(duration) => {
                if now.elapsed() > duration {
                    drop(philo_workers.take())
                }
            },
            None => continue,
        }
    }
}

fn setup(args: &mut PhilosopherArguments) -> (Vec<Philosopher>, Arc<Table>, Rc<Receiver<State>>) {

    let number_of_philosophers = args.number_of_philosophers as usize;

    let right = |index| {
        (index + 1) % number_of_philosophers
    };

    let (tx, rx) = mpsc::sync_channel(0);
    let sender = args.state_sender.take().unwrap_or(tx);


    let philosophers: Vec<Philosopher> = (0..number_of_philosophers).map(|index: usize| {
        let ctx = sender.clone();
        Philosopher::new(index, ctx, 
            index, right(index), 
            args.range_in_ms.unwrap_or_else(|| {(0,1000)})
        )
    }).collect();

    let table : Arc<Table> = Arc::new(Table {
        forks: (0..number_of_philosophers).map(|_| {
            Mutex::new(())
        }).collect(),
        }
    );

    (philosophers, table, Rc::new(rx))
}

struct Table {
    forks: Vec<Mutex<()>>
}

#[derive(Debug)]
struct Philosopher {
    index: usize,
    transmitter: Sender,
    left: usize,
    right: usize,
    range: (u32, u32),
}

struct PhilosopherPool {
    working_philosophers: Vec<Option<JoinHandle<()>>>,
    sender: Option<mpsc::Sender<()>>,

}

impl Philosopher {
    fn new(index: usize, transmitter: Sender, left: usize, right: usize, range: (u32, u32)) -> Self {
        Philosopher { 
            index, 
            transmitter, 
            left,
            right,
            range
        }
    }

    fn think(&self) {
        let time = rand::thread_rng().gen_range(self.range.0..self.range.1);
        let activity = State::Thinking(
            StateMessage { index: self.index as i32, for_a_time: Some(time) }
        );
        self.transmitter.send(activity).expect("Send failed!");
        thread::sleep(Duration::from_millis(time as u64));
    }

    fn take_forks<'a>(&'a self, table: &'a Table) -> (MutexGuard<()>, MutexGuard<()>) {
        let activity = State::Waiting(StateMessage { index: self.index as i32, for_a_time: None });
        self.transmitter.send(activity).expect("Send failed");
        let mut_guard_left_res = table.forks[self.left].lock();
        let mut_guard_right_res = table.forks[self.right].lock();

        let mut_guard_left = if let Ok(mut_guard_left) = mut_guard_left_res {
            mut_guard_left
        }
        else
        {
            table.forks[self.left].lock().unwrap()
        };
        
        let mut_guard_right = if let Ok(mut_guard_right) = mut_guard_right_res {
            mut_guard_right
        }
        else
        {
            table.forks[self.right].lock().unwrap()
        };
        
        (mut_guard_left, mut_guard_right)
    }

    fn eat(&self, table: &Table) {
        let (_mut_left, _mut_right) = self.take_forks(table);
        let time = rand::thread_rng().gen_range(self.range.0..self.range.1);
        let activity = State::Eating(
            StateMessage { index: self.index as i32, for_a_time: Some(time) }
        );
        self.transmitter.send(activity).expect("Send failed!");
        thread::sleep(Duration::from_millis(time as u64));
    }

}

impl PhilosopherPool {
    fn new(philosophers: Vec<Philosopher>, table: Arc<Table>) -> Self {

    let (sender, receiver) = mpsc::channel();
    let receiver = Arc::new(Mutex::new(receiver));
    let handles: Vec<_> = philosophers.into_iter().map(|philospher| {
        let table = table.clone();
        let receiver = receiver.clone(); 
        Some(thread::spawn(move || {
            loop {
                philospher.think();
                match receiver.lock().unwrap().try_recv(){
                    Ok(_) => (),
                    Err(error) => {
                        match error {
                            mpsc::TryRecvError::Empty => (),
                            mpsc::TryRecvError::Disconnected => break,
                        }
                    }
                }
                philospher.eat(&table);
            }
        }))
        }).collect();
        PhilosopherPool {
            working_philosophers: handles,
            sender: Some(sender)
        }
    }
}

impl Drop for PhilosopherPool {
    fn drop(&mut self) {
        drop(self.sender.take());
        for handle in &mut self.working_philosophers {
            handle.take().unwrap().join().expect("Error terminating thread");
        }
    }
}