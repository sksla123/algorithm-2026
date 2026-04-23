use std::io::{BufRead, BufReader, BufWriter, Write, stdin, stdout};

fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut reader = BufReader::new(stdin.lock());
    let mut writer = BufWriter::with_capacity(16 * 1024, stdout);
    let mut line = String::new();

    let mut counter = 0;

    while let Ok(read_bytes) = reader.read_line(&mut line) {
        if read_bytes == 0 {break;}

        if (0..=1).contains(&counter) {
            let f = line.trim().parse::<f32>().expect("Invalid float");
            let _ = write!(writer, "{:.2}\n", f);
        }
        else {
            let _ = write!(writer, "{}", line);
        }
        counter+= 1;
        line.clear();
    }
    let _ = writer.flush();
}