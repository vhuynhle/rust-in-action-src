fn main() {
    let number: u32 = 0xAABBCCDD;
    let big_endian: [u8; 4] = [0xAA, 0xBB, 0xCC, 0xDD];
    let little_endian: [u8; 4] = [0xDD, 0xCC, 0xBB, 0xAA];

    let a: u32 = unsafe { std::mem::transmute(big_endian) };
    let b: u32 = unsafe { std::mem::transmute(little_endian) };

    if a == number {
        println!("Big Endian");
    }

    if b == number {
        println!("Little Endian");
    }
}
