pub mod translate;

use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(author = "BDT team", version = "0.0.0", about = "A MMU module simulation", long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
  #[command(subcommand)]
  pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
  Translate(translate::TranslateOptions),
}

#[derive(ValueEnum, Debug, Clone)]
pub enum Output {
  Text,
  Json,
  Table,
}
