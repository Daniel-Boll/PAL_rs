use clap::Parser;
use pal_rs::{
  cli::{Cli, Commands},
  mmu,
};

fn main() -> anyhow::Result<()> {
  let cli = Cli::parse();

  match &cli.command {
    Commands::Translate(opts) => mmu::entrypoint(opts),
    Commands::Analysis => unimplemented!(),
  }
}
