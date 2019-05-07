use crate::node::*;

type Orn = Option<Rc<Node>>

pub fn parse(id: &str, input: &mut NodeParser, env: &Env) -> Orn {
  match id {
    "int" => parse_int(input, env),
    "uint" => parse_uint(input, env),
    "u8" => parse_u8(input, env),
    "i8" => parse_i8(input, env),
    "u16" => parse_u16(input, env),
    "i16" => parse_i16(input, env),
    "u32" => parse_u32(input, env),
    "i32" => parse_i32(input, env),
    "u64" => parse_u64(input, env),
    "i64" => parse_i64(input, env),
    "f32" => parse_f32(input, env),
    "f64" => parse_f64(input, env),
    

    "tp" => parse_tp(input, env),
    "ta" => parse_ta(input, env),
    "tf" => parse_tf(input, env),
    "void" => parse_void(input, env),

    "var" => parse_var(input, env),
    
    "struct" => parse_struct(input, env),
    "union" => parse_union(input, env),
    "enum" => parse_enum(input, env),

    // type def
    "type" => parse_type(input, env),
    
    "sfd" => parse_sfd(input, env),
    "ufd" => parse_ufd(input, env),
    "efd" => parse_efd(input, env),
    
    "fn" => parse_fn(input, env),
    
    // function declaration
    "fd" => parse_fd(input, env),
    
    // variable declaration
    "vd" => parse_vd(input, env),
    
    "mac" => parse_mac(input, env),
    
    "estm" => parse_estm(input, env),
    "lstm" => parse_lstm(input, env),
    "jstm" => parse_jstm(input, env),
    "bstm" => parse_bstm(input, env),

    _ => {
      println!("undefined intrinsic");
      None
    }
  }
}

fn expect<T: Span>(node: Option<Rc<T>>) -> Option<Rc<T>> {
  if node.is_none() {
    println!("unexpected token at {}", node.span_pos());
  }
  node
}

fn parse_tp(input: &mut NodeParser, env: &Env) -> Option<Rc<Node>> {
  expect(TyNode::parse(input, env))
}

fn parse_ta(input: &mut NodeParser, env: &Env) -> Option<Rc<Node>> {
  let ty = expect(TyNode::parse(input, env));
  let len = expect(ExpNode::parse(input, env));
  if len.ty.tag() != Tag::Int {
    println!("expected length of array to be of type integer");
    return None
  }
}

fn parse_tf(input: &mut NodeParser, env: &Env) -> Option<Rc<Node>> {

}

fn parse_void(input: &mut NodeParser, env: &Env) -> Option<Rc<Node>> {

}


fn parse_var(input: &mut NodeParser, env: &Env) -> Option<Rc<Node>> {

}

    
fn parse_struct(input: &mut NodeParser, env: &Env) -> Option<Rc<Node>> {

}

fn parse_union(input: &mut NodeParser, env: &Env) -> Option<Rc<Node>> {

}

fn parse_enum(input: &mut NodeParser, env: &Env) -> Option<Rc<Node>> {

}


    // type def
fn parse_type(input: &mut NodeParser, env: &Env) -> Option<Rc<Node>> {

}

    
fn parse_sfd(input: &mut NodeParser, env: &Env) -> Option<Rc<Node>> {

}

fn parse_ufd(input: &mut NodeParser, env: &Env) -> Option<Rc<Node>> {

}

fn parse_efd(input: &mut NodeParser, env: &Env) -> Option<Rc<Node>> {

}

    
fn parse_fn(input: &mut NodeParser, env: &Env) -> Option<Rc<Node>> {

}

    
    // function declaration
fn parse_fd(input: &mut NodeParser, env: &Env) -> Option<Rc<Node>> {

}

    
    // variable declaration
fn parse_vd(input: &mut NodeParser, env: &Env) -> Option<Rc<Node>> {

}

    
fn parse_mac(input: &mut NodeParser, env: &Env) -> Option<Rc<Node>> {

}

    
fn parse_estm(input: &mut NodeParser, env: &Env) -> Option<Rc<Node>> {

}

fn parse_lstm(input: &mut NodeParser, env: &Env) -> Option<Rc<Node>> {

}

fn parse_jstm(input: &mut NodeParser, env: &Env) -> Option<Rc<Node>> {

}

fn parse_bstm(input: &mut NodeParser, env: &Env) -> Option<Rc<Node>> {

}


