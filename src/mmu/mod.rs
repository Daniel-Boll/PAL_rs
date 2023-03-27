use crate::{cli::translate::TranslateOptions, memory::primary::PrimaryMemory, pal::PAL};
use std::str::FromStr;

use self::{
  address::LogicalAddress,
  page_table::{PageTable, PageTableEntry},
  trace::Trace,
};

pub mod address;
pub mod page_table;
pub mod trace;

#[derive(Debug, Clone)]
pub enum TranslationResult {
  Fault,
  Hit,
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
        entries: (0..(1 << (32 - page_size.ilog2())))
          .map(|_| PageTableEntry::default())
          .collect(),
      },
      page_size,
    }
  }

  pub fn translate(&mut self, address: &LogicalAddress) -> TranslationResult {
    let (page, offset) = address.split(self.page_size);
    // println!("Page: {}, Offset: {}", page, offset);

    let res = match self.page_table.get_frame(page) {
      Some(frame) => {
        PAL::get().insert(frame);

        // println!(
        //   "Found address in page table {:06x}",
        //   LogicalAddress::join(&address, frame, offset, self.page_size)
        // );
        TranslationResult::Hit
      }
      None => {
        match PrimaryMemory::get().alloc_frame() {
          Some(frame) => {
            // 1. Insert page table
            self.page_table.set_frame(page, frame);

            // 2. Send frame to PAL
            PAL::get().insert(frame);

            // println!("[MEM] Allocated frame {frame}");
            TranslationResult::Fault
          }
          None => {
            // 1. PAL deallocate a MF and return the allocated frame index
            let frame = PAL::get().find_frame_to_deallocate();
            // println!("[PAL] Deallocated frame {frame}");

            // 2. Invalidate the page table entry
            self.page_table.invalidate_frame(frame);

            // 3. Insert page table
            self.page_table.set_frame(page, frame);

            // 4. Send frame to PAL
            PAL::get().insert(frame);
            // println!("[PAL] Allocated frame {frame}");
            TranslationResult::Fault
          }
        }
      }
    };
    // PAL::get().print();
    res
  }

  pub fn translate_str(&mut self, address: &str) -> anyhow::Result<()> {
    self.translate(&LogicalAddress::from_str(&address.to_string())?);

    Ok(())
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
    trace: trace_file,
    page_table_size,
    algorithm,
    pal_table_entries,
  }: &TranslateOptions,
) -> anyhow::Result<Vec<TranslationResult>> {
  PrimaryMemory::create(*pal_table_entries);
  PAL::create(algorithm.clone(), *pal_table_entries);
  let mut mmu = MMU::new(*page_table_size);
  let trace = Trace::from_file(trace_file)?;

  Ok(
    trace
      .into_iter()
      .map(|address| mmu.translate(&address))
      .collect(),
  )
}

#[cfg(test)]
mod tests {
  use rand::Rng;

  use crate::pal::PALAlgorithm;

  use super::*;
  #[test]
  fn test_mmu() {
    let mmu = MMU::new(4096);
    PrimaryMemory::create(4096);
    PAL::create(PALAlgorithm::LRU, 4096);

    assert_eq!(mmu.page_size, 4096);
    assert_eq!(mmu.page_table.entries.len(), 4096u32.ilog2() as usize);
  }

  #[test]
  fn test_translate() {
    let mut mmu = MMU::new(4096);
    PrimaryMemory::create(4096);
    PAL::create(PALAlgorithm::LRU, 4096);

    mmu.translate_str("345678").unwrap();
    mmu.translate_str("345678").unwrap();
  }

  #[test]
  fn manually() {
    let mut mmu = MMU::new(4096);
    PrimaryMemory::create(4096);
    PAL::create(PALAlgorithm::LRU, 4096);

    ["001123", "002123", "001123", "003123", "005123", "006123"]
      .iter()
      .for_each(|address| {
        mmu.translate_str(address).unwrap();
      });
  }

  #[test]
  fn complex() {
    let mut mmu = MMU::new(4096);
    PrimaryMemory::create(4096);
    PAL::create(PALAlgorithm::LRU, 4096);

    // Generate random addresses
    let addresses = (0..10000)
      .map(|_| {
        let mut rng = rand::thread_rng();
        let address: u32 = rng.gen();
        format!("{:06x}", address)
      })
      .collect::<Vec<String>>();

    addresses.iter().for_each(|address| {
      mmu.translate_str(address).unwrap();
    });
  }
}
