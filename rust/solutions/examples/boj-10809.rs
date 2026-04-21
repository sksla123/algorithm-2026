use std::io::{self, Read};

fn read_all() -> String {
    let mut fstdin = io::stdin().lock();
    let mut raw_input = String::new();

    fstdin.read_to_string(&mut raw_input).expect("Failed to read.");
    raw_input
}

fn main() {
    let input = read_all();
    let chars = input.trim().bytes();

    let mut alphabets = [-1 as isize; 26];

    for (pos,c) in chars.enumerate() {
        let a_idx = c as usize - 'a' as usize;

        if alphabets[a_idx] ==  -1 {
            alphabets[a_idx] = pos as isize;
        }
    }

    for a in alphabets.iter() {
        print!("{} ", a);
    }
}