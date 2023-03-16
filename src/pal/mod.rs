pub trait PALTable {
  fn find_frame_to_deallocate(&mut self) -> usize;
  fn update_access(&mut self, frame: usize);
  fn insert(&mut self, frame: usize) -> Option<usize>;
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct LRUPALTableEntry {
  pub frame: usize,
  pub last_access: usize,
}

pub struct LRUPALTable {
  pub entries: Vec<LRUPALTableEntry>,
}

impl PALTable for LRUPALTable {
  fn find_frame_to_deallocate(&mut self) -> usize {
    let mut min = (0, usize::MAX);
    for LRUPALTableEntry { frame, last_access } in self.entries.iter() {
      if last_access < &min.1 {
        min = (*frame, *last_access);
      }
    }

    let (frame, _) = min;
    self.entries.retain(|x| x.frame != frame);
    frame
  }

  fn update_access(&mut self, frame: usize) {
    if let Some(last_access) = self.entries.get_mut(frame) {
      last_access.last_access += 1;
    }
  }

  fn insert(&mut self, frame: usize) -> Option<usize> {
    match self.entries.get(frame) {
      Some(_) => {
        self.update_access(frame);
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
          last_access: 0,
        });

        frame_to_deallocate
      }
    }
  }
}

pub enum PALAlgorithm {
  LRU,
}

pub struct PAL {
  pub table: Box<dyn PALTable>,
}

impl PAL {
  pub fn new(algorithm: PALAlgorithm, frame_count: usize) -> Self {
    println!("Creating a hashmap with capacity: {}", frame_count);

    match algorithm {
      PALAlgorithm::LRU => Self {
        table: Box::new(LRUPALTable {
          entries: Vec::with_capacity(frame_count),
        }),
      },
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
