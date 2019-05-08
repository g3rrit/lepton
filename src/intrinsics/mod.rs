use crate::node::*;

pub fn parse(id: &str, input: &mut NodeParser, env: &Env) -> Option<Node> {
  match id {
    "int" => Node::TYINT(TintNode::new()),
    "uint" => Node::TYUINT(TuintNode::new()),
    "u8" => Node::TYU8(TyU8Node::new()),
    "i8" => Node::TYI8(TyI8Node::new()),
    "u16" => Node::TYU16(TyU16Node::new()),
    "i16" => Node::TYI16(TyI16Node::new()),
    "u32" => Node::TYU32(TyU32Node::new()),
    "i32" => Node::TYI32(TyI32Node::new()),
    "u64" => Node::TYU64(TyU64Node::new()),
    "i64" => Node::TYI64(TyI64Node::new()),
    "f32" => Node::TYF32(TyF32Node::new()),
    "f64" => Node::TYF64(TyF64Node::new()),
    
    "void" => Node::VTY(VTyNode::new()),

    "tp" => parse_tp(input, env),
    "ta" => parse_ta(input, env),
    "tf" => parse_tf(input, env),

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

fn expect<T: Span>(node: Option<T>) -> Option<T> {
  if node.is_none() {
    println!("unexpected token at {}", node.span_pos());
  }
  node
}

fn parse_tp(input: &mut NodeParser, env: &Env) -> Option<Node> {
  let ty = expect(TyNode::parse(input, env))?;
  Node::PTY(PTyNode::new(ty))
}

fn parse_ta(input: &mut NodeParser, env: &Env) -> Option<Node> {
  let ty = expect(TyNode::parse(input, env))?;
  let len = expect(ExpNode::parse(input, env))?;
  /*
  if len.ty.tag() != Tag::Int {
    println!("expected length of array to be of type integer");
    return None
  }
  */
  
  Node::ATY(ATyNode::new(ty, len))
}

fn parse_tf(input: &mut NodeParser, env: &Env) -> Option<Node> {
  let list = expect(PListNode::parse(input, env))?;
  let ty = expect(TyNode::parse(input, env))?;
  
}

fn parse_void(input: &mut NodeParser, env: &Env) -> Option<Node> {

}


fn parse_var(input: &mut NodeParser, env: &Env) -> Option<Node> {

}

    
fn parse_struct(input: &mut NodeParser, env: &Env) -> Option<Node> {

}

fn parse_union(input: &mut NodeParser, env: &Env) -> Option<Node> {

}

fn parse_enum(input: &mut NodeParser, env: &Env) -> Option<Node> {

}


    // type def
fn parse_type(input: &mut NodeParser, env: &Env) -> Option<Node> {

}

    
fn parse_sfd(input: &mut NodeParser, env: &Env) -> Option<Node> {

}

fn parse_ufd(input: &mut NodeParser, env: &Env) -> Option<Node> {

}

fn parse_efd(input: &mut NodeParser, env: &Env) -> Option<Node> {

}

    
fn parse_fn(input: &mut NodeParser, env: &Env) -> Option<Node> {

}

    
    // function declaration
fn parse_fd(input: &mut NodeParser, env: &Env) -> Option<Node> {

}

    
    // variable declaration
fn parse_vd(input: &mut NodeParser, env: &Env) -> Option<Node> {

}

    
fn parse_mac(input: &mut NodeParser, env: &Env) -> Option<Node> {

}

    
fn parse_estm(input: &mut NodeParser, env: &Env) -> Option<Node> {

}

fn parse_lstm(input: &mut NodeParser, env: &Env) -> Option<Node> {

}

fn parse_jstm(input: &mut NodeParser, env: &Env) -> Option<Node> {

}

fn parse_bstm(input: &mut NodeParser, env: &Env) -> Option<Node> {

}


