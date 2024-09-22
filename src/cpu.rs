pub enum Flags {
  Z,
  N,
  H,
  C,
}

#[derive(Default, Debug)]
#[allow(dead_code)]
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
}

#[derive(Debug)]
pub struct Cpu {
  reg: Register,
}

impl Cpu {
  pub fn new() -> Cpu {
    Cpu {
      reg: Register::new(),
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
}
