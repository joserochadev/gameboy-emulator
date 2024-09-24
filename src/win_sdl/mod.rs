use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::ttf::{self, Sdl2TtfContext};
use sdl2::{video::Window, EventPump, Sdl};

use crate::cpu::Cpu;

pub struct WinSDL {
  pub sdl: Sdl,
  pub event_pump: EventPump,
  pub ttf_context: Sdl2TtfContext,
  pub canvas: Canvas<Window>,
}

impl WinSDL {
  pub fn new(title: &str, width: usize, height: usize) -> Result<Self, &'static str> {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let ttf_context = ttf::init().unwrap();

    let window = video_subsystem
      .window(title, width as u32, height as u32)
      .build()
      .unwrap();

    let canvas = window.into_canvas().build().unwrap();
    let event_pump: sdl2::EventPump = sdl.event_pump().unwrap();

    Ok(WinSDL {
      sdl,
      event_pump,
      ttf_context,
      canvas,
    })
  }

  pub fn draw_text(&mut self, text: &str, x: i32, y: i32) {
    let font = self
      .ttf_context
      .load_font("./fonts/dogica.ttf", 15)
      .expect("Failed to load font");

    let surface = font
      .render(text)
      .blended(Color::RGB(255, 255, 255))
      .expect("Failed to render text");

    let texture_creator = self.canvas.texture_creator();
    let texture = texture_creator
      .create_texture_from_surface(&surface)
      .expect("Failed to create texture");

    let target = sdl2::rect::Rect::new(x, y, surface.width(), surface.height());
    self.canvas.copy(&texture, None, Some(target)).unwrap();
  }

  pub fn draw_memory_view(
    &mut self,
    memory: &[u8],
    address: u16,
    x: i32,
    y: i32,
    n_row: i32,
    n_coll: i32,
  ) {
    let mut n_row_y = y;
    let mut addr = address;

    for _row in 0..=n_row {
      let mut offset = format!("{:04X}:", addr);

      for _col in 0..=n_coll {
        offset += &format!(" {:02X}", memory[addr as usize]);
        addr += 1;
      }

      self.draw_text(&offset, x, n_row_y);
      n_row_y += 20;
    }
  }

  pub fn draw_cpu_registers(&mut self, cpu: &Cpu, x: i32, y: i32) {
    let row_space = 20;

    self.draw_text(&format!("A: {:02X}", cpu.reg.a), x, y + row_space * 0);
    self.draw_text(&format!("B: {:02X}", cpu.reg.b), x, y + row_space * 1);
    self.draw_text(&format!("C: {:02X}", cpu.reg.c), x, y + row_space * 2);
    self.draw_text(&format!("D: {:02X}", cpu.reg.d), x, y + row_space * 3);
    self.draw_text(&format!("E: {:02X}", cpu.reg.e), x, y + row_space * 4);
    self.draw_text(&format!("F: {:02X}", cpu.reg.f), x, y + row_space * 5);
    self.draw_text(&format!("H: {:02X}", cpu.reg.h), x, y + row_space * 6);
    self.draw_text(&format!("L: {:02X}", cpu.reg.l), x, y + row_space * 7);
    self.draw_text(&format!("SP: {:04X}", cpu.reg.sp), x, y + row_space * 8);
    self.draw_text(&format!("PC: {:04X}", cpu.reg.pc), x, y + row_space * 9);
  }
}
