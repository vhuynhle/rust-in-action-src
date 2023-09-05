use std::convert::TryInto;

fn main() {
    let a: i32 = 10;
    let b: u16 = 100;

    // The type of b_ is inferred from the type of a
    let b_ = b.try_into().unwrap();

    if a < b_ {
        println!("{} < {}", a, b);
    } else if a == b_ {
        println!("{} = {}", a, b);
    } else {
        println!("{} > {}", a, b);
    }
}
