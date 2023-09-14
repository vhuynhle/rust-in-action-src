use core::time;
use std::{process, thread::sleep};

fn main() {
    let delay = time::Duration::from_secs(1);
    let pid = process::id();
    println!("{}", pid);

    for i in 0..60 {
        sleep(delay);
        println!(". {}", i + 1);
    }
}
