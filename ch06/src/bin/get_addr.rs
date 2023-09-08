fn main() {
    let a: i64 = 42;
    let a_ptr = &a as *const i64;

    #[allow(clippy::transmutes_expressible_as_ptr_casts)]
    let a_addr: usize = unsafe { std::mem::transmute(a_ptr) };
    println!("a: {}, ({:p}..0x{:x})", a, a_ptr, a_addr + 7);
}
