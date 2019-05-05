use std::rc::Rc;
use crate::node::*;

//---------------------------------------
// FUNCTION_TABLE
//---------------------------------------

pub struct FnTable {
  vec: Vec<Rc<FnNode>>,
}

impl FnTable {
  pub fn get(&self, id: &IdNode) -> Vec<Rc<FnNode>> {
    Vec::new()
  }
}

//---------------------------------------
// VARIABLE_TABLE
//---------------------------------------

pub struct VarTable {
  vec: Vec<Rc<VarNode>>,
}

impl VarTable {
  pub fn get(&self, id: &IdNode) -> Vec<Rc<VarNode>> {
    Vec::new()
  }
}

//---------------------------------------
// INTRINSIC_TABLE
//---------------------------------------

pub struct InTable {
  vec: Vec<Rc<InNode>>,
}

impl InTable {
  pub fn get(&self, id: &IdNode) -> Vec<Rc<InNode>> {
    Vec::new()
  }
}

//---------------------------------------
// MACRO_TABLE
//---------------------------------------

pub struct MacTable {
  vec: Vec<Rc<MacNode>>,
}

impl MacTable {
  pub fn get(&self, id: &IdNode) -> Vec<Rc<MacNode>> {
    Vec::new()
  }
}
