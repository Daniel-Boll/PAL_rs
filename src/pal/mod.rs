use clap::ValueEnum;
use core::fmt::Debug;
use singleton_manager::sm;
use std::collections::VecDeque;
use std::{str::FromStr, sync::Mutex};

use self::counter::CounterPALTable;
use self::second_chance::SecondChancePALTable;

pub mod counter;
pub mod lru;
pub mod second_chance;

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

#[derive(ValueEnum, Debug, Clone, Hash, Eq, PartialEq, Copy)]
pub enum PALAlgorithm {
  LRU,
  Counter,
  SecondChance,
}

impl FromStr for PALAlgorithm {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "lru" => Ok(PALAlgorithm::LRU),
      "counter" => Ok(PALAlgorithm::Counter),
      "second_chance" => Ok(PALAlgorithm::SecondChance),
      _ => Err(format!("Unknown algorithm: {}", s)),
    }
  }
}

#[derive(Debug)]
pub struct PAL {
  pub table: Box<dyn PALTable>,
  pub guard: Mutex<()>,
}

impl Clone for PAL {
  fn clone(&self) -> Self {
    Self {
      table: self.table.clone_dyn(),
      guard: Mutex::new(()),
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
        guard: Mutex::new(()),
      },
      PALAlgorithm::LRU => Self {
        table: Box::new(lru::LRUPALTable {
          entries: Vec::with_capacity(frame_count),
        }),
        guard: Mutex::new(()),
      },
      PALAlgorithm::SecondChance => Self {
        table: Box::new(SecondChancePALTable {
          entries: VecDeque::with_capacity(frame_count),
        }),
        guard: Mutex::new(()),
      },
    }
  }

  pub fn find_frame_to_deallocate(&mut self) -> usize {
    let _guard = self.guard.lock().unwrap();
    self.table.find_frame_to_deallocate()
  }

  pub fn update_access(&mut self, frame: usize) {
    let _guard = self.guard.lock().unwrap();
    self.table.update_access(frame)
  }

  pub fn insert(&mut self, frame: usize) -> Option<usize> {
    let _guard = self.guard.lock().unwrap();
    self.table.insert(frame)
  }

  pub fn print(&self) {
    self.table.print()
  }

  pub fn get() -> &'static mut Self {
    sm().get::<Self>("PAL").unwrap()
  }

  pub fn create(algorithm: PALAlgorithm, frame_count: usize) {
    sm().set("PAL", Self::new(algorithm, frame_count)).unwrap();
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

  #[test]
  fn test_second_chance() {
    let mut pal = PAL::new(PALAlgorithm::SecondChance, 4);

    // all elements are setted as accessed
    [
      (0, None),
      (0, None),
      (1, None),
      (2, None),
      (2, None),
      (1, None),
      (3, None),
      (3, None),
      (5, Some(0)),
      (5, None),
    ]
    .iter()
    .for_each(|(x, expected)| {
      let insertion_result = pal.insert(*x);
      println!("[{x}] - {:?}", insertion_result);
      assert_eq!(insertion_result, *expected);
    });

    // 2 and 3 not accessed, but 2 is removed because of the queue
    println!("======================");
    pal = PAL::new(PALAlgorithm::SecondChance, 4);
    [
      (0, None),
      (0, None),
      (1, None),
      (2, None),
      (1, None),
      (3, None),
      (5, Some(2)),
      (5, None),
    ]
    .iter()
    .for_each(|(x, expected)| {
      let insertion_result = pal.insert(*x);
      println!("[{x}] - {:?}", insertion_result);
      assert_eq!(insertion_result, *expected);
    });

    // only 2 is not accessed
    println!("======================");
    let mut pal = PAL::new(PALAlgorithm::SecondChance, 4);
    [
      (0, None),
      (0, None),
      (1, None),
      (2, None),
      (1, None),
      (3, None),
      (3, None),
      (5, Some(2)),
      (5, None),
    ]
    .iter()
    .for_each(|(x, expected)| {
      let insertion_result = pal.insert(*x);
      println!("[{x}] - {:?}", insertion_result);
      assert_eq!(insertion_result, *expected);
    });

    // 1 and 2 not accessed, but 1 is removed because of the queue
    println!("======================");
    let mut pal = PAL::new(PALAlgorithm::SecondChance, 4);
    [
      (0, None),
      (0, None),
      (1, None),
      (2, None),
      (3, None),
      (3, None),
      (5, Some(1)),
      (5, None),
    ]
    .iter()
    .for_each(|(x, expected)| {
      let insertion_result = pal.insert(*x);
      println!("[{x}] - {:?}", insertion_result);
      assert_eq!(insertion_result, *expected);
    });
  }
}
