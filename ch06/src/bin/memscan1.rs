static GLOBAL: i32 = 10000;

fn noop() -> *const i32 {
    let noop_local = 12345;
    &noop_local as *const i32
}

fn main() {
    let local_str = "a";
    let local_int = 123;
    let boxed_str = Box::new("b");
    let boxed_int = Box::new(456);
    let fn_int = noop();

    println!("GLOBAL:        {:p}", &GLOBAL);
    println!("local_str:     {:p}", &local_str);
    println!("local_int:     {:p}", &local_int);
    println!("boxed_str:     {:p}", boxed_str);
    println!("boxed_str:     {:p}", Box::into_raw(boxed_str));
    println!("boxed_int:     {:p}", boxed_int);
    println!("boxed_int:     {:p}", Box::into_raw(boxed_int));
    println!("fn_int:        {:p}", fn_int);
}
