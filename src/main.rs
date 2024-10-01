use std::env;
use std::fs::File;
use std::io::Read;

mod bus;
mod cpu;
mod tests;
mod utils;
mod win_sdl;

use tests::TestSuite;
use utils::fps_counter::FpsCounter;
use utils::frame_counter::FrameCounter;

use bus::Bus;
use cpu::Cpu;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use win_sdl::WinSDL;

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

  let mut debugger = WinSDL::new("Debugger", 400, 400).unwrap();

  let mut fps_counter = FpsCounter::new();
  let mut frame_counter = FrameCounter::new();

  let mut step_error = 0;

  // let mut test = TestSuite::new(&mut cpu, &mut bus.memory);
  // test.run_test("./roms/json_tests/20.json");

  cpu.debug();
  cpu.view_memory_at(&bus.memory, cpu.reg.pc as usize, 8);

  'running: loop {
    // let fps = fps_counter.get_fps();
    // let avg_frame_time = frame_counter.update();

    // debugger.canvas.set_draw_color(Color::RGB(0, 0, 0));
    // debugger.canvas.clear();

    // debugger.draw_cpu_registers(&cpu, 10, 10);
    // debugger.draw_memory_view(&bus.memory, 0x0000, 10, 300, 16, 15);
    // debugger.draw_memory_view(&bus.memory, cpu.reg.pc, 10, 230, 0, 6);

    // // debugger.draw_ascii_grid(&bus.memory, 10, 850, 300);

    // debugger.draw_text(&format!("fps:{} | {:.1}(ms)", fps, avg_frame_time), 640, 10);
    // debugger.canvas.present();

    for event in debugger.event_pump.poll_iter() {
      match event {
        Event::Quit { .. } => break 'running,
        Event::KeyDown { keycode, .. } => match keycode {
          Some(Keycode::Space) => {
            let cpu_step = cpu.step();
            cpu.debug();
            cpu.view_memory_at(&bus.memory, cpu.reg.pc as usize, 8);

            if step_error == 0 {
              if let Err(e) = cpu_step {
              eprintln!("{}", e);
                step_error = -1
              }
            }
          }

          Some(Keycode::X) => {
            cpu.reg.set_hl(0x7fff);
            cpu.debug();
            cpu.view_memory_at(&bus.memory, cpu.reg.pc as usize, 8);
          }

          _ => (),
        },
        _ => (),
      }
    }

    if step_error == 0 {
      // cpu.debug();
      // cpu.view_memory_at(&bus.memory, cpu.reg.pc as usize, 8);

      if let Err(e) = cpu.step(){
        eprintln!("{}", e);
        step_error = -1;
      }
    }
  }
}
