use std::io::{self, Read};

fn main() {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut input = String::new();

    handle.read_to_string(&mut input).expect("Error occured while reading inputs");

    let mut input = input.split_whitespace().map(|s| {s.parse::<i32>().expect("Unable to parse")});

    let a = input.next().expect("Unable to get A");
    let b = input.next().expect("Unable to get B");
    let c = input.next().expect("Unable to get C");
    let s = format!("{a}{b}").parse::<i32>().expect("Unable to parse A+B");

    println!("{}\n{}", a+b-c, s-c);
}