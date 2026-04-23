use std::io::{self, BufRead, BufReader, BufWriter, Write};

fn get_value(buf: &[u8], start: usize) -> u8 {
    let mut v = 0;
    for &b in &buf[start..] {
        if b < b'0' || b > b'9' { break; }
        v = v * 10 + (b - b'0');
    }
    v
}

const BITSOUT: [&[u8]; 2] = [b"0\n", b"1\n"];

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut reader = BufReader::new(stdin.lock());
    let mut writer = BufWriter::with_capacity(16 * 1024, stdout.lock());

    let mut buf = Vec::with_capacity(32);
    let mut s: u32 = 0;

    let _ = reader.read_until(b'\n', &mut buf);
    buf.clear();

    while let Ok(n) = reader.read_until(b'\n', &mut buf) {
        if n == 0 { break; }
        
        match buf[0] {
            b'a' => {
                if buf[1] == b'd' {
                    let value = get_value(&buf, 4);
                    s |= 1 << (value - 1);
                } else {
                    s = 0b1111_1111_1111_1111_1111;
                }
            },
            b'r' => {
                let value = get_value(&buf, 7);
                s &= !(1 << (value - 1));
            },
            b'c' => {
                let value = get_value(&buf, 6);
                let _ = writer.write_all(BITSOUT[((s >> (value - 1)) & 1) as usize]);
            },
            b't' => {
                let value = get_value(&buf, 7);
                s ^= 1 << (value - 1);
            },
            b'e' => {
                s = 0;
            },
            _ => {}
        }
        buf.clear();
    }
    let _ = writer.flush();
}