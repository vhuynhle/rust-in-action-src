fn main() {
    let a: i64 = 42;
    let a_ptr = &a as *const i64;

    println!("a: {}, address: {:p}, a_ptr: {:?}", a, &a, a_ptr);
}
