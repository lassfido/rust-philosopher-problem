use std::sync::mpsc;
use std::time::Duration;
use std::fmt::Display;
pub struct PhilosopherArguments {
    pub number_of_philosophers: i32,
    pub range_in_ms: Option<(u32,u32)>,
    pub duration: Option<Duration>,
    pub state_sender: Option<mpsc::SyncSender<State>>,
}

pub enum State {
    Eating(StateMessage),
    Thinking(StateMessage),
    Waiting(StateMessage)
}

pub struct StateMessage{
    pub index: i32,
    pub for_a_time: Option<u32>
}

impl Default for PhilosopherArguments {
    fn default() -> Self {
        PhilosopherArguments { 
            number_of_philosophers: 5,
            range_in_ms: (Some((0,1000))), 
            duration: None, 
            state_sender: None
        }
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use State::*;
        let mut state_string = String::new();
        let time: Option<u32>;
        let index: i32;
        let extract_state_message = |state_message: &StateMessage| -> (i32, Option<u32>)
        {
            (state_message.index, state_message.for_a_time)
        };
        match self {
            Eating(state_message) => {
                state_string.push_str("eating");
                (index, time) = extract_state_message(state_message);
            },
            Thinking(state_message) => {
                state_string.push_str("thinking");
                (index, time) = extract_state_message(state_message);
            },
            Waiting(state_message) => {
                state_string.push_str("waiting");
                (index, time) = extract_state_message(state_message);
            },
        };

        match time{

            Some(time) => f.write_fmt(format_args!("Philosopher {index:>3} is {activity:<10} for {time:>5} ms", index=index, activity=state_string, time=time )),
            None => f.write_fmt(format_args!("Philosopher {index:>3} is {activity:<10} to eat", index=index, activity=state_string)),
        }
        
        
    }
}