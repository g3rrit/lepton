

#[derive(Debug)]
pub enum Token {
  ID(String),   //                        id
  IFID(String), // Intrinsic function ID [fn]
  ITID(String), // Intrinsic type ID     <ty>
  STR(String),  //                       "st"
  CHAR(char),   //                       'c'
  INT(u64),     //                        10
  FLOAT(f64),   //                        1f
  EOF, 

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
