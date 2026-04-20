use std::io::{self, BufReader, BufWriter, BufRead, Write};

enum Command {
    Add(u8),
    Remove(u8),
    Check(u8),
    Toggle(u8),
    All,
    Empty,
    Invalid
}

fn parse_command(command: &str) -> Command {
    let mut iter = command.split_whitespace();
    
    let cmd = match iter.next() {
        Some(c) => c,
        None => return Command::Invalid,
    };

    match cmd {
        "add" => iter.next().and_then(|v| v.parse::<u8>().ok()).map_or(Command::Invalid, Command::Add),
        "remove" => iter.next().and_then(|v| v.parse::<u8>().ok()).map_or(Command::Invalid, Command::Remove),
        "check" => iter.next().and_then(|v| v.parse::<u8>().ok()).map_or(Command::Invalid, Command::Check),
        "toggle" => iter.next().and_then(|v| v.parse::<u8>().ok()).map_or(Command::Invalid, Command::Toggle),
        "all" => Command::All,
        "empty" => Command::Empty,
        _ => Command::Invalid,
    }
}

const BITS: [&[u8]; 2] = [b"0\n", b"1\n"];

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut reader = BufReader::new(stdin.lock());
    let mut writer = BufWriter::with_capacity(64 * 1024, stdout.lock());
    let mut line = String::new();

    let mut s: i32 = 0;

    let _ = reader.read_line(&mut line);
    line.clear();
    
    while let Ok(bytes_counter) = reader.read_line(&mut line) {
        if bytes_counter == 0 { break; }

        match parse_command(line.trim()) {
            Command::Add(v) => s |= 1 << v,
            Command::Remove(v) => s &= !(1 << v),
            Command::Check(v) => {
                let bit = ((s >> v) & 1) as usize;
                writer.write_all(BITS[bit]).expect("Unable to write.");
            },
            Command::Toggle(v) => s ^= 1 << v,
            Command::All => s = (1 << 21) - 2,
            Command::Empty => s = 0,
            Command::Invalid => {} 
        }
        line.clear();
    }
    
    writer.flush().expect("Unable to write the residual datas.");
}