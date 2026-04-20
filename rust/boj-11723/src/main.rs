use std::io::{BufRead, BufReader, BufWriter, Write, stdin, stdout};
use std::str::{FromStr, SplitWhitespace};

enum Command {
    Add(u8),
    Remove(u8),
    Check(u8),
    Toggle(u8),
    All,
    Empty
}

impl Command {
    fn parse_next_u8(iter: &mut SplitWhitespace) -> Result<u8, ()> {
        iter.next()
            .and_then(|value| value.parse::<u8>().ok())
            .ok_or(())
    }
}

impl FromStr for Command {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split_whitespace();

        let cmd  = iter.next().ok_or(())?;

        match cmd {
            "add" => {
                Self::parse_next_u8(&mut iter).map(|val| Command::Add(val))
            },
            "remove" => {
                Self::parse_next_u8(&mut iter).map(|val| Command::Remove(val))
            },
            "check" => {Self::parse_next_u8(&mut iter).map(|val| Command::Check(val))
            },
            "toggle" => {Self::parse_next_u8(&mut iter).map(|val| Command::Toggle(val))
            },
            "all" => {Ok(Self::All)},
            "empty" => {Ok(Self::Empty)},
            _ => Err(())
        }
    }
}

fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut reader = BufReader::new(stdin.lock());
    let mut writer = BufWriter::with_capacity(4 * 1024, stdout.lock());
    
    let mut line = String::new();

    while let Ok(byte_counter) = reader.read_line(&mut line) {
        if byte_counter == 0 {break;}

        writer.write_all(line.as_bytes()).expect("Unable to write.");
        line.clear();
    }

    writer.flush().expect("Unable to write the leftover datas.");
}

