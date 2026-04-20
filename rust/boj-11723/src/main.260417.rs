use std::{collections::HashSet, io::{self, Read}};

fn read_all() -> String {
    let mut fstdin = io::stdin().lock();
    let mut raw_input = String::new();
    fstdin.read_to_string(&mut raw_input).expect("Failed to read");
    
    raw_input
}

enum Command {
    Add(i8),
    Remove(i8),
    Check(i8),
    Toggle(i8),
    All,
    Empty,
    Invalid
}

fn parse_command (command: &str) -> Command {
    let c:Vec<_> = command.split_whitespace().collect();

    match c.as_slice() {
        ["add", v] => {
            v.parse::<i8>()
                .map(|_v| {Command::Add(_v)})
                .expect("Failed to parse (Command add)")
        },
        ["remove", v] => {
            v.parse::<i8>()
                .map(|_v| {Command::Remove(_v)})
                .expect("Failed to parse (Command remove)")
        },
        ["check", v] => {
            v.parse::<i8>()
                .map(|_v| {Command::Check(_v)})
                .expect("Failed to parse (Command check)")
        },
        ["toggle", v] => {
            v.parse::<i8>()
                .map(|_v| {Command::Toggle(_v)})
                .expect("Failed to parse (Command toggle)")
        },
        ["all"] => {Command::All},
        ["empty"] => {Command::Empty},
        _ => {Command::Invalid}
    }
}

fn main() {
    let input = read_all();

    let mut s = HashSet::new();

    let user_commands = input
        .lines()
        .skip(1)
        .map(|cmd| {parse_command(cmd)});

    for cmd in user_commands{
        match cmd {
            Command::Add(v) => {s.insert(v);},
            Command::Remove(v) => {s.remove(&v);},
            Command::Check(v) => {println!("{}", s.contains(&v) as i32);},
            Command::Toggle(v) => {
                if s.contains(&v) {
                    s.remove(&v);
                }
                else {
                    s.insert(v);
                }
            },
            Command::All => {s = (1..=20).collect();},
            Command::Empty => {s.clear();},
            Command::Invalid => {std::process::exit(-1)}
        }
    }
}
