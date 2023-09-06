use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() -> Result<(), String> {
    let filename = std::env::args().nth(1)
        .ok_or_else(|| "File name not provided".to_string())?;

    let f = File::open(filename).unwrap();
    let mut reader = BufReader::new(f);

    let mut line = String::new();
    loop {
        let len = reader.read_line(&mut line).unwrap();
        if len == 0 {
            break;
        }

        println!("{} ({} byte long)", line, len);
        line.truncate(0);
    }

    Ok(())
}
