use crate::node::Node;
use std::fmt::Display;

#[derive(Display)]
pub enum Token {
    ID(String),
    STR(String),
    CHAR(char),
    INT(u64),
    FLOAT(f64),
    EOF,
}

impl Node for Token {
    type Element = Token;
    fn get(&self) -> Token {
        self
    }
}
