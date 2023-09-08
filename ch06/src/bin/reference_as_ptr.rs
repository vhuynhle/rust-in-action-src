static B: [u64; 10] = [99, 97, 114, 114, 121, 116, 111, 119, 101, 108];
static C: [u8; 11] = [116, 104, 97, 110, 107, 115, 102, 105, 115, 104, 0];

fn main() {
    let a = 42;
    let b = &B;
    let c = &C;

    println!("a: {}, b: {:p}, c: {:p}", a, b, c);

    println!("Addresss and values of the elements of array B:");
    for (i, elem) in B.iter().enumerate() {
        println!("{}: @{:p}, value = {}", i, elem, elem);
    }
}
