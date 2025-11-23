

// Top Level

#[derive(Debug, Clone)]
pub struct Program {
  pub items: Vec<Item>,
}

#[derive(Debug, Clone)]
pub enum Item {
  Func(FuncDef),
  Decl(Decl),
}

// Declarations

#[derive(Debug, Clone)]
pub enum Decl {
  Const(ConstDecl),
  Var(VarDecl),
}

#[derive(Debug, Clone)]
pub struct ConstDecl {
  pub ty: BType,
  pub defs: Vec<ConstDef>,
}

#[derive(Debug, Clone)]
pub struct ConstDef {
  pub ident: String,
  pub dims: Vec<Expr>,           // [ConstExp]
  pub init: ConstInitVal,
}

#[derive(Debug, Clone)]
pub enum ConstInitVal {
  Expr(Expr),
  List(Vec<ConstInitVal>),       // { a, { b, c }, d }
}

#[derive(Debug, Clone)]
pub struct VarDecl {
  pub ty: BType,
  pub defs: Vec<VarDef>,
}

#[derive(Debug, Clone)]
pub struct VarDef {
  pub ident: String,
  pub dims: Vec<Expr>,           // [ConstExp]
  pub init: Option<InitVal>,
}

#[derive(Debug, Clone)]
pub enum InitVal {
  Expr(Expr),
  List(Vec<InitVal>),
}

#[derive(Debug, Clone)]
pub enum BType {
  Int,
  Float,
}

// Function

#[derive(Debug, Clone)]
pub struct FuncDef {
  pub ret_ty: FuncType,
  pub name: String,
  pub params: Vec<FuncParam>,
  pub body: Block,
}

#[derive(Debug, Clone)]
pub enum FuncType {
  Int,
  Float,
  Void,
}

#[derive(Debug, Clone)]
pub struct FuncParam {
  pub ty: BType,
  pub name: String,
  pub dims: Vec<Option<Expr>>,   // a[][3][x]  => [None, Some(3), Some(x)]
}

// Statements

#[derive(Debug, Clone)]
pub enum Stmt {
  Block(Block),
  If {
    cond: Expr,
    then_branch: Box<Stmt>,
    else_branch: Option<Box<Stmt>>,
  },
  While {
    cond: Expr,
    body: Box<Stmt>,
  },
  Break,
  Continue,
  Return(Option<Expr>),
  Assign {
    target: LVal,
    value: Expr,
  },
  Expr(Option<Expr>),            // Exp? ;
}

#[derive(Debug, Clone)]
pub struct Block {
  pub items: Vec<BlockItem>,
}

#[derive(Debug, Clone)]
pub enum BlockItem {
  Decl(Decl),
  Stmt(Stmt),
}

// Expressions

#[derive(Debug, Clone)]
pub enum Expr {
  Literal(Lit),
  LVal(LVal),
  Ident(String),         
  Call {
    func: String,
    args: Vec<Expr>,
  },
  Unary {
    op: UnaryOp,
    expr: Box<Expr>,
  },
  Binary {
    op: BinaryOp,
    left: Box<Expr>,
    right: Box<Expr>,
  },
}

#[derive(Debug, Clone)]
pub struct LVal {
  pub ident: String,
  pub indices: Vec<Expr>,
}

#[derive(Debug, Clone)]
pub enum Lit {
  Int(i64),
  Float(f64),
}

#[derive(Debug, Clone)]
pub enum UnaryOp {
  Pos,
  Neg,
  Not,
}

#[derive(Debug, Clone)]
pub enum BinaryOp {
  Add,
  Sub,
  Mul,
  Div,
  Mod,

  Lt,
  Gt,
  Le,
  Ge,

  Eq,
  Ne,

  And,
  Or,
}