mod cli;
mod cmdb;
mod ninjagen;
mod utils;

use anyhow::Result;
use clap::Parser;
use cli::Opts;

fn main() -> Result<()> {
  let opts: Opts = Opts::parse();
  let mut ng = ninjagen::NinjaGen::new(opts.into())?;
  ng.generate()?;
  Ok(())
}
