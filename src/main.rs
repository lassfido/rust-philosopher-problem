use std::time::Duration;
use std::sync::mpsc;
use std::thread;
use rust_philosopher_problem::run;
use rust_philosopher_problem::general_functions::*;
fn main() {
    let mut args = PhilosopherArguments::default();
    let (tx, rx) = mpsc::channel();
    args.range_in_ms = Some((1000, 5000));
    args.duration = Some(Duration::from_secs(5));
    args.state_sender = Some(tx);
    thread::spawn(move || run(Some(args)));
    loop {
        let received = rx.recv();
        match received {
            Ok(message) => println!("{}", message),
            Err(_) => break,
        }
        
    }
}
