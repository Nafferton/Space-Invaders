use anyhow::Result;
use space_invaders::disassemble_op;
use std::fs;

fn main() -> Result<()> {
    let rom_contents = fs::read("roms/invaders.rom").expect("could not read file");
    let mut pc: usize = 0;
    let rom_buffer = &rom_contents[..];

    loop {
        if pc >= rom_buffer.len() {
            break;
        }
        pc += disassemble_op(rom_buffer, pc);
    }
    Ok(())
}
