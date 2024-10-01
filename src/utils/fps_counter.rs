use std::time::{Duration, Instant};

pub struct FpsCounter {
  last_time: Instant,
  frames: u32,
  fps: u32,
}

impl FpsCounter {
  pub fn new() -> Self {
    FpsCounter {
      last_time: Instant::now(),
      frames: 0,
      fps: 0,
    }
  }

  pub fn get_fps(&mut self) -> u32 {
    self.frames += 1;
    let elapsed_time = self.last_time.elapsed();

    if elapsed_time >= Duration::from_secs(1) {
      self.fps = self.frames;
      self.frames = 0;
      self.last_time = Instant::now();
    }

    self.fps
  }
}
