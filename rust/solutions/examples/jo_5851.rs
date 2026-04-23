use std::io::{self, BufReader, BufWriter, BufRead, Write};

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut reader = BufReader::new(stdin.lock());
    let mut writer = BufWriter::new(stdout.lock());
    let mut line = String::new();

    let _ = reader.read_line(&mut line);
    // let n = line.trim().parse::<i32>().unwrap();
    line.clear();

    let _ = reader.read_line(&mut line);
    let k = line.trim().parse::<i32>().unwrap();
    line.clear();

    let mut buf = Vec::new();

    let _ = reader.read_until(b'\n', &mut buf);
    let mut i = 0;

    let mut red = k;
    // let mut white = n-k;

    while let Some(h) = buf.get(i) {
        i+=1;
        match h {
            b'R' => {red -= 1},
            b'W' => {
                continue;
                // white -= 1
            },
            _ => {break;}
        }
    }

    if red == 0 {
        let _ = write!(writer, "W");
    }
    else {
        let _ = write!(writer, "R");
    }
    let _ = writer.flush();
}