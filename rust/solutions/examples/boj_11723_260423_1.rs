use std::io::{self, BufRead, BufReader, BufWriter, Write};

fn get_value(buf:&Vec<u8>, start:usize) -> u8 {
    let mut i = start;
    let mut v:u8 = 0;
    let mut temp:u8 = buf[i];
    while temp != b'\n' {
        v *= 10;
        v += temp - b'0';
        i += 1;
        temp = buf[i];
    }
    v
}

const BITSOUT:[&[u8; 2]; 2] = [b"0\n", b"1\n"];

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut reader = BufReader::new(stdin.lock());
    let mut writer = BufWriter::with_capacity(16 * 1024, stdout.lock());

    let mut buf:Vec<u8> = Vec::new();
    let mut s:u32 = 0;

    reader.read_until(b'\n', &mut buf).expect("Failed to read.");
    buf.clear();

    while let Ok(read_bytes) = reader.read_until(b'\n', &mut buf) {
        if read_bytes == 0 {break;}
        match buf[0] {
            b'a' => {
                match buf[1] {
                    b'd' => {
                        let value = get_value(&buf, 4);
                        s |= 1 << (value - 1);
                    },
                    b'l' => {
                        s = 0b1111_1111_1111_1111_1111;
                    },
                    _ => {panic!("\'a\' branch Error: Invalid input.")}
                }
            },
            b'r' => {
                let value = get_value(&buf, 7);
                s &= !(1 << (value - 1));
            },
            b'c' => {
                let value = get_value(&buf, 6);
                writer.write_all(BITSOUT[(s >> (value - 1) & 1) as usize]).expect("Failed to write");
            },
            b't' => {
                let value = get_value(&buf, 7);
                s ^= 1 << (value - 1);
            },
            b'e' => {s = 0;},
            _ => {panic!("Error: Invalid input.")},   
        }
        buf.clear();
    }
    writer.flush().expect("Failed to flush");
}