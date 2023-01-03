use std::sync::Mutex;
use std::{thread};
use std::sync::mpsc;
use std::time::Duration;
use std::cell::RefCell;
type Sender = mpsc::Sender<String>;
use rand::Rng;
use rand::rngs::ThreadRng;

pub fn run() -> ! {
    let number_of_philosphers = 4;
    let states = Mutex::new(vec!(State::Thinking));
    let (tx, rx) = mpsc::channel();
    for i in 0..number_of_philosphers {
        let ctx = tx.clone();
        thread::spawn(move || Philospher::new(number_of_philosphers, i, ctx).start_working());
    }

    loop {
        let received : String = rx.recv().unwrap_or_default();
        println!("{}", received);
    }
}

struct Philospher {
    total_number_of_philosophers: i32,
    i_am_philosopher: i32,
    transmit_to_main_thread: Sender,
    random_number_generator: RefCell<ThreadRng>,
    state: State,
}

impl Philospher {
    fn new(number_of_philosophers: i32, index: i32, transmitter: Sender) -> Self {
        Philospher { 
            total_number_of_philosophers: number_of_philosophers, 
            i_am_philosopher: index, 
            transmit_to_main_thread: transmitter, 
            random_number_generator: RefCell::new(rand::thread_rng()), 
            state: State::Thinking,
        }
    }
    fn start_working(&self) -> ! {
        loop {
            self.think();
            self.take_forks();
            self.eat();
            self.put_forks();
        }
    }

    fn think(&self) {
        let time = self.random_number_generator.borrow_mut().gen_range(0..1000);
        self.transmit_to_main_thread.send(format!("Philosopher {} is thinking for {} ms!", self.i_am_philosopher, time ).to_string()).expect("Send failed!");
        thread::sleep(Duration::from_millis(time));
    }

    fn take_forks(&self) {

    }

    fn eat(&self) {
        let time = self.random_number_generator.borrow_mut().gen_range(0..1000);
        self.transmit_to_main_thread.send(format!("Philosopher {} is eating for {} ms!", self.i_am_philosopher, time ).to_string()).expect("Send failed!");
        thread::sleep(Duration::from_millis(time));
    }

    fn put_forks(&self){

    }
}

enum State {
    Thinking,
    Hungry,
    Eating,
}

