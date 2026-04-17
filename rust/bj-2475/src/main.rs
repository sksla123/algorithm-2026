use std::io::{self, Read};

fn read_all() -> String {
    let mut handler = io::stdin().lock();
    let mut input = String::new();

    handler.read_to_string(&mut input).expect("Failed to read.");
    input
}

fn main() {
    let raw_input = read_all();
    let tokens = raw_input
        .split_whitespace()
        .map(|s| {s.parse::<i32>().expect("Failed to parse")});

    let ss: i32 = tokens
        .map(|s| {s.pow(2)})
        .sum();

    println!("{}", ss%10);
}