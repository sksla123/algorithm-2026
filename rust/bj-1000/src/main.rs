use std::io;

fn main() {
    let result: i32 = {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .unwrap();

        input.split_whitespace().map(|w| {w.parse::<i32>().unwrap()}).sum::<i32>()
    };

    println!("{}", result);
}
