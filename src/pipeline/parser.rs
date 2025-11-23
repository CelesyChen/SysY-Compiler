use pest_derive::Parser;
use pest::Parser;
use std::path::PathBuf;
use anyhow::{self, bail};
use std::fs;

use super::ds::CompilerPipeline;

#[derive(Parser)]
#[grammar = "grammar/sysY.pest"]
struct SysYParser;

impl CompilerPipeline {
  pub fn readfile (
    &mut self,
    path: &PathBuf
  ) -> anyhow::Result<()> {
    if !path.is_file() {
      bail!("No such file.")
    }

    self.src = fs::read_to_string(&path)?;

    Ok(())
  }

  pub fn parse (
    &mut self,
  ) -> anyhow::Result<()> {
    let pairs = match SysYParser::parse(Rule::program, &self.src) {
      Ok(pairs) => pairs,
      Err(e) => {
        panic!("ERROR: {}", e)
      }
    };

    Ok(())
  }
}
