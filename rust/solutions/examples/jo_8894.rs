use std::io::{self, BufReader, BufWriter, BufRead, Write};

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut reader = BufReader::new(stdin.lock());
    let mut writer = BufWriter::new(stdout.lock());
    let mut line = String::new();

    let _ = reader.read_line(&mut line);
    let f = line.trim().parse::<f32>().expect("Invalid float");
    let _ = write!(writer, "{:.3}\n", f);
    line.clear();

    let _ = reader.read_line(&mut line);
    let _ = write!(writer, "{}", line.trim()); 

    line.clear();

    let _ = writer.flush();
}