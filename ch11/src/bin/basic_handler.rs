#![cfg(not(windows))]

use std::{thread::sleep, time::Duration};

use libc::{SIGTERM, SIGUSR1};

static mut SHUTDOWN: bool = false;

fn main() {
    register_signal_handler();

    let delay = Duration::from_secs(1);

    for i in 1_usize.. {
        println!("{}", i);
        unsafe {
            if SHUTDOWN {
                println!("*");
                return;
            }
        }

        sleep(delay);

        let signal = if i > 5 { SIGTERM } else { SIGUSR1 };

        unsafe {
            libc::raise(signal);
        }
    }
    unreachable!();
}

fn register_signal_handler() {
    unsafe {
        libc::signal(SIGTERM, handle_sigterm as usize);
        libc::signal(SIGUSR1, handle_sigusr1 as usize);
    }
}

fn handle_sigterm(_signal: i32) {
    println!("SIGTERM");
    unsafe {
        SHUTDOWN = true;
    }
}

fn handle_sigusr1(_signal: i32) {
    println!("SIGUSR1");
}
