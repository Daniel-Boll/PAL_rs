use clap::Parser;
use pal_rs::{
  cli::{Cli, Commands},
  mmu::{self, TranslationResult},
};

fn main() -> anyhow::Result<()> {
  let cli = Cli::parse();

  let results = match &cli.command {
    Commands::Translate(opts) => mmu::entrypoint(opts),
  };

  let (total_hits, total_misses) = results?.iter().fold((0, 0), |(a, b), result| match result {
    TranslationResult::Hit => (a + 1, b),
    TranslationResult::Fault => (a, b + 1),
  });

  println!("Misses: {}", total_misses);
  println!("Hits: {}", total_hits);
  Ok(())
}
