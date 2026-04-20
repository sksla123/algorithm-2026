use std::io::{BufRead, BufReader, BufWriter, Write, stdin, stdout};

fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut reader = BufReader::new(stdin.lock());
    let mut writer = BufWriter::with_capacity(16 * 1024, stdout);
    let mut line = String::new();

    let mut counter = 0;
    while let Ok(bytes_counter) = reader.read_line(&mut line) {
        if bytes_counter == 0 {break;}
        counter += 1;
        
        if counter< 3 {
            let f = line.trim().parse::<f32>().expect("Unable to parse.");
            write!(writer, "{0:.2}\n", f).expect("Unable to write.");
        }
        else {
            write!(writer, "{}", line).expect("Unable to write.");
            break;
        }
        line.clear();
    }
    writer.flush().expect("Unable to flush");
}