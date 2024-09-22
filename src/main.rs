use std::env;
use std::fs::File;
use std::io::Read;

mod cpu;
use cpu::{Cpu,Flags};



fn main() {
  if env::args().len() < 2 {
    println!("Error: rom not found.");
    println!("Usage: cargo run <ROM_PATH>");
  }

  print!("{:?}", env::args().len());
  let rom_path = env::args().nth(1).unwrap();
  let mut rom = File::open(&rom_path).unwrap();
  let mut rom_buffer: Vec<u8> = Vec::new();
  rom.read_to_end(&mut rom_buffer).unwrap();

  let mut cpu = Cpu::new();

  println!("Rom Path: {:?}", rom_path);
  println!("Rom Buffer: {:?}", rom_buffer);

  cpu.set_flag(Flags::Z, true);
  cpu.set_flag(Flags::C, false);

  cpu.debug();
}
