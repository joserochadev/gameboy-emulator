#![allow(dead_code)]

use std::ptr::null_mut;

use crate::cpu::Cpu;

#[derive(Debug)]
pub struct Bus {
  pub memory: [u8; 0xffff],
  cpu: *mut Cpu,
}

impl Bus {
  pub fn new() -> Bus {
    Bus {
      memory: [0; 0xffff],
      cpu: null_mut(),
    }
  }

  pub fn cpu_connect(&mut self, cpu: &mut Cpu) {
    self.cpu = cpu;
  }

  pub fn read(&self, addr: u16) -> u8 {
    self.memory[addr as usize]
  }

  pub fn write(&mut self, addr: u16, data: u8) {
    self.memory[addr as usize] = data;
  }
}
