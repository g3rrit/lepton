
use crate::node::{ Node, RefNode };
use crate::lexer::Lexer;
use crate::token::Token;
use crate::util::*;

pub trait Parser<T> {
    fn parse(&self, lexer: &mut Lexer) -> Option<Box<T>>;
}
pub type RefParser<T> = Box<Parser<T>>;

// -- OR_PARSER -------------------------

struct OrParser<T> {
    comb_vec: Vec<RefParser<void>>,
}

impl Parser<T> for OrParser<T> {
    fn parse(&self, lexer: &mut Lexer) -> Option<Box<T>> {
        for parser in self.vec {
            match parser.parse(lexer) {
                Some(val) => return Some(val),
                None => ()
            }
        }
        None
    }
}

// -- AND_PARSER ------------------------

struct AndParser<T> {
    comb_vec: Vec<RefParser>,
    fold: &Fn(&mut Vec<RefNode>) -> RefNode;
}

impl Parser for AndParser {
    fn parse(&self, lexer: &mut Lexer) -> ParserRet {
        let mut node_vec: Vec<RefNode> = Vec::new();
        let n: usize = 0;
        for parser in self.vec {
            match parser.parse(lexer) {
                Some((nn, node)) => {
                    n += nn;
                    node_vec.push(node);
                },
                None => {
                    lexer.rewind(n);
                    return None;
                }
            }
        }
        Some((n, self.fold(&node_vec))
    }
}

// -- TOKEN_PARSER ----------------------

struct IdParser;

impl IdParser {
    pub fn new() -> Self {
        Self
    }
}

impl Parser for IdParser {
    fn parse(&self, lexer: &mut Lexer) -> ParserRet {
        let token = lexer.next();
        match token {
            Token::ID(s) => Some((1, Box::new(token))),
            _ => None
        }
    }
}

