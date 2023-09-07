fn main() {
    let a: u16 = 50115;
    let b: i16 = -15421;

    println!("a: {:016b} {}", a, a);
    println!("b: {:016b} {}", b, b);

    let bit_pattern_a = format!("{:016b}", a);
    let bit_pattern_b = format!("{:016b}", b);

    if (a as i64) == (b as i64) {
        println!("a = b");
    } else {
        println!("a != b");
    }

    if bit_pattern_a == bit_pattern_b {
        println!("a and b have the same bit pattern: {}", bit_pattern_a);
    } else {
        println!(
            "a and b have different bit patterns: {} vs {}",
            bit_pattern_a, bit_pattern_b
        );
    }
}
