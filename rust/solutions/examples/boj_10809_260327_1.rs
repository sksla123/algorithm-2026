use std::{io::{self, Read}, collections::HashMap};

fn read_all() -> String {
    let mut fstdin = io::stdin().lock();
    let mut raw_input = String::new();

    fstdin.read_to_string(&mut raw_input).expect("Failed to read.");
    raw_input
}

fn main() {
    let input = read_all();
    let chars = input.trim().bytes();

    let mut cpos: HashMap<u8, usize> = HashMap::new();

    for (i, c) in chars.enumerate(){
        cpos.entry(c).or_insert(i);
    }

    for a in b'a'..=b'z'{
        match cpos.get(&a){
            Some(pos) => print!("{pos} "), 
            None => print!("-1 ")
        }
    }
}