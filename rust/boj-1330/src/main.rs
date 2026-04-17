use std::io::{self, Read};

const RESULTS:[&str; 3] = ["<", "==", ">"];

fn main() {
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    let mut input = String::new();
    handle.read_to_string(&mut input).expect("Failed to read your input");

    let mut tokens = input.split_ascii_whitespace().map(|s| s.parse::<i32>().expect("Unable to parse"));

    let a = tokens.next().expect("Unable to read A");
    let b = tokens.next().expect("Unable to read B");

    unsafe{
        println!("{}", RESULTS.get_unchecked((a >= b) as usize + (a > b) as usize));
    }
}