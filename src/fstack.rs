use std::rc::Rc;

pub struct Fstack<T> {
  from: Vec<Option<Rc<T>>>,
  to:   Vec<Option<Rc<T>>>,
}

impl<T> Fstack<T> {
  pub fn new() -> Self {
    Self {
      from: Vec::new(),
      to: Vec::new(),
    }
  }
}

pub trait FstackT<T> {
  fn next_item(&mut self) -> Option<Rc<T>>;
  fn fs<'a>(&'a mut self) -> &'a mut Fstack<T>;
  
  fn next(&mut self) -> Option<Rc<T>>  {
    let res = match self.fs().from.pop() {
      Some(res) => res,
      None => self.next_item(),
    };
    match &res {
      Some(res) => self.fs().to.push(Some(Rc::clone(res))),
      None => self.fs().from.push(None),
    }
    res
  }
  
  fn rewind(&mut self, n: u32) {
    for _ in 0..n {
      match self.fs().to.pop() {
        Some(val) => self.fs().from.push(val),
        None => {
          println!("unable to rewind {} Items from Fstack", n);
          return;
        }
      }
    }
  }
  
  fn commit(&mut self) {
    self.fs().to.clear();
  }
}
