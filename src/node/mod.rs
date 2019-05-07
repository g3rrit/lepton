use crate::node_parser::Parser;


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
  OP_EM(OpEmNode), //    !    33   exclamation mark
  OP_PC(OpPcNode), //    %    37   percent sign
  OP_AM(OpAmNode), //    &    38   ampersand
  OP_AS(OpAsNode), //    *    42   asterix
  OP_PS(OpPsNode), //    +    43   plus sign
  OP_MS(OpMsNode), //    -    45   minus sign
  OP_DP(OpDpNode), //    .    46   decimal point
  OP_SL(OpSlNode), //    /    47   slash
  OP_CN(OpCnNode), //    :    58   colon
  OP_SE(OpSeNode), //    ;    59   semicolon
  OP_EQ(OpEqNode), //    =    61   equal sign
  OP_QM(OpQmNode), //    ?    63   question mark
  OP_AT(OpAtNode), //    @    64   commercial sign
  OP_BS(OpBsNode), //    \    92   backslash
  OP_CI(OpCiNode), //    ^    94   circumflex
  OP_US(OpUsNode), //    _    95   underscore
  OP_VB(OpVbNode), //    |    124  vertical bar
  OP_TD(OpTdNode), //    ~    126  tilde

  // TYPES
  TY(TyNode),
  MTY(MTyNode), // Primitive Type
  PTY(PTyNode), // Pointer Type
  ATY(ATyNode), // Array Type
  CTY(CTyNode), // Compound Type
  FTY(FTyNode), // Function Type
  OTY(OTyNode), // Option Type
  VTY(VTyNode), // Void Type
  ITY(ITyNode), // Intrinsic Type
  
  // LIST
  LIST(ListNode),
  PLIST(PListNode), // Paranthesis List ( )
  BLIST(BListNode), // Bracket List { }
  SLIST(SListNode), // Square Bracket List [ ]
  ALIST(AListNode), // Angle Bracket List < >

  // VARIABLE
  VAR(VarNode), // Variable

  // TYPE_DECLARATION
  TDCL(TdclNode), // Type Declaration
  SDCL(SdclNode), // Struct Type Declaration
  UDCL(UdclNode), // Union Type Declaration
  EDCL(EdclNode), // Enum Type Declaration
  
  // TYPE_ALIASING
  TYPE(TypeNode),
  
  // TYPE_FORWARD_DECLARATION
  TFDCL(TfdclNode), // Type Forward Declaration
  SFDCL(SfdclNode), // Struct Forward Declaration
  UFDCL(UfdclNode), // Union Forward Declaration
  EFDCL(EfdclNode), // Enum Forward Declaration
  
  // FUNCTION
  FN(FnNode),
  
  // DECLARATIONS
  FNDCL(FnDclNode),
  VARDCL(VarDclNode),
  
  // MACRO 
  MAC(MacNode),
  
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

// PRIMITIVES
pub struct IdNode;
pub struct StrNode;
pub struct CharNode;
pub struct IntNode;
pub struct FloatNode;

// OPERATORS
pub struct OpEmNode; //    !    33   exclamation mark
pub struct OpNsNode; //    #    35   number sign, pound
pub struct OpDsNode; //    $    36   dollar sign
pub struct OpPcNode; //    %    37   percent sign
pub struct OpAmNode; //    &    38   ampersand
pub struct OpLpNode; //    (    40   left paranthesis
pub struct OpRpNode; //    )    41   right paranthesis
pub struct OpAsNode; //    *    42   asterix
pub struct OpPsNode; //    +    43   plus sign
pub struct OpCmNode; //    ,    44   comma
pub struct OpMsNode; //    -    45   minus sign
pub struct OpDpNode; //    .    46   decimal point
pub struct OpSlNode; //    /    47   slash
pub struct OpCnNode; //    :    58   colon
pub struct OpSeNode; //    ;    59   semicolon
pub struct OpLtNode; //    <    60   less-than sign
pub struct OpEqNode; //    =    61   equal sign
pub struct OpGtNode; //    >    62   greater-than sign
pub struct OpQmNode; //    ?    63   question mark
pub struct OpAtNode; //    @    64   commercial sign
pub struct OpLsNode; //    [    91   left square bracket
pub struct OpBsNode; //    \    92   backslash
pub struct OpRsNode; //    ]    93   right square bracket
pub struct OpCiNode; //    ^    94   circumflex
pub struct OpUsNode; //    _    95   underscore
pub struct OpLbNode; //    {    123  left brace
pub struct OpVbNode; //    |    124  vertical bar
pub struct OpRbNode; //    }    125  right brace
pub struct OpTdNode; //    ~    126  tilde

// TYPES
pub struct TyNode;
pub struct MTyNode; // Primitive Type
pub struct ITyNode; // Intrinsic Type
pub struct PTyNode; // Pointer Type
pub struct ATyNode; // Array Type
pub struct CTyNode; // Compound Type
pub struct FTyNode; // Function Type
pub struct OTyNode; // Option Type
pub struct VTyNode; // Void Type

// LIST
pub struct ListNode;
pub struct PListNode;
pub struct BListNode;
pub struct SListNode;
pub struct AListNode;

// VARIABLE
pub struct VarNode; // Variable

// TYPE_DECLARATION
pub struct TdclNode;   // Type Declaration
pub struct SdclNode; // pub Struct Type Declaration
pub struct UdclNode; // Union Type Declaration
pub struct EdclNode; // Enum Type Declaration

// TYPE_ALIASING
pub struct TypeNode;

// TYPE_FORWARD_DECLARATION
pub struct TfdclNode; // Type Forward Declaration
pub struct SfdclNode; // Struct Forward Declaration
pub struct UfdclNode; // Union Forward Declaration
pub struct EfdclNode; // Enum Forward Declaration

// FUNCTION
pub struct FnNode;

  // DECLARATIONS
pub struct FnDclNode;
pub struct VarDclNode;

// MACRO 
pub struct MacNode;

// INTRINSIC_TYPE
pub struct InTyNode;

// DATA_NODE (VARIABLE WITH DATA)
pub struct DataNode;

// STATEMENT
pub struct StmNode;
pub struct EStmNode; // Expression Statement
pub struct LStmNode; // Label Statement
pub struct JStmNode; // Jump Statement
pub struct BStmNode; // Block Statement

// EXPRESSION
pub struct ExpNode;

// VOID
pub struct VoidNode;


/*
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
*/
