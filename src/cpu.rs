use crate::{memory::Memory, opcodes};

pub struct Cpu {
    pub registers: [u16; 8],
    pc: u16,
    psr: u16,
    pub memory: Memory,
    halted: bool,
}

impl Cpu {
    pub fn new(memory: Memory) -> Cpu {
        Cpu {
            registers: [0, 0, 0, 0, 0, 0, 0, 0],
            pc: 0x3000,
            psr: 0,
            memory,
            halted: false,
        }
    }

    pub fn step(&mut self) {
        let instr = self.memory.read(self.pc);
        self.pc += 1;
        let opcode = instr >> 12;
        let a = (instr >> 9) & 0x7;
        let b = (instr >> 6) & 0x7;

        match opcode {
            opcodes::ADD => {
                let imm_flag = (instr >> 5) & 1;
                let c = if imm_flag == 0 {
                    self.registers[(instr & 0b111) as usize]
                } else {
                    sext(instr & 0b11111, 5) // 5 bit signed immediate value
                };
                self.setcc(a, self.registers[b as usize].wrapping_add(c)); // Should wrap!
            }
            opcodes::AND => {
                let imm_flag = (instr >> 5) & 1;
                let c = if imm_flag == 0 {
                    self.registers[(instr & 0b111) as usize]
                } else {
                    sext(instr & 0b11111, 5) // 5 bit signed immediate value
                };
                self.setcc(a, self.registers[b as usize] & c);
            }
            opcodes::BR => {
                let flags = (instr >> 9) & 0b111;
                if (self.psr & 0b111) & flags != 0 {
                    self.pc = self.pc.wrapping_add(sext(instr & 0x1FF, 9));
                }
            }
            opcodes::JMP => {
                // RET is same as JMP R7
                self.pc = self.registers[b as usize];
            }
            opcodes::JSR => {
                // JSRR is a variation of JSR, same opcode
                self.registers[7] = self.pc; // Store PC in R7
                if (instr >> 11) & 1 == 1 {
                    self.pc += sext(instr & 0x7FF, 11);
                } else {
                    self.pc = self.registers[b as usize];
                }
            }
            opcodes::LD => self.setcc(
                a,
                self.memory
                    .read(self.pc.wrapping_add(sext(instr & 0x1FF, 9))),
            ),
            opcodes::LDI => {
                let addr = self
                    .memory
                    .read(self.pc.wrapping_add(sext(instr & 0x1FF, 9)));
                self.setcc(a, self.memory.read(addr));
            }
            opcodes::LDR => self.setcc(
                a,
                self.registers[b as usize].wrapping_add(sext(instr & 0x3F, 6)),
            ),
            opcodes::LEA => self.setcc(a, self.pc.wrapping_add(sext(instr & 0x1FF, 9))),
            opcodes::NOT => self.setcc(a, !self.registers[b as usize]),
            opcodes::RTI => {
                // RTI only valid for returns from supervisor mode, supervisor mode not supported here
                panic!("Unsupported instruction!");
            }
            opcodes::ST => {
                self.memory.write(
                    self.pc.wrapping_add(sext(instr & 0x1FF, 9)),
                    self.registers[a as usize],
                );
            }
            opcodes::STI => {
                let addr = self
                    .memory
                    .read(self.pc.wrapping_add(sext(instr & 0x1FF, 9)));
                self.memory.write(addr, self.registers[a as usize]);
            }
            opcodes::STR => {
                self.memory.write(
                    self.registers[b as usize].wrapping_add(sext(instr & 0x3F, 6)),
                    self.registers[a as usize],
                );
            }
            opcodes::TRAP => self.trap(instr & 0xFF),
            _ => {
                println!("Unknown opcode: {:04X}", opcode);
                self.halted = true;
            }
        }
    }

    pub fn trap(&mut self, vector: u16) {
        //println!("TRAP: x{:X}", vector);
        self.registers[7] = self.pc; // Store PC in R7 just to comply with specification, not used
        if vector == 0x20 {
            // GETC
            self.registers[0] = 0x0041; // A
        } else if vector == 0x21 {
            // OUT
            let chr = (self.registers[0] as u8) as char;
            print!("{}", chr);
        } else if vector == 0x22 {
            // PUTS
            let mut addr = self.registers[0];
            let mut c = self.memory.read(addr);
            while c != 0 {
                print!("{}", (c as u8) as char);
                addr += 1;
                c = self.memory.read(addr);
            }
        } else if vector == 0x25 {
            println!("Halted processor!");
            self.halted = true;
        }
    }

    fn setcc(&mut self, reg: u16, value: u16) {
        self.psr &= 0b1111_1111_1111_1000; // Clear the condition codes
        let z = (value == 0) as u16;
        let n = (value >> 15) & 0x1;
        let p = (!n & 1) ^ z;
        self.psr |= n << 2 | z << 1 | p;
        self.registers[reg as usize] = value;
    }

    pub fn run(&mut self) {
        while !self.halted {
            self.step();
        }
    }

    pub fn dump(&self) {
        println!("Registers:");
        for i in 0..8 {
            println!("R{}:\tx{:04X}", i, self.registers[i]);
        }
        println!("PC:\tx{:04X}", self.pc);
        println!(
            "PSR:\tx{:04X} (N={}, Z={}, P={})",
            self.psr,
            (self.psr >> 2) & 1,
            (self.psr >> 1) & 1,
            self.psr & 1
        );
    }
}

// Sign extend integer to 16 bits
pub fn sext(val: u16, width: u8) -> u16 {
    let sign = (val >> (width - 1)) & 1;
    let ext = !sign.wrapping_add_signed(-1) << (width - 1);
    ext | val
}
