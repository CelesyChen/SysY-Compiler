use pest::Parser;
use pest_derive::Parser;
use std::path::Path;
use anyhow;
use std::fs;

#[derive(Parser)]
#[grammar = "grammar/sysY.pest"]
struct IdentParser;

pub fn parse_all_tests() -> anyhow::Result<()> {
  let test_dir = Path::new("./test");
  for entry in fs::read_dir(test_dir)? {
    let entry = entry?;

    let path = entry.path();
    let pathname = path.to_str().unwrap();

    if !path.is_file() || pathname.ends_with(".in") || pathname.ends_with(".out") {
      continue;
    }

    println!("Parsing file: {}", path.display());

    let text = fs::read_to_string(&path)?;

    match IdentParser::parse(Rule::program, &text) {
      Ok(pairs) => {
        // println!("{:#?}", pairs);
      }

      Err(e) => {
        println!("ERROR parsing {} : {}", path.display(), e);
      }
    }
  }

  Ok(())
}

fn main() -> anyhow::Result<()> {
  parse_all_tests()?;
  Ok(())
}
