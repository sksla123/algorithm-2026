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


use std::io::{BufRead, BufReader, BufWriter, Write, stdin, stdout};

fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut reader = BufReader::new(stdin.lock());
    let mut writer = BufWriter::with_capacity(16 * 1024, stdout);
    let mut line = String::new();

    let _ = reader.read_line(&mut line);
    line.clear();

    let mut v= Vec::new();
    let mut counter = 0;

    while let Ok(read_bytes) = reader.read_line(&mut line) {
        if read_bytes == 0 {break;}
        counter += 1;
        let mut iter = line.trim().split_whitespace();
        

        let sex = iter.next().unwrap().parse::<i32>().unwrap();
        if sex == 2 {
            v.push(counter);
            line.clear();
            continue;
        }

        let age = iter.next().unwrap().parse::<i32>().unwrap();
        if age <= 18 || age > 40  {
            if age <= 18 {
                let x = iter.next().unwrap().parse::<i32>().unwrap();
                if age < 17 {
                    v.push(counter);
                    line.clear();
                    continue;
                }
                if x == 1 {
                    line.clear();
                    continue;
                }
                v.push(counter);
            }
        }
        line.clear();
    }

    let l = v.len();
    write!(writer, "{}\n", l);
    for i in 0..l {
        write!(writer, "{} ", v[i]);
    }
    let _ = writer.flush();
}
