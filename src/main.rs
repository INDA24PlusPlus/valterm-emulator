use cpu::Cpu;
use memory::Memory;

pub mod cpu;
pub mod memory;
pub mod opcodes;

fn main() {
    let mut memory = Memory::new();
    memory.load(
        0x3000,
        vec![
            0xE002, 0xF022, 0xF025, 0x0041, 0x0042, 0x0043, 0x0044, 0x000A,
        ],
    );
    //memory.dump(0x3000, 0x3004);
    let mut cpu = Cpu::new(memory);
    cpu.run();
}
