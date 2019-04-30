use crate::node::Node;
use crate::token::Token;

enum Tag {
  ID,
  STR,
  INT,
  FLOAT,
  VAR_ID,
  FN_ID,
  IN_ID,
  LB_ID,
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
}

trait Tagged {
  fn tag() -> Tag;
} 

impl Tagged for Node {
  fn tag(&self) -> Tag {
    match self {
      Node::ID(_) -> Tag::ID,
      Node::STR(_) -> Tag::STR,
      Node::INT(_) -> Tag::INT,
      Node::FLOAT(_) -> Tag::FLOAT,
      Node::VAR_ID(_) -> Tag::VAR_ID,
      Node::FN_ID(_) -> Tag::FN_IDj,
      Node::IN_ID(_) -> Tag::IN_ID,
      Node::LB_ID(_) -> Tag::LB_ID,
      Node::OP(token) -> {
        match *token {
          Token::OP_EM -> Tag::OP_EM,
          Token::OP_NS -> Tag::OP_NS,
          Token::OP_DS -> Tag::OP_DS,
          Token::OP_PC -> Tag::OP_PC,
          Token::OP_AM -> Tag::OP_AM,
          Token::OP_LP -> Tag::OP_LP,
          Token::OP_RP -> Tag::OP_RP,
          Token::OP_AS -> Tag::OP_AS,
          Token::OP_PS -> Tag::OP_PS,
          Token::OP_CM -> Tag::OP_CM,
          Token::OP_MS -> Tag::OP_MS,
          Token::OP_DP -> Tag::OP_DP,
          Token::OP_SL -> Tag::OP_SL,
          Token::OP_CN -> Tag::OP_CN,
          Token::OP_SE -> Tag::OP_SE,
          Token::OP_LT -> Tag::OP_LT,
          Token::OP_EQ -> Tag::OP_EQ,
          Token::OP_GT -> Tag::OP_GT,
          Token::OP_QM -> Tag::OP_QM,
          Token::OP_AT -> Tag::OP_AT,
          Token::OP_LS -> Tag::OP_LS,
          Token::OP_BS -> Tag::OP_BS,
          Token::OP_RS -> Tag::OP_RS,
          Token::OP_CI -> Tag::OP_CI,
          Token::OP_US -> Tag::OP_US,
          Token::OP_LB -> Tag::OP_LB,
          Token::OP_VB -> Tag::OP_VB,
          Token::OP_RB -> Tag::OP_RB,
          Token::OP_TD -> Tag::OP_TD,
          _ -> panic!("undefined operator"),
        }
      }
    }
  }

  fn is_tag(&self, tag: Tag) -> Bool {
    if let tag 
  }
}
