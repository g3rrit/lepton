use crate::env::Env;
use crate::node::*;

use crate::fstack::*;
use std::rc::Rc;
use crate::token_parser::TokenParser;

pub struct NodeParser {
  input: TokenParser,
}

impl NodeParser {
  
  pub fn new(input: TokenParser) -> Self {
    Self {
      input: input,
    }
  }
  
  pub fn next(&self) -> Option<Node> {
    None
  }
  
}
