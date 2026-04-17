use std::io::{self, Read};

fn read_all() -> String {
    let mut stdin = io::stdin().lock();
    let mut input = String::new();
    stdin.read_to_string(&mut input).expect("Failed to read");
    input
}

fn main() {
    let input = read_all();

    let mut tokens = input
        .split_whitespace()
        .map(|s| {s.parse::<usize>().expect("Failed to parse strin into u8")});

    let n = tokens.next().expect("Unable to load N");

    let max_star = "*".repeat(n);

    for i in 1..=n{
        println!("{}", &max_star[0..i]);
    }
}