use std::thread::JoinHandle;
use std::time;

fn main() {
    let start = time::Instant::now();
    let num_threads = 10;
    let mut handles: Vec<JoinHandle<()>> = Vec::with_capacity(num_threads);

    for _ in 0..num_threads {
        handles.push(std::thread::spawn(|| {
            let pause = time::Duration::from_millis(300);
            std::thread::sleep(pause);
        }))
    };

    for handle in handles {
        handle.join().unwrap();
    }

    let end = time::Instant::now();
    let duration = end.duration_since(start);
    println!("{:02}ms", duration.as_secs_f64() * 1000.0);
}