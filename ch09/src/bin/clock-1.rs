use std::mem::zeroed;
use chrono::{DateTime, Local, TimeZone};
use clap::{Arg, command};

struct Clock;

impl Clock {
    fn get() -> DateTime<Local> {
        Local::now()
    }

    #[cfg(not(windows))]
    fn set<Tz: TimeZone>(t: DateTime<Tz>) {
        use libc::{timeval, time_t, suseconds_t};
        use libc::{settimeofday, timezone};

        let t = t.with_timezone(&Local);

        let mut u: timeval = unsafe { zeroed() }; // memset(&u, 0, sizeof(u));
        u.tv_sec = t.timestamp() as time_t;
        u.tv_usec = t.timestamp_subsec_micros() as suseconds_t;

        unsafe {
            let mock_tz: *const timezone = std::ptr::null();
            settimeofday(&u as *const timeval, mock_tz);
        }
    }
}

fn main() {
    let command = command!()
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
            .required(false));
    let matches = command.clone().get_matches();

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
        let t_ = matches.get_one::<String>("datetime");
        if t_.is_none() {
            let _ = command.clone().print_help();
            std::process::exit(1);
        }
        let t_: &String = t_.unwrap();
        let parser = match std.as_str() {
            "rfc2822" => DateTime::parse_from_rfc2822,
            "rfc3339" => DateTime::parse_from_rfc3339,
            _ => unimplemented!()
        };
        let err_msg = format!("Unable to parse {} according to {}", t_, std);
        let t = parser(t_).expect(&err_msg);
        Clock::set(t);

        let maybe_error = std::io::Error::last_os_error();
        let os_error_code = &maybe_error.raw_os_error();
        match os_error_code {
            Some(error_code) if *error_code != 0 =>
                { eprintln!("Unable to set the time: {:}", maybe_error); },
            _ => (),
        }
    }
}