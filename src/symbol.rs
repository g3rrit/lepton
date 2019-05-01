
pub trait Symbol {
  fn get(&self) -> Option<Node>;
  fn id(&self) -> IdNode;
}
