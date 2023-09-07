/// Simplified CHIP-8 CPU
struct Cpu {
    registers: [u8; 16],    // 16 registers
    program_counter: usize, // program counter
    memory: [u8; 0x1000],   // 4K RAM
    stack: [u16; 16],
    stack_pointer: usize,
}

impl Cpu {
    fn read_opcode(&self) -> u16 {
        let p = self.program_counter;
        let op_byte1 = self.memory[p] as u16;
        let op_byte2 = self.memory[p + 1] as u16;
        (op_byte1 << 8) | op_byte2
    }

    fn run(&mut self) {
        loop {
            let opcode = self.read_opcode();
            // Set the next instruction
            self.program_counter += 2;

            let c = ((opcode & 0xF000) >> 12) as u8;
            let x = ((opcode & 0x0F00) >> 8) as u8;
            let y = ((opcode & 0x00F0) >> 4) as u8;
            let d = (opcode & 0x000F) as u8;
            let nnn = opcode & 0xFFF;
            match (c, x, y, d) {
                (0, 0, 0, 0) => {
                    return; // Termination
                }
                (0, 0, 0xE, 0xE) => self.ret(),
                (0x2, _, _, _) => self.call(nnn),
                (0x8, _, _, 0x4) => self.add_xy(x, y),
                _ => todo!("opcode {:04x}", opcode),
            }
        }
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];
        let (val, overflow) = arg1.overflowing_add(arg2);
        self.registers[x as usize] = val;
        if overflow {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
    }

    fn call(&mut self, addr: u16) {
        let sp = self.stack_pointer;
        let stack = &mut self.stack;

        if sp > stack.len() {
            panic!("Stack overflow!");
        }

        // Save the program counter. As in run(), it is current instruction + 2
        stack[sp] = self.program_counter as u16;
        self.stack_pointer += 1; // The slot in the stack is taken by program counter

        // Go to the specified address
        self.program_counter = addr as usize;
    }

    fn ret(&mut self) {
        if self.stack_pointer == 0 {
            panic!("Stack underflow");
        }

        // Find the position of the caller and unwind the stack
        self.stack_pointer -= 1;
        let call_addr = self.stack[self.stack_pointer];

        // Jump to it
        self.program_counter = call_addr as usize;
    }
}

fn main() {
    /////////////////////////////////////////////
    // Initialize the machine
    /////////////////////////////////////////////
    let mut cpu = Cpu {
        registers: [0; 16],
        memory: [0; 4096],
        program_counter: 0,
        stack: [0; 16],
        stack_pointer: 0,
    };

    cpu.registers[0] = 5;
    cpu.registers[1] = 10;

    let mem = &mut cpu.memory;

    /////////////////////////////////////////////
    // Program the machine
    /////////////////////////////////////////////

    // Define a function
    let add_twice: [u8; 6] = [
        0x80, 0x14, // add reg[0] reg[1]
        0x80, 0x14, // add reg[0] reg[1]
        0x00, 0xEE, // return
    ];

    // Load the function into memory, at ADDR 0x100
    mem[0x100..0x106].copy_from_slice(&add_twice);

    // Main program
    // Call the function, opcode = 2, addr = 100
    mem[0x000] = 0x21;
    mem[0x001] = 0x00;

    // Call the function again
    mem[0x002] = 0x21;
    mem[0x003] = 0x00;

    // HALT
    mem[0x004] = 0x00;
    mem[0x005] = 0x00;

    println!("{:?}", &mem[0x100..0x106]);

    cpu.run();

    assert_eq!(cpu.registers[0], 45);
    println!("5 + (10 * 2) + (10 * 2) = {}", cpu.registers[0]);
}
