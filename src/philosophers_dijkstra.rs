use std::thread;
use std::sync::{Arc, Mutex, mpsc};
use std::time::Duration;
use rand::Rng;
type Sender = mpsc::Sender<String>;

pub fn run() -> ! {
    let number_of_philosphers = 6;
    let left = |index| {
        (index + number_of_philosphers - 1) % number_of_philosphers
    };

    let right = |index| {
        (index + 1) % number_of_philosphers
    };

    let (tx, rx) = mpsc::channel();
    let philosophers: Vec<Philospher> = (0..number_of_philosphers).map(|index| {
        let ctx = tx.clone();
        Philospher::new(index, ctx, left(index), right(index))
    }).collect();


    let table : Arc<Table> = Arc::new(Table {
        forks: (0..number_of_philosphers).map(|_| {
            Mutex::new(())
        }).collect(),
        }
    );

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

struct Table {
    forks: Vec<Mutex<()>>
}
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

    fn think(&self) {
        let time = rand::thread_rng().gen_range(0..1000);
        self.transmitter.send(format!("Philosopher {} is thinking for {} ms!", self.index, time ).to_string()).expect("Send failed!");
        thread::sleep(Duration::from_millis(time));
    }


    fn eat(&self, table: &Table) {
        let _mut_guard_left = table.forks[self.left].lock().unwrap();
        let _mut_guard_right = table.forks[self.right].lock().unwrap();
        let time = rand::thread_rng().gen_range(0..1000);
        self.transmitter.send(format!("Philosopher {} is eating for {} ms!", self.index, time ).to_string()).expect("Send failed!");
        thread::sleep(Duration::from_millis(time));
    }

}

