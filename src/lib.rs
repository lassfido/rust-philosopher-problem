mod philosophers_dijkstra;
pub mod general_functions;


pub fn run(args: Option<general_functions::PhilosopherArguments>) {
    philosophers_dijkstra::run(args);
}