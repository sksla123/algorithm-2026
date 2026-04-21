use std::io::{self, Read};

fn read_all() -> String {
    let mut fstdin = io::stdin().lock();
    let mut raw_input = String::new();

    fstdin.read_to_string(&mut raw_input).expect("Failed to read input\n");
    raw_input
}

fn main() {
    let n = read_all()
        .split_whitespace()
        .map(|s| {s.parse::<i32>().expect("Failed to parse into integer\n")})
        .next()
        .expect("Failed to get one\n");

    for i in 1..=9 {
        println!("{0} * {1} = {2}", n, i, n*i);
    }
}