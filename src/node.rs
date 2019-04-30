use std::fmt::Display;
use std::rc::Rc;

pub enum Node {
  ID(Rc<Token>),
  STR(Rc<Token>),
  INT(Rc<Token>),
  FLOAT(Rc<Token>),
  OP(Rc<Token>),         
  VAR_ID(Rc<VAR_NODE>),  // VARIABLE
  FN_ID(Rc<FN_NODE>),    // FUNCTION
  IN_ID(Rc<IN_NODE>),    // INTRINSIC
  LB_ID(Rc<LB_NODE>),    // LABEL
}
