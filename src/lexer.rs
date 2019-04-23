use std::rc::Rc;
use crate::util::*;
use crate::token::*;
use crate::input::*;

pub struct Lexer {
    input: Input,
    from: Vec<Rc<Token>>,
    to: Vec<Rc<Token>>,
}

impl Lexer {
    pub fn new(input: Input) -> Self {
        Self {
            input: input,
            from: Vec::new(),
            to: Vec::new(),
        }
    }
    
    fn next_token(&mut self) -> Token {
        Token::ID("lol")
    }
    
    pub fn next(&mut self) -> Rc<Token> {
        let res = match self.from.pop() {
            Some(tok) => tok,
            None => Rc::new(self.next_token()),
        };
        
        if let Token::EOF = *res {
            self.to.push(Rc::clone(&res));
        }
        
        res
    }
    
    pub fn rewind(&mut self, n: u32) {
        for _ in 0..n {
            match self.to.pop() {
                Some(tok) => self.from.push(tok),
                None => {
                    comp_warn!("unable to rewind {} Token", n);
                    return;
                }
            }
        }
    }
    
    pub fn commit(&mut self) {
        self.to.clear();
    }
}
