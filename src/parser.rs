use pest_derive::Parser;
use pest::Parser;

#[derive(Parser)]
#[grammar = "grammar/sysY.pest"]
struct IdentParser;