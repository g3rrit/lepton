pub mod mac;


use std::rc::Rc;
use crate::token::Token;

//---------------------------------------
// NODE
// Collection of all 
// possible Nodes
//---------------------------------------
pub enum Node {
  // PRIMITIVES
  ID(IdNode),
  STR(StrNode),
  CHAR(CharNode),
  INT(IntNode),
  FLOAT(FloatNode),
  
  // OPERATORS
  OP_EM, //    !    33   exclamation mark
  OP_NS, //    #    35   number sign, pound
  OP_DS, //    $    36   dollar sign
  OP_PC, //    %    37   percent sign
  OP_AM, //    &    38   ampersand
  OP_LP, //    (    40   left paranthesis
  OP_RP, //    )    41   right paranthesis
  OP_AS, //    *    42   asterix
  OP_PS, //    +    43   plus sign
  OP_CM, //    ,    44   comma
  OP_MS, //    -    45   minus sign
  OP_DP, //    .    46   decimal point
  OP_SL, //    /    47   slash
  OP_CN, //    :    58   colon
  OP_SE, //    ;    59   semicolon
  OP_LT, //    <    60   less-than sign
  OP_EQ, //    =    61   equal sign
  OP_GT, //    >    62   greater-than sign
  OP_QM, //    ?    63   question mark
  OP_AT, //    @    64   commercial sign
  OP_LS, //    [    91   left square bracket
  OP_BS, //    \    92   backslash
  OP_RS, //    ]    93   right square bracket
  OP_CI, //    ^    94   circumflex
  OP_US, //    _    95   underscore
  OP_LB, //    {    123  left brace
  OP_VB, //    |    124  vertical bar
  OP_RB, //    }    125  right brace
  OP_TD, //    ~    126  tilde

  // TYPES
  TY(TyNode),
  ITY(ITyNode), // Intrinsic Type
  PTY(PTyNode), // Pointer Type
  ATY(ATyNode), // Array Type
  CTY(CTyNode), // Compound Type
  FTY(FTyNode), // Function Type
  OTY(OTyNode), // Option Type
  VTY(VTyNode), // Void Type

  // VARIABLE
  VAR(VarNode), // Variable

  // TYPE_DECLARATION
  TDEC(TdecNode),   // Type Declaration
  STDEC(STdecNode), // Struct Type Declaration
  UTDEC(UTdecNode), // Union Type Declaration
  ETDEC(ETdecNode), // Enum Type Declaration
  
  // FUNCTION
  FN(FnNode),
  
  // MACRO 
  MAC(MacNode),
  
  // INTRINSIC_TYPE
  INTY(InTyNode), 
  
  // DATA_NODE (VARIABLE WITH DATA)
  DATA(DataNode),
  
  // STATEMENT
  STM(StmNode),
  ESTM(EStmNode), // Expression Statement
  LSTM(LStmNode), // Label Statement
  JSTM(JStmNode), // Jump Statement
  BSTM(BStmNode), // Block Statement

  // EXPRESSION
  EXP(ExpNode),
  
  // VOID
  VOID(VoidNode),
}

type IdNode = Rc<Token>;
type StrNode = Rc<Token>;
type IntNode = Rc<Token>;
type CharNode = Rc<Token>;
type FloatNode = Rc<Token>;

struct VarNode;
struct FnNode;
struct InNode;
struct MacNode;
struct LbNode;

pub enum TyNode {
  INTRINSIC(ITyNode),
  ARRAY(ATyNode),
  COMPOUND(CTyNode),
  FN(FTyNode),
}

impl TyNode {
  pub fn size(&self) -> usize {
    match self {
      INTRINSIC(var) => var.size(),
      ARRAY(var) => var.size(),
      COMPOUND(var) => var.size(),
      FN(var) => var.size(),
    }
  }
}

pub enum ITyNode {
  INT,
  UINT,
  I8,
  U8,
  I16,
  U16,
  I32,
  U32,
  I64,
  U64,
  F32,
  F64,
  BOOL,
}

impl ITyNode {
  pub fn size(&self) -> usize {
    match self {
      INT => 4, // TODO: machine dependent
      UINT => 4, // same
      I8 => 1,
      U8 => 1,
      I16 => 2,
      U16 => 2,
      I32 => 4,
      U32 => 4,
      I64 => 8,
      U64 => 8,
      F32 => 4,
      F64 => 8,
      BOOL => 1,
    }
  }
}

pub struct ATyNode {
  type: TyNode,
}

impl ATyNode {
  pub fn type(&self) -> &TyNode {
    &self.type
  }
  
  pub fn size(&self) -> usize {
    4 // TODO: ptr size
  }
}

pub struct CTyNode {
}

pub struct FTyNode {
}

pub struct VarNode;
pub struct FnNode;
pub struct LbNode;
pub struct InNode;
