use std::{
  fs::File,
  io::{BufRead, BufReader},
  str::FromStr,
};

use super::address::LogicalAddress;

pub struct Trace {
  addresses: Vec<LogicalAddress>,
}

// Impl from file
impl Trace {
  pub fn new() -> Self {
    Self {
      addresses: Vec::new(),
    }
  }

  pub fn add(&mut self, address: LogicalAddress) {
    self.addresses.push(address);
  }

  pub fn get(&self, index: usize) -> &LogicalAddress {
    &self.addresses[index]
  }

  pub fn len(&self) -> usize {
    self.addresses.len()
  }

  #[must_use]
  pub fn is_empty(&self) -> bool {
    self.len() == 0
  }

  pub fn from_file(path: &str) -> anyhow::Result<Self> {
    let mut trace = Self::new();

    let file = File::open(path)?;
    let reader = BufReader::new(file);

    reader
      .lines()
      .for_each(|line| trace.add(LogicalAddress::from_str(&line.unwrap()).unwrap()));

    Ok(trace)
  }
}

impl Default for Trace {
    fn default() -> Self {
        Self::new()
    }
}

// impl iter to trace
impl IntoIterator for Trace {
  type Item = LogicalAddress;
  type IntoIter = std::vec::IntoIter<Self::Item>;

  fn into_iter(self) -> Self::IntoIter {
    self.addresses.into_iter()
  }
}
