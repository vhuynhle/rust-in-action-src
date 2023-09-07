const BIAS: i32 = 127;
const RADIX: f32 = 2.0;

fn examine_float(f: f32) {
    let f_bits = f.to_bits();
    println!("Bits: {:032b}", f_bits);

    let sign = f_bits >> 31;
    println!("Sign:\t{}", sign);
    let exponent_raw = (f_bits >> 23) & 0xFF;
    let exponent = (exponent_raw as i32) - BIAS;
    println!(
        "Exponent: raw {:08b}, interpreted: {}",
        exponent_raw, exponent
    );

    let mantissa_raw = f_bits & 0x7fffff; // 23 bits
    println!("Mantissa: raw {:023b}", mantissa_raw);

    let mut mantissa = 1.0_f32;
    for i in 0..23 {
        let mask = 1 << i;
        let bit = f_bits & mask;
        if bit != 0 {
            let i_ = i as f32;
            let weight = RADIX.powf(i_ - 23.0);
            mantissa += weight;
        }
    }
    println!("Mantissa value: {}", mantissa);

    if sign == 1 {
        println!(
            "The value: {} = -1 * {} * ({} ^ {})",
            f, mantissa, RADIX, exponent
        );
    } else {
        println!(
            "The value: {} = +1 * {} * ({} ^ {})",
            f, mantissa, RADIX, exponent
        );
    }
}

fn main() {
    let x: f32 = 42.42;
    examine_float(x);
}
