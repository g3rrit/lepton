use std::fmt::Display;
use std::rc::Rc;
use crate::token::Token;

pub enum IdNode {
  ID(Rc<Token>),
  TY_ID(Rc<TyNode>),    // TYPE
  VAR_ID(Rc<VarNode>),  // VARIABLE
  FN_ID(Rc<FnNode>),    // FUNCTION
  IN_ID(Rc<InNode>),    // INTRINSIC
  MAC_ID(Rc<MacNode>),  // MACRO
  LB_ID(Rc<LbNode>),    // LABEL
}

pub enum LitNode {
  STR(Rc<Token>),
  CHAR(Rc<Token>),
  INT(Rc<Token>),
  FLOAT(Rc<Token>),
}

type OpNode = Rc<Token>;

type StrNode = Rc<Token>;
type IntNode = Rc<Token>;
type FloatNode = Rc<Token>;

// TODO
struct VarNode;
struct FnNode;
struct InNode;
struct MacNode;
struct LbNode;

pub enum TyNode {
  INTRINSIC(ITyNode),
  ARRAY(ATyNode),
  COMPOUND(CTyNode),
  FN(FTyNode),
}

impl TyNode {
  pub fn size(&self) -> usize {
    match self {
      INTRINSIC(var) => var.size(),
      ARRAY(var) => var.size(),
      COMPOUND(var) => var.size(),
      FN(var) => var.size(),
    }
  }
}

pub enum ITyNode {
  INT,
  UINT,
  I8,
  U8,
  I16,
  U16,
  I32,
  U32,
  I64,
  U64,
  F32,
  F64,
  BOOL,
}

impl ITyNode {
  pub fn size(&self) -> usize {
    match self {
      INT => 4, // TODO: machine dependent
      UINT => 4, // same
      I8 => 1,
      U8 => 1,
      I16 => 2,
      U16 => 2,
      I32 => 4,
      U32 => 4,
      I64 => 8,
      U64 => 8,
      F32 => 4,
      F64 => 8,
      BOOL => 1,
    }
  }
}

pub struct ATyNode {
  type: TyNode,
}

impl ATyNode {
  pub fn type(&self) -> &TyNode {
    &self.type
  }
  
  pub fn size(&self) -> usize {
    4 // TODO: ptr size
  }
}

pub struct CTyNode {
}

pub struct FTyNode {
}

pub struct VarNode;
pub struct FnNode;
pub struct LbNode;
pub struct InNode;
