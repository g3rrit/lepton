
use env::Env;
use node_parser::node::Node;
use node_parser::NodeParser;
use std::rc::Rc;

trait Parser {
  fn parse(input: &mut NodeParser, env: &mut Env) -> Option<Rc<Node>>;
}
