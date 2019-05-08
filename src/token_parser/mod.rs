pub mod token;
pub mod input;

use std::rc::Rc;
use self::token::*;
use self::input::*;

use crate::fstack::*;

pub struct TokenParser {
  input: Input,
}

impl TokenParser {
  pub fn new(input: Input) -> Self {
    Self {
      input: input,
    }
  }
  
  pub fn next() -> Token {
    Token::EOF
  }
  
}


