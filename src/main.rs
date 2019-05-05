#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[macro_use]
mod util;

mod token_parser;
mod node_parser;

mod fstack;

extern crate macro_lib;
use macro_lib::*;

struct One;
impl One { fn parse(&self) -> Option<()> { println!("lol inone"); None } }

struct Two;
impl Two { fn parse(&self) -> Option<()> { println!("lol intwo"); None } }

trait Parser {
  fn parse(&self) -> Option<()>;
}

#[derive(EnumParser)]
enum TestE {
  ONE(One),
  TWO(Two),
}



fn main() {
  
  let foo = TestE::ONE(One);
  foo.parse();
  
  use token_parser::input::*;

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

}
