use std::str::FromStr;

// TODO: Check if we can `instantiate` one of this kind of struct with the determinated size of the
// page.
pub struct LogicalAddress {
  pub value: u64,
}

impl LogicalAddress {
  pub fn join(&self, frame: usize, offset: usize, page_size: usize) -> usize {
    (frame << (32 - page_size.ilog2())) | offset
  }
}

impl LogicalAddress {
  pub fn value(&self) -> usize {
    self.value as usize
  }

  pub fn page(&self, page_size: usize) -> usize {
    self.value() >> page_size.ilog2()
  }

  pub fn offset(&self, page_size: usize) -> usize {
    self.value() & (page_size - 1)
  }

  pub fn split(&self, page_size: usize) -> (usize, usize) {
    (self.page(page_size), self.offset(page_size))
  }
}

impl FromStr for LogicalAddress {
  type Err = std::num::ParseIntError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(LogicalAddress {
      value: u64::from_str_radix(s, 16)?,
    })
  }
}
