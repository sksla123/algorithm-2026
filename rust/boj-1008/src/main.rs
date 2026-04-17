use std::io;

fn main() {
    let result = {
        let mut input = String::new();
        
        io::stdin()
            .read_line(&mut input)
            .unwrap();

        let mut iter = input.split_whitespace().map(|x| {x.parse::<f64>().unwrap()});
        let first = iter.next().unwrap();

        iter.fold(first, |acc, x| {acc / x})
    };

    println!("{}", result);
}
