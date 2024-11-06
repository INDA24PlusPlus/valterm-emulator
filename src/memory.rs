use crate::terminal;

#[derive(Debug, Clone)]
pub struct Memory {
    data: [u16; 0x10000],
}

impl Default for Memory {
    fn default() -> Self {
        Memory::new()
    }
}

impl Memory {
    pub fn new() -> Memory {
        Memory { data: [0; 0x10000] }
    }

    pub fn read(&self, address: u16) -> u16 {
        // Emulate KBSR and KBDR
        // This way of doing it is wrong, but it works sometimes
        // An asynchronous queue is probably needed to handle this properly
        if address == 0xFE00 {
            return 0x8000;
        }
        if address == 0xFE02 {
            return terminal::read_raw().expect("CTRL-C!");
        }
        self.data[address as usize]
    }

    pub fn write(&mut self, address: u16, value: u16) {
        self.data[address as usize] = value;
    }

    pub fn load(&mut self, start: u16, program: Vec<u16>) {
        for (i, instruction) in program.iter().enumerate() {
            self.data[start as usize + i] = *instruction;
        }
    }

    pub fn dump(&self, start: u16, end: u16) {
        println!("Memory dump from x{:04X} to x{:04X}", start, end);
        for i in start..end {
            println!("x{:04X}:\tx{:04X}", i, self.data[i as usize]);
        }
    }
}
