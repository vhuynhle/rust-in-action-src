use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let filename = std::env::args()
        .skip(1)
        .next()
        .expect("File name not provided");
    let f = File::open(filename).expect("Cannot open file");
    let reader = BufReader::new(f);

    for line in reader.lines() {
        let line = line.expect("Read error");
        println!("{} ({} bytes long)", line, line.len());
    }
}
