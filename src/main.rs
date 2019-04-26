#![allow(dead_code)]

#[macro_use]
mod util;
mod input;
mod token;
mod lexer;
mod parser;
mod node;

use input::*;
use util::*;
use parser::*;
use lexer::*;



fn main() {

  let mut input = Input::new("test_files/test.txt");
  
  comp_warn!("test {} lol {} sdf", 14, "sd");
  
  println!("read: {}", input.next().unwrap());
  println!("read: {}", input.next().unwrap());
  println!("read: {}", input.next().unwrap());
  
  input.rewind(2);
  
  println!("read: {}", input.next().unwrap());
  input.commit();
  
  input.rewind(1);
  
  println!("read: {}", input.next().unwrap());

  let id_parser : IdParser =  IdParser::new();
  let mut lexer = Lexer::new();
  let res = id_parser.parse(&mut lexer);
  match res {
    Some((n, node)) => println!("n: {} node: {}", n, node),
    None => println!("didnt match id"),
  }
}
