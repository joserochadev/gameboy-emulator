use std::env;
use std::fs::File;
use std::io::Read;

mod bus;
mod cpu;
mod win_sdl;

use bus::Bus;
use cpu::Cpu;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use win_sdl::WinSDL;

// const WINDOW_WIDTH: usize = 400;
// const WINDOW_HEIGHT: usize = 200;

fn main() {
  if env::args().len() < 2 {
    println!("Error: rom not found.");
    println!("Usage: cargo run <ROM_PATH>");
  }

  let rom_path = env::args().nth(1).unwrap();
  let mut rom = File::open(&rom_path).unwrap();
  let mut rom_buffer: Vec<u8> = Vec::new();
  rom.read_to_end(&mut rom_buffer).unwrap();

  let mut bus = Bus::new();
  let mut cpu = Cpu::new();
  cpu.bus_connect(&mut bus);

  bus.cpu_connect(&mut cpu);

  bus.memory[0..=255].copy_from_slice(&mut rom_buffer[0..=255]);

  println!("Rom Path: {:?}", rom_path);
  println!("Rom Buffer: {:?}", &bus.memory[0..=255]);

  let mut debugger = WinSDL::new("Debugger", 900, 650).unwrap();

  'running: loop {
    debugger.canvas.set_draw_color(Color::RGB(0, 0, 0));
    debugger.canvas.clear();

    debugger.draw_cpu_registers(&cpu, 10, 10);
    debugger.draw_memory_view(&bus.memory, 0x0000, 10, 300, 16, 15);
    debugger.draw_memory_view(&bus.memory, cpu.reg.pc, 10, 250, 0, 6);

    debugger.canvas.present();

    for event in debugger.event_pump.poll_iter() {
      match event {
        Event::Quit { .. } => break 'running,
        Event::KeyDown { keycode, .. } => match keycode {
          Some(Keycode::Space) => {
            if let Err(e) = cpu.step() {
              eprintln!("{}", e);
            }
          }

          _ => (),
        },
        _ => (),
      }
    }
  }
}
