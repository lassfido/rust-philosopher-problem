use std::time::Duration;
use rust_philosopher_problem::run;
use rust_philosopher_problem::general_functions::*;
fn main() {
    let mut args = PhilosopherArguments::default();
    args.range_in_ms = Some((1000, 5000));
    args.duration = Some(Duration::from_secs(5));
    run(Some(args));
}
