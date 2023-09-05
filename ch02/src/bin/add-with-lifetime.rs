#[allow(clippy::needless_lifetimes)]
fn add_with_lifetime<'a, 'b>(i: &'a i32, j: &'b i32) -> i32 {
    i + j
}

fn main() {
    let a = 100;
    let b = 23;

    println!("a + b = {}", add_with_lifetime(&a, &b));
}
