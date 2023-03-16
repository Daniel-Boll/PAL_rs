use once_cell::sync::Lazy;
use std::sync::Mutex;

#[derive(Default)]
pub struct Frame {
  pub data: bool,
}

pub struct PrimaryMemory {
  pub frames: Vec<Frame>,
}

impl PrimaryMemory {
  pub fn new(size: usize) -> Self {
    Self {
      frames: (0..size).map(|_| Frame::default()).collect(),
    }
  }

  pub fn get_frame(&self, index: usize) -> bool {
    self.frames[index].data
  }

  pub fn alloc_frame(&mut self) -> Option<usize> {
    self.frames.iter().position(|frame| !frame.data)
  }
}

pub static mut PRIMARY_MEMORY: Lazy<Mutex<PrimaryMemory>> =
  Lazy::new(|| Mutex::new(PrimaryMemory::new(4)));
