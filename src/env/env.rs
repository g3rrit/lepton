use crate::symbol_table::*;
use crate::symbol::*;
use std::collections::HashMap;

pub struct Env {
  lexer: Lexer,
  
  fn_table: FnTable,
  var_table: VarTable,
  in_table: InTable,
  mac_table: MacTable,
  
  use_stack: Vec<String>,
                 
  symbol_stack: Vec<HashMap<IdNode, Rc<Symbol>>>,
}

impl Env {
  
  pub fn ss_push_stack(&mut self) {
    self.symbol_stack.push(HashMap::new());
  }
  
  pub fn ss_pop_stack(&mut self) {
    self.symbol_stack.pop();
  }
  
  pub fn ss_push(&mut self, s: Rc<Symbol>) {
    if let Some(hmap) = self.symbol_stack.last() {
      hmap.insert(s.id(), Rc::clone(s));
    } else {
      panic!("no element in symbol stack!");
    }
  }
                 
  pub fn ss_get(&self, id: &IdNode) -> Option<Node> {
    if let Some(hmap) = self.symbol_stack.last() {
      if let Some(s) = hmap.get(id) {
        s.get()
      } else { None }
    } else { None }
  }

  pub fn get_fn(&self, id: &IdNode) -> Vec<Rc<FnNode>> {
    self.fn_table.get(id)
  }
  
  pub fn get_var(&self, id: &IdNode) -> Vec<Rc<FnNode>> {
    self.var_table.get(id)
  }
  
  pub fn get_in(&self, id: &IdNode) -> Vec<Rc<InNode>> {
    self.in_table.get(id)
  }
  
  pub fn get_mac(&self, id: &IdNode) -> Vec<Rc<MacNode>> {
    self.mac_table.get(id)
  }
  
}
