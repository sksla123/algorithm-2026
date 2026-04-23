use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    let mut A = String::new();
    handle.read_line(&mut A).expect("Unable to get 1st input!");

    let mut B = String::new();
    handle.read_line(&mut B).expect("Unable to get 2nd input!");

    let mut C = String::new();
    handle.read_line(&mut C).expect("Unable to get 3rd input!");

    let A = A.trim();
    let B = B.trim();
    let C = C.trim();

    let result_int = {
        A.parse::<i32>().expect("A is not a number") + B.parse::<i32>().expect("B is not a number") - C.parse::<i32>().expect("C is not a number")
    };

    let result_seq = {
        let s = String::from(A) + &B;
        s.parse::<i32>().expect("A + B is not a number") - C.parse::<i32>().expect("B is not a number")
    };

    println!("{0}\n{1}", result_int, result_seq);
}