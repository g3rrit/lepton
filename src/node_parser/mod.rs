use crate::env::Env;
use crate::node::*;

use crate::fstack::*;
use std::rc::Rc;
use crate::token_parser::TokenParser;

pub trait Parser {
  fn parse(input: &mut NodeParser, env: &mut Env) -> Option<Rc<Node>>;
}

pub struct NodeParser {
  input: TokenParser,
  fs: Fstack<Node>,
}

impl NodeParser {
  
  pub fn new(input: TokenParser) -> Self {
    Self {
      input: input,
      fs: Fstack::new(),
    }
  }
  
}

impl FstackT<Node> for NodeParser {
  fn next_item(&mut self) -> Option<Rc<Node>> {
      if let Some(_tok) = self.input.next() {
      }
      None
  }
  fn fs<'a>(&'a mut self) -> &'a mut Fstack<Node> {
    &mut self.fs
  }
}
