pub mod token;
pub mod input;

use std::rc::Rc;
use self::token::*;
use self::input::*;

use crate::fstack::*;

pub struct TokenParser {
  input: Input,
  fs: Fstack<Token>,
}

impl TokenParser {
  pub fn new(input: Input) -> Self {
    Self {
      input: input,
      fs: Fstack::new(),
    }
  }
  
}


impl FstackT<Token> for TokenParser {
  fn next_item(&mut self) -> Option<Rc<Token>> {
    Some(Rc::new(Token::ID(String::from("lol"))))
  }

  fn fs<'a>(&'a mut self) -> &'a mut Fstack<Token> {
    &mut self.fs
  }
}
