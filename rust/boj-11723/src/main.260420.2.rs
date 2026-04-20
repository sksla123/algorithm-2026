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
    fn parse_next_u8(iter: &mut SplitWhitespace) -> Result<u8, String> {
        iter.next()
            .and_then(|value| value.parse::<u8>().ok())
            .ok_or("Unable to parse".to_string())
    }
}

impl FromStr for Command {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split_whitespace();

        let cmd  = iter.next().ok_or(format!("'{0}' is not allowed command", s))?;

        match cmd {
            "add" => {
                Self::parse_next_u8(&mut iter).map(|val| Command::Add(val - 1))
            },
            "remove" => {
                Self::parse_next_u8(&mut iter).map(|val| Command::Remove(val - 1))
            },
            "check" => {Self::parse_next_u8(&mut iter).map(|val| Command::Check(val - 1))
            },
            "toggle" => {Self::parse_next_u8(&mut iter).map(|val| Command::Toggle(val - 1))
            },
            "all" => {Ok(Self::All)},
            "empty" => {Ok(Self::Empty)},
            _ => Err(format!("'{0}' is not allowed command", s))
        }
    }
}

const BITS:[&[u8]; 2]  = [b"0\n", b"1\n"];

struct BitArray {
    array: i32
}

impl BitArray {
    fn new() -> Self {
        Self {
            array: 0
        }
    }

    fn add(&mut self, val:u8) -> Result<(), String> {
        if val >= 32 {
            return Err(format!("'{}' is out of range.", val));
        }
        self.array |= 1 << val;
        Ok(())
    }

    fn remove(&mut self, val:u8) -> Result<(), String> {
        if val >= 32 {
            return Err(format!("'{}' is out of range.", val));
        }
        self.array &= !(1 << val);
        Ok(())
    }

    fn check<W: Write>(&self, writer: &mut BufWriter<W> , val:u8) -> Result<(), String> {
        if val >= 32 {
            return Err(format!("'{}' is out of range.", val));
        }
        let bit =  (self.array >> val & 1) as usize;
        writer.write_all(BITS[bit]).expect("Unable to write");
        Ok(())
    }

    fn toggle(&mut self, val:u8) -> Result<(), String> {
        if val >= 32 {
            return Err(format!("'{}' is out of range.", val));
        }
        self.array ^= 1 << val;
        Ok(())
    }

    fn all(&mut self) {
        self.array = 0b1111_1111_1111_1111_1111;
    }

    fn clear(&mut self) {
        self.array = 0;
    }

    fn execute_command<W: Write>(&mut self, writer:&mut BufWriter<W>, cmd: Command) -> Result<(), String> {
        match cmd {
            Command::Add(v) => self.add(v)?,
            Command::Remove(v) => self.remove(v)?,
            Command::Check(v) => self.check(writer, v)?,
            Command::Toggle(v) => self.toggle(v)?,
            Command::All => self.all(),
            Command::Empty => self.clear()
        }

        Ok(())
    }
}

fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut reader = BufReader::new(stdin.lock());
    let mut writer = BufWriter::with_capacity(4 * 1024, stdout.lock());
    
    let mut line = String::new();
    let mut s = BitArray::new();

    reader.read_line(&mut line).expect("Unable to read");
    line.clear();

    while let Ok(byte_counter) = reader.read_line(&mut line) {
        if byte_counter == 0 {break;}

        let cmd = Command::from_str(line.trim()).expect("Unable to parse command");
        s.execute_command(&mut writer, cmd).expect("Failed to execute");

        line.clear();
    }

    writer.flush().expect("Unable to write the leftover datas.");
}

