use chrono::{DateTime, Local};
use clap::{arg, Arg, command};

struct Clock;

impl Clock {
    fn get() -> DateTime<Local> {
        Local::now()
    }

    fn set() -> ! {
        unimplemented!()
    }
}

fn main() {
    let matches = command!()
        .about("Gets and sets the time")
        .version("0.1")
        .arg(Arg::new("action")
            .short('a')
            .value_parser(["get", "set"])
            .default_value("get")
        )
        .arg(Arg::new("std")
            .short('s')
            .long("standard")
            .value_parser(["rfc2822",
                "rfc3339",
                "timestamp"])
            .default_value("rfc3339"))
        .arg(Arg::new("datetime")
            .help("When <action> is 'set', apply <datetime>. Otherwise, ignore.")
            .required(false)).get_matches();

    let action = matches.get_one::<String>("action").unwrap();
    let std = matches.get_one::<String>("std").unwrap();

    if action == "get" {
        let now = Clock::get();
        if std == "timestamp" {
            println!("{}", now.timestamp())
        } else if std == "rfc2822" {
            println!("{}", now.to_rfc2822());
        } else {
            println!("{}", now.to_rfc3339());
        }
    } else {
        unimplemented!()
    }
}