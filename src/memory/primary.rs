use std::sync::Mutex;

use singleton_manager::sm;

#[derive(Default)]
pub struct Frame {
  pub data: bool,
}

pub struct PrimaryMemory {
  pub frames: Vec<Frame>,
  pub guard: Mutex<()>, // resource aquisition is initialization (RAII)
}

impl PrimaryMemory {
  pub fn new(size: usize) -> Self {
    Self {
      frames: (0..size).map(|_| Frame::default()).collect(),
      guard: Mutex::new(()),
    }
  }

  pub fn get_frame(&self, index: usize) -> bool {
    self.frames[index].data
  }

  pub fn alloc_frame(&mut self) -> Option<usize> {
    let _guard = self.guard.lock().expect("Failed to get guard");

    for (i, frame) in self.frames.iter_mut().enumerate() {
      if !frame.data {
        frame.data = true;
        return Some(i);
      }
    }

    None
  }

  pub fn get() -> &'static mut Self {
    sm().get::<Self>("MEMORY").unwrap()
  }

  pub fn create(size: usize) {
    sm()
      .set("MEMORY", PrimaryMemory::new(size))
      .expect("Failed to create memory");
  }
}
