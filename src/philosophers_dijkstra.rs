use std::sync::mpsc::Receiver;
use std::thread::{self, JoinHandle};
use std::sync::{Arc, Mutex, mpsc, MutexGuard};
use std::time::{Duration, Instant};
use rand::Rng;
type Sender = mpsc::Sender<String>;

pub fn run(for_a_duration: Option<Duration>) -> () {

    let (philosophers, table, rx) = setup(6);
    let mut philo_workers = Option::Some(PhilosopherPool::new(philosophers, table));
    let now = std::time::Instant::now();
    loop {
        let received = rx.recv();
        match received {
            Ok(message) => println!("{}", message),
            Err(_) => break,
        }
        match for_a_duration {
            Some(duration) => {
                if now.elapsed() > duration {
                    drop(philo_workers.take())
                }
            },
            None => continue,
        }
    }
}

fn setup(number_of_philosophers: usize) -> (Vec<Philosopher>, Arc<Table>, Receiver<String>) {

    let left = |index| {
        (index + number_of_philosophers - 1) % number_of_philosophers 
    };

    let right = |index| {
        (index + 1) % number_of_philosophers
    };

    let (tx, rx) = mpsc::channel();

    let philosophers: Vec<Philosopher> = (0..number_of_philosophers).map(|index| {
        let ctx = tx.clone();
        Philosopher::new(index, ctx, left(index), right(index))
    }).collect();

    let table : Arc<Table> = Arc::new(Table {
        forks: (0..number_of_philosophers).map(|_| {
            Mutex::new(())
        }).collect(),
        }
    );

    (philosophers, table, rx)
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
}

struct PhilosopherPool {
    working_philosophers: Vec<Option<JoinHandle<()>>>,
    sender: Option<mpsc::Sender<()>>,

}

impl Philosopher {
    fn new(index: usize, transmitter: Sender, left: usize, right: usize) -> Self {
        Philosopher { 
            index, 
            transmitter, 
            left,
            right
        }
    }

    fn send_log_message(&self, activity: &str, time: Option<&str>){
        let prefix = format!("Philosopher {} is {:<20}", self.index, activity);
        let mut time_string: String = String::new();
        match time {
            Some(time) => time_string = format!("for {}", time),
            None => ()
        }
        self.transmitter.send(prefix + time_string.as_str() + "!").expect("Send has failed!");
    }

    fn think(&self) {
        let time = rand::thread_rng().gen_range(0..1000);
        self.send_log_message("thinking", Some(format!("{:>3} ms", time).as_str()));
        thread::sleep(Duration::from_millis(time));
    }

    fn take_forks<'a>(&'a self, table: &'a Table) -> (MutexGuard<()>, MutexGuard<()>) {
        #[cfg(debug_assertions)]
        let now = Instant::now();

        let mut_guard_left = table.forks[self.left].lock().unwrap();
        let mut_guard_right = table.forks[self.right].lock().unwrap();
        #[cfg(debug_assertions)]
        self.transmitter.send(format!("Philosopher {} waited for {:>3} ms to eat!", self.index, now.elapsed().as_millis())).expect("Send failed!");
        (mut_guard_left, mut_guard_right)
    }

    fn eat(&self, table: &Table) {
        let (_mut_left, _mut_right) = self.take_forks(table);
        let time = rand::thread_rng().gen_range(0..1000);
        self.send_log_message("eating", Some(format!("{:>3} ms", time).as_str()));
        thread::sleep(Duration::from_millis(time));
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
                philospher.eat(&table);
                match receiver.lock().unwrap().try_recv(){
                    Ok(_) => (),
                    Err(error) => {
                        match error {
                            mpsc::TryRecvError::Empty => continue,
                            mpsc::TryRecvError::Disconnected => break,
                        }
                    }
                }
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