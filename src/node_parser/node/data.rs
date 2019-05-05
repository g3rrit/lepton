

pub struct DataNode {
  id: IdNode,
  ty: TyNode,
  val: ExpNode,
}

impl DataNode {
  pub fn new(id: IdNode, ty: TyNode, val: ExpNode) -> Self {
    Self {
      id, ty, val
    }
  }
  
  
}
