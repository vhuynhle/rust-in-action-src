use libc::{raise, signal, SIGTERM, SIG_DFL, SIG_IGN};

fn main() {
    unsafe {
        signal(SIGTERM, SIG_IGN);
        raise(SIGTERM);
    }

    println!("Ok");

    unsafe {
        signal(SIGTERM, SIG_DFL);
        raise(SIGTERM);
    }
}
