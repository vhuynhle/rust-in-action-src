use std::thread::{sleep, spawn};
use std::time;

fn main() {
    let pause = time::Duration::from_millis(20);

    // Duration implements the Copy trait,
    // so move actually creates a copy here.
    let handle1 = spawn(move || {
        sleep(pause);
    });

    let handle2 = spawn(move || {
        sleep(pause);
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}