pub mod philosophers_dijkstra;
use std::time::Duration;

use philosophers_dijkstra::run;
fn main() {
    run(Some(Duration::from_secs(5)));
}
