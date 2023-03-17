use clap::Args;

use crate::pal::PALAlgorithm;

use super::Output;

#[derive(Args)]
pub struct TranslateOptions {
  /// The Page Table size in bits
  #[arg(long, default_value = "4096")]
  pub page_table_size: usize,

  /// Output format
  #[arg(long, default_value = "text")]
  pub output: Output,

  /// Algorithm to the PAL system
  #[arg(long, default_value = "lru")]
  pub algorithm: PALAlgorithm,

  /// The trace file
  #[arg(default_value = "/dev/stdin")]
  pub trace: String,
}
