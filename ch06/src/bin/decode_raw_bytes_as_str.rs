use std::borrow::Cow;
use std::ffi::CStr;
use std::os::raw::c_char;

static B: [u8; 10] = [99, 97, 114, 114, 121, 116, 111, 119, 101, 108];
static C: [u8; 11] = [116, 104, 97, 110, 107, 115, 102, 105, 115, 104, 0];

fn main() {
    let a = 42;
    let b: String;
    let c: Cow<str>;

    unsafe {
        let b_ptr = &B as *const u8 as *mut u8;
        b = String::from_raw_parts(b_ptr, 10, 10);

        let c_ptr = &C as *const u8 as *const c_char;
        c = CStr::from_ptr(c_ptr).to_string_lossy();
    }

    println!("a: {}, b: {}, c: {}", a, b, c);

    // A normal String is allocated and stored on heap.
    // b is constructed from static memory, so do not (automatically) drop it here.
    // Otherwise, the program will segfault trying to free static memory at the end of
    // the current scope.
    let _ = std::mem::ManuallyDrop::new(b);

    // c is copy-on-write, and it is only borrowed in this case.
    // So dropping c is ok because it does not free any memory.
    match c {
        Cow::Borrowed(_) => println!("c is borrowed"),
        Cow::Owned(_) => println!("c is owned"),
    }
    drop(c);
}
