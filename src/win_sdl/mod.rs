#![allow(dead_code)]
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
  pub window: Window,
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

    let canvas = window.clone().into_canvas().build().unwrap();
    let event_pump: sdl2::EventPump = sdl.event_pump().unwrap();

    Ok(WinSDL {
      sdl,
      event_pump,
      ttf_context,
      canvas,
      window,
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
    let space = 20;

    self.draw_text(&format!("A: {:02X}", cpu.reg.a), x, y + space * 0);
    self.draw_text(&format!("B: {:02X}", cpu.reg.b), x, y + space * 1);
    self.draw_text(&format!("C: {:02X}", cpu.reg.c), x, y + space * 2);
    self.draw_text(&format!("D: {:02X}", cpu.reg.d), x, y + space * 3);
    self.draw_text(&format!("E: {:02X}", cpu.reg.e), x, y + space * 4);
    self.draw_text(&format!("F: {:02X}", cpu.reg.f), x, y + space * 5);
    self.draw_text(&format!("H: {:02X}", cpu.reg.h), x, y + space * 6);
    self.draw_text(&format!("L: {:02X}", cpu.reg.l), x, y + space * 7);
    self.draw_text(&format!("SP: {:04X}", cpu.reg.sp), x, y + space * 8);
    self.draw_text(&format!("PC: {:04X}", cpu.reg.pc), x, y + space * 9);

    self.draw_text(
      if cpu.get_flag(crate::cpu::Flags::Z) == 1 {
        "Z"
      } else {
        "-"
      },
      x + 100 + space * 0,
      y,
    );
    self.draw_text(
      if cpu.get_flag(crate::cpu::Flags::N) == 1 {
        "N"
      } else {
        "-"
      },
      x + 100 + space * 1,
      y,
    );
    self.draw_text(
      if cpu.get_flag(crate::cpu::Flags::H) == 1 {
        "H"
      } else {
        "-"
      },
      x + 100 + space * 2,
      y,
    );
    self.draw_text(
      if cpu.get_flag(crate::cpu::Flags::C) == 1 {
        "C"
      } else {
        "-"
      },
      x + 100 + space * 3,
      y,
    );
  }

  pub fn bytes_to_ascii(&self, bytes: &[u8]) -> String {
    // Filtra os bytes para garantir que estejam no intervalo de caracteres ASCII imprimíveis
    bytes[0..=255]
      .iter()
      .map(|&b| {
        if b.is_ascii_graphic() || b.is_ascii_whitespace() {
          b as char // Converte o byte para o caractere ASCII correspondente
        } else {
          '.' // Se não for um caractere imprimível, substitui por um ponto
        }
      })
      .collect::<String>()
  }

  pub fn draw_ascii_grid(&mut self, bytes: &[u8], cols: usize, start_x: i32, start_y: i32) {
    let ascii_string = self.bytes_to_ascii(bytes);

    // Coleta os caracteres ASCII em um vetor que vive enquanto necessário
    let chars: Vec<char> = ascii_string.chars().collect();

    let y = start_y;
    for (row_index, row) in chars.chunks(cols).enumerate() {
      let row_str: String = row.iter().collect();
      self.draw_text(&row_str, start_x, y + row_index as i32 * 18); // Atualiza a posição y com o índice da linha
    }
  }
}

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
