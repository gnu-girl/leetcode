use std::env;

mod solution;

fn main() {

    let mut args: Vec<String>= env::args().skip(0).collect();

    // Remove the first element of the call (file path)
    args.remove(0);
    println!("input:   {:?}\nsolution: {:?}",args, solution::longest_common_prefix(&args));
}