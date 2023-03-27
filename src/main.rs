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

  let (_, b) = results?.iter().fold((0, 0), |(a, b), r| match r {
    TranslationResult::Hit => (a + 1, b),
    TranslationResult::Fault => (a, b + 1),
  });

  print!("{}", b);
  Ok(())
}
