use clap::ValueEnum;
use core::fmt::Debug;
use std::str::FromStr;

use self::counter::CounterPALTable;

pub mod counter;

pub trait PALTable {
  fn find_frame_to_deallocate(&mut self) -> usize;
  fn update_access(&mut self, frame: usize);
  fn insert(&mut self, frame: usize) -> Option<usize>;
  fn clone_dyn(&self) -> Box<dyn PALTable>;
  fn print(&self);
}

impl Debug for dyn PALTable {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    write!(f, "PALTable <imagine>")
  }
}

#[derive(ValueEnum, Debug, Clone)]
pub enum PALAlgorithm {
  LRU,
  Counter,
}

impl FromStr for PALAlgorithm {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "lru" => Ok(PALAlgorithm::LRU),
      _ => Err(format!("Unknown algorithm: {}", s)),
    }
  }
}

#[derive(Debug)]
pub struct PAL {
  pub table: Box<dyn PALTable>,
}

impl Clone for PAL {
  fn clone(&self) -> Self {
    Self {
      table: self.table.clone_dyn(),
    }
  }
}

impl PAL {
  pub fn new(algorithm: PALAlgorithm, frame_count: usize) -> Self {
    match algorithm {
      PALAlgorithm::Counter => Self {
        table: Box::new(CounterPALTable {
          entries: Vec::with_capacity(frame_count),
        }),
      },
      PALAlgorithm::LRU => unimplemented!(),
    }
  }

  pub fn find_frame_to_deallocate(&mut self) -> usize {
    self.table.find_frame_to_deallocate()
  }

  pub fn update_access(&mut self, frame: usize) {
    self.table.update_access(frame)
  }

  pub fn insert(&mut self, frame: usize) -> Option<usize> {
    self.table.insert(frame)
  }

  pub fn print(&self) {
    self.table.print()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_lru() {
    let mut pal = PAL::new(PALAlgorithm::LRU, 4);

    [0, 0, 1, 2, 1, 3, 3, 5, 5].iter().for_each(|&x| {
      println!("[{x}] - {:?}", pal.insert(x));
    });
  }
}
