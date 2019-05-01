use crate::node::id::IdNode;
use crate::env::Env;

pub struct MacNode {
  id: IdNode,
  params: Vec<IVarNode>,
  ty: Node,
  body: IBStmNode

  parse_vec: Vec<fn(env: &mut Env) -> Option<Node>>,
}

impl MacNode {
  
  pub fn eval(&self, env: &mut Env, args: Vec<Node>) -> Option<Node> {
    
  }
  
  pub fn parse(&self, env: &mut Env) -> Option<Node> {
    let mut pvec: Vec<Node> = Vec::new();
    for pf in parse_vec {
      if let Some(node) = pf(env) {
        pvec.push(node);
      } else {
        return None;
      }
    }


}
}
