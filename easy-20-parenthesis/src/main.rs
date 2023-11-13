use std::env;

mod solution;

fn main() {

    let args: Vec<String> = env::args().collect();

    println!("input:   {:?}\nsolution: {:?}",args[1], solution::is_valid(args[1].as_str()));
}
