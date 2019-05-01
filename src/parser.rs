use crate::lexer::Lexer;
use crate::token::Token;
use crate::tag::Tagged;
use crate::node::*;
use crate::util::*;
use crate::env::Env;
use std::rc::Rc;

pub trait Parser {
  fn parse(lexer: &mut Env) -> Option<Node>;
}

/*
impl Parser for Token {
  type Item = Token;
  fn parse(lexer: &mut Lexer) -> Option<Rc<Item>> {
    Some(lexer.next())
  }
}
*/

pub struct EOF;
impl Parser for EOF {
  type Item = ();
  fn parse(lexer: &mut Lexer) -> Option(Item) {
    match *lexer.next() {
      Token::EOF => Some(()),
      _ => None,
    }
  }
}

impl Parser for Node {
  type Item = Node;
  fn parse(lexer: &mut Lexer) -> Option<Item> {
    let res = lexer.next();
    match *res {
      Token::ID(_) => {
        // check if not var node etc
        Some(Node::ID(Rc::clone(res))),
      },
      Token::STR(_) => Some(Node::STR(Rc::clone(res))),
      Token::CHAR(_) => Some(Node::CHAR(Rc::clone(res))),
      Token::INT(_) => Some(Node::INT(Rc::clone(res))),
      Token::FLOAT(_) => Some(Node::FLOAT(Rc::clone(res))),
      Token::EOF => None,

      // OPERATORS
      Token::OP_EM => Some(Node::Op(Rc::clone(res))),
      Token::OP_NS => Some(Node::Op(Rc::clone(res))),
      Token::OP_DS => Some(Node::Op(Rc::clone(res))),
      Token::OP_PC => Some(Node::Op(Rc::clone(res))),
      Token::OP_AM => Some(Node::Op(Rc::clone(res))),
      Token::OP_LP => Some(Node::Op(Rc::clone(res))),
      Token::OP_RP => Some(Node::Op(Rc::clone(res))),
      Token::OP_AS => Some(Node::Op(Rc::clone(res))),
      Token::OP_PS => Some(Node::Op(Rc::clone(res))),
      Token::OP_CM => Some(Node::Op(Rc::clone(res))),
      Token::OP_MS => Some(Node::Op(Rc::clone(res))),
      Token::OP_DP => Some(Node::Op(Rc::clone(res))),
      Token::OP_SL => Some(Node::Op(Rc::clone(res))),
      Token::OP_CN => Some(Node::Op(Rc::clone(res))),
      Token::OP_SE => Some(Node::Op(Rc::clone(res))),
      Token::OP_LT => Some(Node::Op(Rc::clone(res))),
      Token::OP_EQ => Some(Node::Op(Rc::clone(res))),
      Token::OP_GT => Some(Node::Op(Rc::clone(res))),
      Token::OP_QM => Some(Node::Op(Rc::clone(res))),
      Token::OP_AT => Some(Node::Op(Rc::clone(res))),
      Token::OP_LS => Some(Node::Op(Rc::clone(res))),
      Token::OP_BS => Some(Node::Op(Rc::clone(res))),
      Token::OP_RS => Some(Node::Op(Rc::clone(res))),
      Token::OP_CI => Some(Node::Op(Rc::clone(res))),
      Token::OP_US => Some(Node::Op(Rc::clone(res))),
      Token::OP_LB => Some(Node::Op(Rc::clone(res))),
      Token::OP_VB => Some(Node::Op(Rc::clone(res))),
      Token::OP_RB => Some(Node::Op(Rc::clone(res))),
      Token::OP_TD => Some(Node::Op(Rc::clone(res))),
    }
  }
}
