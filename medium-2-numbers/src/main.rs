use std::env;

mod solution;

fn main() {
    // Only get command line args and ignore file path
    let args: Vec<String>= env::args().skip(1).collect();

    println!("input:   {:?}\nsolution: {}",args, solution::convert_to_list(&args).unwrap());
}