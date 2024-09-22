use std::env;
use std::fs::File;
use std::io::Read;

mod bus;
mod cpu;

use bus::Bus;
use cpu::Cpu;

struct EmuContext {
  running: bool
}

fn main() {
  if env::args().len() < 2 {
    println!("Error: rom not found.");
    println!("Usage: cargo run <ROM_PATH>");
  }

  let rom_path = env::args().nth(1).unwrap();
  let mut rom = File::open(&rom_path).unwrap();
  let mut rom_buffer: Vec<u8> = Vec::new();
  rom.read_to_end(&mut rom_buffer).unwrap();

  let ctx = EmuContext{
    running: true,
  };

  let mut bus = Bus::new();
  let mut cpu = Cpu::new();
  cpu.bus_connect(&mut bus);

  bus.cpu_connect(&mut cpu);

  bus.memory[0..=255].copy_from_slice(&mut rom_buffer[0..=255]);

  println!("Rom Path: {:?}", rom_path);
  println!("Rom Buffer: {:?}", &bus.memory[0..=255]);


  while ctx.running {
    cpu.step();
  }
}
