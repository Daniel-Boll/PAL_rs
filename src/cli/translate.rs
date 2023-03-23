use clap::Args;

use crate::pal::PALAlgorithm;

use super::Output;

#[derive(Args)]
pub struct TranslateOptions {
  /// The Page Table size in bits
  #[arg(long, default_value = "4096")]
  pub page_table_size: usize,

  /// The PAL Table entries
  #[arg(long, default_value = "4096")]
  pub pal_table_entries: usize,

  /// Algorithm to the PAL system
  #[arg(long, default_value = "lru")]
  pub algorithm: PALAlgorithm,

  /// The trace file
  #[arg(default_value = "/dev/stdin")]
  pub trace: String,

  /// Output format
  #[arg(long, default_value = "text")]
  pub output: Output,
}
