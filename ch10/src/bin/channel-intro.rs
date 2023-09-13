use std::thread;

#[macro_use]
extern crate crossbeam;

use crossbeam::channel::unbounded;

fn main() {
    let (tx, rx) = unbounded();

    thread::spawn(move || {
        tx.send(42).unwrap();
    });

    select! {
        recv(rx) -> msg => println!("{:?}", msg),
    }
}
