#![allow(dead_code)]
use std::ptr::null_mut;

use crate::bus::Bus;


pub enum Flags {
  Z,
  N,
  H,
  C,
}

#[derive(Default, Debug)]
struct Register {
  a: u8,
  b: u8,
  c: u8,
  d: u8,
  e: u8,
  f: u8,
  h: u8,
  l: u8,
  pc: u16,
  sp: u16,
}

impl Register {
  pub fn new() -> Register {
    Register::default()
  }

  pub fn get_bc(&self) -> u16 {
    let register = ((self.b as u16) << 8) | self.c as u16;
    register
  }

  pub fn set_bc(&mut self, data: u16) {
    let hi = (data >> 8) & 0xFF;
    let lo = data & 0xFF;
    self.b = hi as u8;
    self.c = lo as u8;
  }
  pub fn get_de(&self) -> u16 {
    let register = ((self.d as u16) << 8) | self.e as u16;
    register
  }

  pub fn set_de(&mut self, data: u16) {
    let hi = (data >> 8) & 0xFF;
    let lo = data & 0xFF;
    self.d = hi as u8;
    self.e = lo as u8;
  }
  pub fn get_hl(&self) -> u16 {
    let register = ((self.h as u16) << 8) | self.l as u16;
    register
  }

  pub fn set_hl(&mut self, data: u16) {
    let hi = (data >> 8) & 0xFF;
    let lo = data & 0xFF;
    self.h = hi as u8;
    self.l = lo as u8;
  }
}

#[derive(Debug)]
pub struct Cpu {
  reg: Register,
  bus: *mut Bus
}

impl Cpu {
  pub fn new() -> Cpu {
    Cpu {
      reg: Register::new(),
      bus: null_mut()
    }
  }

  pub fn bus_connect(&mut self, bus: *mut Bus){
    self.bus = bus
  }

  pub fn read(&mut self, addr: u16) -> u8 {
    unsafe {
      (*self.bus).read(addr)
    }
  }

  pub fn write(&mut self, addr: u16, data: u8) {
    unsafe {
      (*self.bus).write(addr, data);
    }
  }

  pub fn debug(&self) {
    println!("Register A: 0x{:02X}", self.reg.a);
    println!("Register B: 0x{:02X}", self.reg.b);
    println!("Register C: 0x{:02X}", self.reg.c);
    println!("Register D: 0x{:02X}", self.reg.d);
    println!("Register E: 0x{:02X}", self.reg.e);
    println!("Register F: 0x{:02X}", self.reg.f);
    println!("Register H: 0x{:02X}", self.reg.h);
    println!("Register L: 0x{:02X}", self.reg.l);
    println!("Program Counter (PC): 0x{:04X}", self.reg.pc);
    println!("Stack Pointer (SP): 0x{:04X}", self.reg.sp);
    println!("Flag Z: {}", self.get_flag(Flags::Z));
    println!("Flag N: {}", self.get_flag(Flags::N));
    println!("Flag H: {}", self.get_flag(Flags::H));
    println!("Flag C: {}", self.get_flag(Flags::C));
    println!("{}","=".repeat(40));
  }

  pub fn get_flag(&self, flag: Flags) -> u8 {
    match flag {
      Flags::Z => (self.reg.f >> 7) & 0b1,
      Flags::N => (self.reg.f >> 6) & 0b1,
      Flags::H => (self.reg.f >> 5) & 0b1,
      Flags::C => (self.reg.f >> 4) & 0b1,
    }
  }

  pub fn set_flag(&mut self, flag: Flags, conditional: bool) {
    if conditional {
      self.reg.f = match flag {
        Flags::Z => self.reg.f | (1 << 7),
        Flags::N => self.reg.f | (1 << 6),
        Flags::H => self.reg.f | (1 << 5),
        Flags::C => self.reg.f | (1 << 4),
      };
    }
  }

  pub fn fetch(&mut self, addr: u16) -> u8 {
    let data = self.read(addr);
    self.reg.pc += 1;
    data

  }

  pub fn fetch16(&mut self, addr: u16) -> u16 {
    let lo = self.fetch(addr);
    let hi = self.fetch(addr+1);
    let data = ((hi as u16) << 8) | lo as u16;
    data
  }

  pub fn decode(&mut self, instruction: u8) {
    match instruction {
      0x31 => {
        let data = self.fetch16(self.reg.pc);
        self.reg.sp = data;
      },
      0xAF => {
        let result = self.reg.a ^ self.reg.a;
        self.reg.a = result;

        self.set_flag(Flags::Z, result == 0);
      },
      0x21 => {
        let data = self.fetch16(self.reg.pc);
        self.reg.set_hl(data);

      },
      0x32 => {
        let addr = self.reg.get_hl();
        let data = self.reg.a;
        self.write(addr, data);

        self.reg.set_hl(addr - 1);
      },
      _ => panic!("Unknow instruction. OPCODE: {:02X}", instruction)
    }
  }

  pub fn step(&mut self) {
    self.debug();

    let instruction = self.fetch(self.reg.pc);
    self.decode(instruction);

  }


}
