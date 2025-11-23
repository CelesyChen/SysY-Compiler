use pest_derive::Parser;
use pest::Parser;
use std::path::PathBuf;
use anyhow::{self, bail};
use std::fs;

#[derive(Parser)]
#[grammar = "grammar/sysY.pest"]
struct SysYParser;

pub fn parse_file(
  path: &PathBuf
) -> anyhow::Result<String> {
  if !path.is_file() {
    bail!("No such file.")
  }

  let segment = fs::read_to_string(&path)?;

  let _ = match SysYParser::parse(Rule::program, &segment) {
    Ok(pairs) => pairs,
    Err(e) => {
      panic!("ERROR parsing {} : {}", path.display(), e)
    }
  };

  Ok("Ok".to_string())
}