
use env::Env;
use crate::node::Node;
use crate::node_parser::NodeParser;
use std::rc::Rc;

trait Parser {
  fn parse(input: &mut NodeParser, env: &mut Env) -> Option<Rc<Node>>;
}
