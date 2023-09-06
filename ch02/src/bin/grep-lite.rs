use clap::{arg, command};
use regex::Regex;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

fn process_lines<T: BufRead>(reader: T, re: Regex) {
    for (i, line) in reader.lines().enumerate() {
        let line: String = line.expect("File read error");

        if re.find(&line).is_some() {
            println!("{}: {}", i, line);
        }
    }
}

fn main() {
    let matches = command!()
        .arg(arg!([pattern] "The pattern to search for").required(true))
        .arg(arg!([file] "File to search").required(false))
        .get_matches();

    let pattern = matches.get_one::<String>("pattern").unwrap();
    let re = Regex::new(pattern).unwrap();

    let input_file = matches.get_one::<String>("file");

    if let Some(input_file) = input_file {
        let f = File::open(input_file).expect("Cannot open file.");
        let reader = BufReader::new(f);
        process_lines(reader, re);
    } else {
        let stdin = io::stdin();
        let reader = stdin.lock();
        process_lines(reader, re);
    }
}
