use std::io::{self, BufReader, BufWriter, BufRead, Write, stdin, stdout};

fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut reader = BufReader::new(stdin.lock());
    let mut writer = BufWriter::with_capacity(64 * 1024, stdout.lock());
    let mut line = String::new();

    while let Ok(bytes_counter) = reader.read_line(&mut line) {
        if bytes_counter == 0 { break; }
        writer.write_all(line.as_bytes()).expect("Unable to write");
        line.clear();
    }
    
    writer.flush().expect("Unable to write the residual datas.");
}