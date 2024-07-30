use std::env::args;
use std::fs::File;
use std::io::Read;

use chip_8_core::{self, MEMORY_SIZE};

fn main() {
    let args: Vec<String> = args().collect();
    let mut emu = chip_8_core::Emu::new();

    let mut rom = File::open(&args[1]).expect("Não foi possível localizar ou abrir a ROM");
    let mut buffer = Vec::new();

    rom.read_to_end(&mut buffer).unwrap();
    emu.load(&buffer);

    while emu.p_counter < MEMORY_SIZE as u16 - 1 {
        emu.tick();
    }
}
