use std::collections::VecDeque;

use super::PALTable;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SecondChancePALTableEntry {
  pub frame: usize,
  pub accessed: bool,
}

#[derive(Debug, Clone)]
pub struct SecondChancePALTable {
  pub entries: VecDeque<SecondChancePALTableEntry>,
}

impl PALTable for SecondChancePALTable {
  fn find_frame_to_deallocate(&mut self) -> usize {
    // iterate circularly through the entries
    let mut index = 0;
    loop {
      let entry = &mut self.entries[index];
      if entry.accessed {
        entry.accessed = false;
        index = (index + 1) % self.entries.len();
      } else {
        let removed_frame = self.entries.remove(index).unwrap().frame;
        return removed_frame;
      }
    }
  }

  fn update_access(&mut self, index: usize) {
    self.entries[index].accessed = true;
  }

  fn insert(&mut self, frame: usize) -> Option<usize> {
    match self.entries.iter().position(|x| x.frame == frame) {
      Some(index) => {
        self.update_access(index);
        None
      }
      None => {
        let frame_to_deallocate = if self.entries.len() >= self.entries.capacity() {
          let deallocated_frame = self.find_frame_to_deallocate();
          Some(deallocated_frame)
        } else {
          None
        };

        self.entries.push_back(SecondChancePALTableEntry {
          frame,
          accessed: false,
        });

        frame_to_deallocate
      }
    }
  }

  fn clone_dyn(&self) -> Box<dyn PALTable> {
    Box::new(self.clone())
  }

  fn print(&self) {
    println!("SecondChance PAL Table {{");
    for SecondChancePALTableEntry { frame, accessed } in self.entries.iter() {
      println!("   [{frame}]: {accessed}");
    }
    println!("}}");
  }
}
