use std::{fs::File, io::Read};

use cpu::Cpu;
use memory::Memory;

pub mod cpu;
pub mod memory;
pub mod opcodes;
pub mod terminal;

fn u8_to_u16(v: Vec<u8>) -> Vec<u16> {
    let mut result = Vec::new();
    for i in 0..v.len() / 2 {
        result.push((v[i * 2] as u16) << 8 | v[i * 2 + 1] as u16);
    }
    result
}

/// Return starting address and code
fn load_file(filename: &str) -> (u16, Vec<u16>) {
    let mut f = File::open(filename).unwrap();
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).unwrap();
    (
        ((buffer[0] as u16) << 8 | buffer[1] as u16),
        u8_to_u16(buffer[2..].to_vec()),
    )
}

fn main() {
    let mut memory = Memory::new();
    //memory.load(0x3000, vec![0xF023, 0xF025]);
    let (addr, buffer) = load_file("examples/2048.obj");
    memory.load(addr, buffer);
    let mut cpu = Cpu::new(memory);
    /* for _ in 0..100 {
        cpu.step();
        cpu.dump();
    } */
    cpu.run();
}
