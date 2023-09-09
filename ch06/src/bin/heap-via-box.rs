use std::mem::drop;

fn main() {
    let a = Box::new(1);
    let b = Box::new(1);
    let c = Box::new(1);

    let result1 = *a + *b + *c;
    println!("On heap: Address of a: {:p}, b: {:p}, c: {:p}", a, b, c);
    println!("On stack: Address of result1: {:p}", &result1);

    drop(a);

    let d = Box::new(1);
    let result2 = *b + *c + *d;
    println!("On heap: Address of d: {:p}", d);
    println!("On stack: Address of result2: {:p}", &result2);

    println!("{} {}", result1, result2);
}
