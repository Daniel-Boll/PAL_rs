#[derive(Debug, Clone, Copy, Default)]
pub struct PageTableEntry {
  pub page_frame_index: usize,
  pub valid: bool,
}

#[derive(Debug, Clone)]
pub struct PageTable {
  pub entries: Vec<PageTableEntry>,
}

impl PageTable {
  pub fn get_frame(&self, index: usize) -> Option<usize> {
    let entry = self.entries[index];

    match entry.valid {
      true => Some(entry.page_frame_index),
      false => None,
    }
  }

  pub fn set_frame(&mut self, index: usize, frame: usize) {
    self.entries[index] = PageTableEntry {
      page_frame_index: frame,
      valid: true,
    }
  }

  pub fn invalidate(&mut self, index: usize) {
    self.entries[index] = PageTableEntry::default();
  }

  pub fn invalidate_frame(&mut self, frame: usize) {
    for entry in self.entries.iter_mut() {
      if entry.page_frame_index == frame {
        entry.valid = false;
      }
    }
  }

  pub fn print_valid_frames(&self) {
    self.entries.iter().enumerate().for_each(|(index, entry)| {
      if entry.valid {
        println!("Page {index} is valid", index = index);
      }
    });
  }
}

