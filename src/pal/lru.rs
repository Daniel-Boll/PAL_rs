use chrono::{DateTime, Utc};

use super::PALTable;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct LRUPALTableEntry {
  pub frame: usize,
  pub last_access: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct LRUPALTable {
  pub entries: Vec<LRUPALTableEntry>,
}

impl PALTable for LRUPALTable {
  fn find_frame_to_deallocate(&mut self) -> usize {
    let mut min = (0, i64::MAX);
    for LRUPALTableEntry { frame, last_access } in self.entries.iter() {
      let last_access = last_access.timestamp_millis();
      if last_access < min.1 {
        min = (*frame, last_access);
      }
    }

    let (frame, _) = min;
    self.entries.retain(|x| x.frame != frame);
    frame
  }

  fn update_access(&mut self, index: usize) {
    self.entries[index].last_access = Utc::now();
  }

  fn insert(&mut self, frame: usize) -> Option<usize> {
    match self.entries.iter().position(|x| x.frame == frame) {
      Some(index) => {
        self.update_access(index);
        None
      }
      None => {
        let frame_to_deallocate = if self.entries.len() >= self.entries.capacity() {
          Some(self.find_frame_to_deallocate())
        } else {
          None
        };

        self.entries.push(LRUPALTableEntry {
          frame,
          last_access: Utc::now(),
        });

        frame_to_deallocate
      }
    }
  }

  fn clone_dyn(&self) -> Box<dyn PALTable> {
    Box::new(self.clone())
  }

  fn print(&self) {
    println!("LRU PAL Table {{");
    for LRUPALTableEntry { frame, last_access } in self.entries.iter() {
      println!(
        "   [{frame}]: {}",
        last_access.format("%Y-%m-%d %H:%M:%S%.3f"),
        frame = frame
      );
    }
    println!("}}");
  }
}
