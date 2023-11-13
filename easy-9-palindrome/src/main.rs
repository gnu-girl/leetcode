use std::env;

mod solution;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input: i32 = args[1].parse::<i32>().unwrap();

    println!("input:   {:?}\nsolution: {:?}",args[1], solution::is_palindrome_no_math(input));
}
