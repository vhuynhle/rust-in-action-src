use std::{
    fs::File,
    io::{BufReader, ErrorKind, Read},
};

const BYTE_PER_LINE: usize = 16;

fn main() -> std::io::Result<()> {
    let filename = std::env::args().nth(1).expect("Usage: fview FILENAME");
    let file = File::open(filename).expect("Cannot open file");
    let mut bufreader = BufReader::new(file);

    let mut buffer: [u8; BYTE_PER_LINE] = [0; BYTE_PER_LINE];

    let mut position_in_input = 0;

    loop {
        match bufreader.read(&mut buffer) {
            Ok(0) => {
                break;
            }
            Ok(n) => {
                print!("[0x{:08x}] ", position_in_input);
                for byte in &buffer[0..n] {
                    print!("{:02x} ", byte);
                }
                println!();
                position_in_input += n;
            }
            Err(e) => {
                if e.kind() == ErrorKind::Interrupted {
                    continue;
                }
                return Err(e);
            }
        }
    }

    Ok(())
}
