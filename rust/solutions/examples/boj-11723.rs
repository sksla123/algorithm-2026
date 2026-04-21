// use std::io::{self, BufRead, BufReader, BufWriter, Write};
// use std::str::FromStr;

// enum Command {
//     Add(u8),
//     Remove(u8),
//     Check(u8),
//     Toggle(u8),
//     All,
//     Empty
// }

// impl FromStr for Command {
//     type Err = String;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         let mut iter = s.split_whitespace();

//         let cmd = iter.next()
//             .ok_or_else(|| format!("\"{0}\" is unknown command.", s))?;

//         fn parse2u8(x: &str)->Result<u, Self::Err> {
//             x.parse::<u8>().map_err(|| format!("\"{0}\" is not valid number.", x))
//         }

//         match cmd {
//             "add" => {
//                 iter.next()
//                     .ok_or_else(|| "Missing argument for 'add'".to_string())
//                     .and_then(|x| parse2u8(x))
//                     .map(|x| Self::Add(x))
//                 },
//             "remove" => {
//                 iter.next()
//                     .ok_or_else(|| "Missing argument for 'add'".to_string())
//                     .and_then(|x| parse2u8(x))
//                     .map(|x| Self::Remove(x))
//                 },
//             "check" => {
//                 iter.next()
//                     .ok_or_else(|| "Missing argument for 'add'".to_string())
//                     .and_then(|x| parse2u8(x))
//                     .map(|x| Self::Check(x))
//                 },
//             "toggle" => {
//                 iter.next()
//                     .ok_or_else(|| "Missing argument for 'add'".to_string())
//                     .and_then(|x| parse2u8(x))
//                     .map(|x| Self::Toggle(x))
//                 },
//             "all" => Ok(Command::All),
//             "empty" => Ok(Command::Empty),
//             _ => Err(format!("\"{0}\" is unknown command.", s));
//         }
//     }
// }

// fn main() {
//     let stdin = io::stdin();
//     let stdout = io::stdout();
//     let mut reader = BufReader::new(stdin.lock());
//     let mut writer = BufWriter::with_capacity(16*1024, stdout.lock());
//     let mut buffer = Vec::new();

//     reader.read_until(b'\n', &mut buffer);
//     buffer.clear();

//     while let Ok(read_bytes) = reader.read_until(b'\n', &mut buffer) {
//         if read_bytes == 0 {break;}

//         buffer.clear();
//     }

//     writer.flush().expect("Unable to flush the residuals");
// }

fn main() {
    println!("hello")
}