use crate::node::id::IdNode;
use crate::env::Env;

pub struct FnNode {
  id: IdNode,
  params: Vec<VarNode>,
  ty: TyNode,
  body: BStmNode,
}

impl FnNode {
  pub fn eval(&self, env: &mut Env, args: Vec<ExpNode>) -> Option<Node> {
    env.ss_push_stack();
    for i in 0..args.len() {
      let var = self.params[i];
      let val = args[i];
      env.ss_push(Rc::new(DataNode::new(var.id().clone(), var.ty().clone(), *val)));
    }
    let res = self.body.eval();
    env.ss_pop_stack();
    res
  }
    
}
