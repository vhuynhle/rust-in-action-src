fn main() {
    // Creating a pointer is safe
    let ptr = 42 as *const Vec<String>;

    // Dereferencing it is unsafe
    unsafe {
        let new_addr = ptr.offset(4);
        println!("{:p} -> {:p}", ptr, new_addr);
    }
}
