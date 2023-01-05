use std::sync::mpsc::Receiver;
use std::thread;
use std::sync::{Arc, Mutex, mpsc, MutexGuard};
use std::time::{Duration};
#[cfg(debug_assertions)]
use std::time::Instant;
use rand::Rng;
type Sender = mpsc::Sender<String>;

pub fn run() -> ! {

    let (philosophers, table, rx) = setup(6);
    let _handles: Vec<_> = philosophers.into_iter().map(|philospher| {
        let table = table.clone();
        thread::spawn(move || {
            loop {
                philospher.think();
                philospher.eat(&table);
            }
        })
    }).collect();

    loop {
        let received : String = rx.recv().unwrap_or_default();
        println!("{}", received);
    }
}

fn setup(number_of_philosophers: usize) -> (Vec<Philospher>, Arc<Table>, Receiver<String>) {

    let left = |index| {
        (index + number_of_philosophers - 1) % number_of_philosophers 
    };

    let right = |index| {
        (index + 1) % number_of_philosophers
    };

    let (tx, rx) = mpsc::channel();

    let philosophers: Vec<Philospher> = (0..number_of_philosophers).map(|index| {
        let ctx = tx.clone();
        Philospher::new(index, ctx, left(index), right(index))
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
struct Philospher {
    index: usize,
    transmitter: Sender,
    left: usize,
    right: usize,
}

impl Philospher {
    fn new(index: usize, transmitter: Sender, left: usize, right: usize) -> Self {
        Philospher { 
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

