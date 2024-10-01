use crate::cpu::Cpu;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::{file, fs};

#[derive(Serialize, Deserialize, Debug)]
struct MemoryValue {
  address: u16,
  value: u8,
}

pub struct TestSuite {
  cpu: *mut Cpu,
  memory: *mut [u8; 0xffff],
}

impl TestSuite {
  pub fn new(cpu: &mut Cpu, memory: &mut [u8; 0xffff]) -> TestSuite {
    TestSuite { cpu, memory }
  }

  pub fn set_cpu_state(&mut self, cpu_state: Value) {
    unsafe {
      (*self.cpu).reg.a = cpu_state["a"].as_u64().unwrap() as u8;
      (*self.cpu).reg.b = cpu_state["b"].as_u64().unwrap() as u8;
      (*self.cpu).reg.c = cpu_state["c"].as_u64().unwrap() as u8;
      (*self.cpu).reg.d = cpu_state["d"].as_u64().unwrap() as u8;
      (*self.cpu).reg.e = cpu_state["e"].as_u64().unwrap() as u8;
      (*self.cpu).reg.f = cpu_state["f"].as_u64().unwrap() as u8;
      (*self.cpu).reg.h = cpu_state["h"].as_u64().unwrap() as u8;
      (*self.cpu).reg.l = cpu_state["l"].as_u64().unwrap() as u8;
      (*self.cpu).reg.pc = cpu_state["pc"].as_u64().unwrap() as u16;
      (*self.cpu).reg.sp = cpu_state["sp"].as_u64().unwrap() as u16;

      (*self.cpu).debug();
    }
  }

  pub fn set_ram_state(&mut self, ram: Value) {
    println!("ram: {}", ram);

    //   if let Some(array) = ram.as_array() {
    //     for item in array.iter() {
    //         if let Some(addr) = item.get(0) {
    //             if let Some(data) = item.get(1) {
    //                 println!("addr: {:?}, data: {:?}", addr, data);
    //               self.memory.wrapping_add(addr.as_u64().unwrap() as usize) = data.as_u64().unwrap() as u8;
    //             }
    //         }
    //     }
    // }
  }

  pub fn run_test(&mut self, path_to_test: &str) -> Result<()> {
    let test_file = fs::read_to_string(path_to_test).expect("Error: Can't read test file.");
    let json_test: Value = serde_json::from_str(&test_file).unwrap();
    self.set_cpu_state(json_test[0]["initial"].clone());
    self.set_ram_state(json_test[0]["initial"]["ram"].clone());

    Ok(())
  }
}
