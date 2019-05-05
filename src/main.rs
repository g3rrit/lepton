#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[macro_use]
mod util;

mod token_parser;
mod node_parser;

mod fstack;


fn main() {
  
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
