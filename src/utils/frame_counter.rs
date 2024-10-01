// utils/frame_counter.rs
use std::time::Instant;

pub const FRAME_HISTORY_COUNT: usize = 100;

pub struct FrameCounter {
  frame_times: [f64; FRAME_HISTORY_COUNT],
  frame_index: usize,
  last_frame_time: Instant,
}

impl FrameCounter {
  pub fn new() -> Self {
    FrameCounter {
      frame_times: [0.0; FRAME_HISTORY_COUNT],
      frame_index: 0,
      last_frame_time: Instant::now(),
    }
  }

  pub fn update(&mut self) -> f64 {
    let now = Instant::now();
    let frame_duration = now.duration_since(self.last_frame_time).as_secs_f64() * 1000.0; // Tempo em ms
    self.last_frame_time = now;

    // Adicionar o tempo do frame atual ao histórico
    self.frame_times[self.frame_index] = frame_duration;
    self.frame_index = (self.frame_index + 1) % FRAME_HISTORY_COUNT;

    // Calcular a média dos últimos frames
    let avg_frame_time: f64 = self.frame_times.iter().sum::<f64>() / FRAME_HISTORY_COUNT as f64;

    avg_frame_time
  }
}
