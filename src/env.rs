use crate::symbol_table::*;

pub struct Env {
  lexer: Lexer;
  
  fn_table: FnTable;
  var_table: VarTable;
  in_table: InTable;
  mac_table: MacTable;
  
  use_stack: Vec<String
}

impl Env {
  
  
  pub fn use_stack_match(&self, &str) -> Vec<String> {
    // return string vec of possible ids
  }

  pub fn get_fn(&self, id: &str) -> Vec<Rc<FnNode>> {
    self.fn_table.get(id)
  }
  
  pub fn 
}
