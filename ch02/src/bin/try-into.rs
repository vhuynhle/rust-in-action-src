use std::convert::TryInto;

fn main() {
    let a: i32 = 10;
    let b: u16 = 100;

    // The type of b_ is inferred from the type of a
    let b_ = b.try_into().unwrap();

    match a.cmp(&b_) {
        std::cmp::Ordering::Less => println!("{} < {}", a, b),
        std::cmp::Ordering::Equal => println!("{} = {}", a, b),
        std::cmp::Ordering::Greater => println!("{} > {}", a, b),
    }
}
