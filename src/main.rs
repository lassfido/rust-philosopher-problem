use std::io::Write;
use std::str::FromStr;
use std::time::Duration;
use std::sync::mpsc;
use std::thread;
use rust_philosopher_problem::run;
use rust_philosopher_problem::general_functions::*;
use termion::{clear, cursor};

fn print_table(new_state: State, states: &mut Vec<String>)
{
    match new_state {
        State::Thinking(message) => {
            states[message.index as usize] = String::from_str("Thinking").unwrap();
        },
        State::Eating(message) => {
            states[message.index as usize] = String::from_str("Eating").unwrap();
        },
        State::Waiting(message) => {
            states[message.index as usize] = String::from_str("Waiting").unwrap();
        },
    }

    print!("{}", cursor::Goto(1,2));
    print!("{}", clear::AfterCursor);
    for state_string in states {
        print!("{:<13} |", state_string);
    }
    std::io::stdout().flush().unwrap();
}
fn main() {
    let mut args = PhilosopherArguments::default();
    let (tx, rx) = mpsc::sync_channel(0);
    args.range_in_ms = Some((1000, 5000));
    args.duration = Some(Duration::from_secs(60));
    args.state_sender = Some(tx);
    let mut states: Vec<String> = Vec::with_capacity(args.number_of_philosophers as usize);
    for _ in 0..args.number_of_philosophers{
        states.push(String::from_str("Nothing").unwrap());
    }

    print!("{}", clear::All);
    print!("{}", cursor::Goto(1,1));
    for i in 0..args.number_of_philosophers{
        print!("Philosopher {:3>} |", i);
    }

    thread::spawn(move || run(Some(args)));

    loop {
        let received = rx.recv();
        match received {
            Ok(message) => print_table(message, &mut states),
            Err(_) => break,
        }
        
    }
}
