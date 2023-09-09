const BYTE_PER_LINE: usize = 16;
const INPUT: &[u8] = br#"
HELLO, WORLD
"#;

fn main() -> std::io::Result<()> {
    let buffer: Vec<u8> = INPUT.into();

    let mut position_in_input = 0;
    for line in buffer.chunks(BYTE_PER_LINE) {
        print!("[0x{:08x}] ", position_in_input);
        for byte in line {
            print!("{:02x} ", byte);
        }
        println!();
        position_in_input += BYTE_PER_LINE;
    }

    Ok(())
}
