use crate::{
  cli::translate::TranslateOptions,
  memory::primary::PRIMARY_MEMORY,
  pal::{PALAlgorithm, PAL},
};
use std::str::FromStr;

use self::{
  address::LogicalAddress,
  page_table::{PageTable, PageTableEntry},
};

pub mod address;
pub mod page_table;

#[derive(Debug, Clone)]
pub struct MMU {
  pub page_table: PageTable,
  pub page_size: usize,
  pub pal: PAL,
}

impl MMU {
  pub fn new(page_size: usize, pal_algorithm: PALAlgorithm) -> Self {
    Self {
      page_table: PageTable {
        entries: (0..(1 << (32 - page_size.ilog2())))
          .map(|_| PageTableEntry::default())
          .collect(),
      },
      page_size,
      // TODO: Get the number of free frames in the PAL from the CLI as well
      pal: PAL::new(pal_algorithm, 4096),
    }
  }

  pub fn translate(&mut self, address: &str) -> usize {
    // TODO: deal with errors later
    let cp = address.to_string();
    let address = LogicalAddress::from_str(address).unwrap();

    let (page, offset) = address.split(self.page_size);

    // println!("Translating address {:?} to page {page} {offset}", cp);

    match self.page_table.get_frame(page) {
      Some(frame) => {
        self.pal.insert(frame);

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
              self.page_table.set_frame(page, frame);

              // 2. Send frame to PAL
              self.pal.insert(frame);

              // println!("[MEM] Allocated frame {frame}");
            }
            None => {
              // 1. PAL deallocate a MF and return the allocated frame index
              let frame = self.pal.find_frame_to_deallocate();

              // 2. Invalidate the page table entry
              self.page_table.invalidate_frame(frame);

              // 3. Insert page table
              self.page_table.set_frame(page, frame);

              // 4. Send frame to PAL
              self.pal.insert(frame);

              // println!("[PAL] Allocated frame {frame}");
            }
          }
        }
      }
    };

    // self.pal.print();

    1
  }
}

impl Default for MMU {
  fn default() -> Self {
    Self::new(4096, PALAlgorithm::LRU)
  }
}

pub fn entrypoint(
  TranslateOptions {
    output: _,
    trace: __,
    page_table_size,
    algorithm,
  }: &TranslateOptions,
) -> anyhow::Result<()> {
  let mmu = MMU::new(*page_table_size, algorithm.clone());
  Ok(())
}

#[cfg(test)]
mod tests {
  use rand::Rng;

  use crate::pal::PALAlgorithm;

  use super::*;
  #[test]
  fn test_mmu() {
    let mmu = MMU::new(4096, PALAlgorithm::LRU);

    assert_eq!(mmu.page_size, 4096);
    assert_eq!(mmu.page_table.entries.len(), 4096u32.ilog2() as usize);
  }

  #[test]
  fn test_translate() {
    let mut mmu = MMU::new(4096, PALAlgorithm::LRU);

    mmu.translate("345678");
    mmu.translate("345678");
  }

  #[test]
  fn manually() {
    let mut mmu = MMU::new(4096, PALAlgorithm::LRU);

    ["001123", "002123", "001123", "003123", "005123", "006123"]
      .iter()
      .for_each(|address| {
        mmu.translate(address);
      });
  }

  #[test]
  fn complex() {
    let mut mmu = MMU::new(4096, PALAlgorithm::LRU);

    // Generate random addresses
    let addresses = (0..10000)
      .map(|_| {
        let mut rng = rand::thread_rng();
        let address: u32 = rng.gen();
        format!("{:06x}", address)
      })
      .collect::<Vec<String>>();

    addresses.iter().for_each(|address| {
      mmu.translate(address);
    });
  }
}
