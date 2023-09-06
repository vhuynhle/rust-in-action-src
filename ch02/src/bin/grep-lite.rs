use clap::{arg, command};
use regex::Regex;

fn main() {
    let matches = command!()
        .arg(arg!([pattern] "The pattern to search for").required(true))
        .get_matches();

    let pattern = matches.get_one::<String>("pattern").unwrap();
    let re = Regex::new(pattern).unwrap();

    let quote = "\
Every face, every shop, bedroom window, public-house, and
dark square is a picture feverishly turned--in search of what?
It is the same with books.
 What do we seek through millions of pages?";

    for (i, line) in quote.lines().enumerate() {
        if re.find(line).is_some() {
            println!("{}: {}", i, line);
        }
    }
}
