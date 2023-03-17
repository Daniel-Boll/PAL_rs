use super::PALTable;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct CounterPALTableEntry {
  pub frame: usize,
  pub last_access: usize,
}

#[derive(Debug, Clone)]
pub struct CounterPALTable {
  pub entries: Vec<CounterPALTableEntry>,
}

impl PALTable for CounterPALTable {
  fn find_frame_to_deallocate(&mut self) -> usize {
    let mut min = (0, usize::MAX);
    for CounterPALTableEntry { frame, last_access } in self.entries.iter() {
      if last_access < &min.1 {
        min = (*frame, *last_access);
      }
    }

    let (frame, _) = min;
    self.entries.retain(|x| x.frame != frame);
    frame
  }

  fn update_access(&mut self, index: usize) {
    self.entries[index].last_access += 1;
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

        self.entries.push(CounterPALTableEntry {
          frame,
          last_access: 0,
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
    for CounterPALTableEntry { frame, last_access } in self.entries.iter() {
      println!("   [{frame}]: {last_access}");
    }
    println!("}}");
  }
}
