use crate::{cli::translate::TranslateOptions, memory::primary::PRIMARY_MEMORY};
use std::str::FromStr;

use self::address::LogicalAddress;

pub mod address;

#[derive(Debug, Clone, Copy, Default)]
pub struct PageTableEntry {
  pub page_frame_index: usize,
  pub valid: bool,

  // optional
  pub dirty: bool,
  pub accessed: bool,
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
      ..self.entries[index]
    }
  }
}

#[derive(Debug, Clone)]
pub struct MMU {
  pub page_table: PageTable,
  pub page_size: usize,
}

impl MMU {
  pub fn new(page_size: usize) -> Self {
    Self {
      page_table: PageTable {
        entries: (0..page_size).map(|_| PageTableEntry::default()).collect(),
      },
      page_size,
    }
  }

  pub fn translate(&mut self, address: &str) -> usize {
    // TODO: deal with errors later
    let address = LogicalAddress::from_str(address).unwrap();

    let (page, offset) = address.split(self.page_size);

    match self.page_table.get_frame(page) {
      Some(frame) => {
        // Nem bati lá, lol
        println!(
          "Found address in page table {:06x}",
          LogicalAddress::join(&address, frame, offset, self.page_size)
        );
      }
      None => {
        unsafe {
          match PRIMARY_MEMORY.lock().unwrap().alloc_frame() {
            Some(frame) => {
              // 1. Insert page table
              // 2. Send frame to PAL
              self.page_table.set_frame(page, frame);
            }
            None => {
              // 1. PAL deallocate a MF and return the allocated frame index
              // 2. Update page table
            }
          }
        }
      }
    };

    1
  }
}

impl Default for MMU {
  fn default() -> Self {
    Self::new(4096)
  }
}

pub fn entrypoint(
  TranslateOptions {
    output: _,
    trace,
    page_table_size,
  }: &TranslateOptions,
) -> anyhow::Result<()> {
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_mmu() {
    let mmu = MMU::new(4096);

    assert_eq!(mmu.page_size, 4096);
    assert_eq!(mmu.page_table.entries.len(), 4096);
  }

  #[test]
  fn test_translate() {
    let mut mmu = MMU::new(4096);

    mmu.translate("345678");
    mmu.translate("345678");
  }
}
