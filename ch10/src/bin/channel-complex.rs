use std::thread;

#[macro_use]
extern crate crossbeam;
use crossbeam::channel::unbounded;

#[derive(Debug)]
enum ConnectivityCheck {
    Ping,
    Pong,
    Pang,
}

use ConnectivityCheck::*;

fn main() {
    let n_messages = 3;
    let (requests_tx, requests_rx) = unbounded();
    let (responses_tx, responses_rx) = unbounded();

    thread::spawn(move || loop {
        match requests_rx.recv().unwrap() {
            Ping => responses_tx.send(Pong).unwrap(),
            Pong => eprintln!("Unexpected pong response"),
            Pang => {
                println!("Stop requested.");
                return;
            }
        }
    });

    for _ in 0..n_messages {
        requests_tx.send(Ping).unwrap();
    }
    requests_tx.send(Pang).unwrap(); // Request termination

    for _ in 0..n_messages {
        select! {
            recv(responses_rx) -> msg => println!("{:?}", msg),
        }
    }
}
