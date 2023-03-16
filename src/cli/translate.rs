use clap::Args;

use super::Output;

#[derive(Args)]
pub struct TranslateOptions {
  /// The Page Table size in bits
  #[arg(long, default_value = "4096")]
  pub page_table_size: usize,

  /// Output format
  #[arg(long, default_value = "text")]
  pub output: Output,

  /// The trace file
  #[arg(default_value = "/dev/stdin")]
  pub trace: String,
}
