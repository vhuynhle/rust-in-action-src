use std::thread::{JoinHandle, spawn, yield_now};
use std::time;

fn main() {
    for n in 1..=1000 {
        let mut handles: Vec<JoinHandle<()>> = Vec::with_capacity(n);
        let start = time::Instant::now();
        for _ in 0..n {
            let handle = spawn(|| {
                let start = time::Instant::now();
                let pause = time::Duration::from_millis(20);
                while start.elapsed() < pause {
                    yield_now();
                }
            });
            handles.push(handle);
        }

        while let Some(handle) = handles.pop() {
            handle.join().unwrap();
        }


        let end = time::Instant::now();
        let duration = end.duration_since(start);
        println!("{},{:.2}", n, duration.as_secs_f64() * 1000.0);
    }
}