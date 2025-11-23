use super::ast::Program;

pub struct CompilerPipeline {
  pub src: String,
  pub ast: Option<Program>,
}