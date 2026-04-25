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

        if age > 18 && age <= 40 {
            line.clear();
            continue;
        } 
        else if age > 16 && age <= 18 {
            let x = iter.next().unwrap().parse::<i32>().unwrap();
            if x == 1 {
                line.clear();
                continue;
            }
        }
        v.push(counter);
        line.clear();
    }

    let l = v.len();
    let _ = write!(writer, "{}\n", l);
    for i in 0..l {
        let _ = write!(writer, "{} ", v[i]);
    }
    let _ = writer.flush();
}
