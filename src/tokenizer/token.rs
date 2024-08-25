/*
  token
*/

use std::fmt;

#[derive(PartialEq)]
#[derive(Clone)]
pub enum TokenType 
{
// basic
  None,    // None
  Word,    // Word
  Endline, // Endline
  Comma,   // ,
  Dot,     // .

  Comment, // #
// quotes
  RawString,          // `
  String,             // "
  Char,               // '
  FormattedRawString, // f``
  FormattedString,    // f""
  FormattedChar,      // f''
// single math
  Plus,     // +
  Minus,    // -
  Multiply, // *
  Divide,   // /
  Equals,   // =
  Modulo,   // %
  Exponent, // ^
// double math
  UnaryPlus,      // ++
  PlusEquals,     // +=

  UnaryMinus,     // --
  MinusEquals,    // -=

  UnaryMultiply,  // **
  MultiplyEquals, // *=

  UnaryDivide,    // //
  DivideEquals,   // /=

  UnaryModulo,    // %%
  ModuloEquals,   // %=

  UnaryExponent,  // ^^
  ExponentEquals, // ^=
// single logical
  GreaterThan, // >
  LessThan,    // <
  Question,    // ?
  Not,         // !
// double logical
  GreaterThanOrEquals, // >=
  LessThanOrEquals,    // <=
  NotEquals,           // !=
// brackets
  CircleBracketBegin, // (
  CircleBracketEnd,   // )
  SquareBracketBegin, // [
  SquareBracketEnd,   // ]
  FigureBracketBegin, // {
  FigureBracketEnd,   // }
// other
  Colon,   // :
  Pointer, // ->
  Tilde,   // ~
// words
  Int,      // Integer
  UInt,     // Unsigned integer
  Float,    // Float
  UFloat,   // Unsigned float
  Rational, // Rational
  Complex,  // Complex

  Array,    // Array

  Bool,      // Bool
  Joint,     // & (and)
  Disjoint,  // ^
  Inclusion, // | (or)
  Exclusion, // ! (not)
  
  Loop, // loop

  Custom(String),
}

impl ToString for TokenType 
{
  fn to_string(&self) -> String 
  {
    match self 
    {
      // basic
      TokenType::None    => String::from("None"),    // None
      TokenType::Word    => String::from("Word"),    // Word
      TokenType::Endline => String::from("\\n"),     // Endline
      TokenType::Comma   => String::from(","),       // ,
      TokenType::Dot     => String::from("."),       // .

      TokenType::Comment => String::from("Comment"), // #
      
      // quotes
      TokenType::RawString          => String::from("Raw String"),           // `
      TokenType::String             => String::from("String"),               // "
      TokenType::Char               => String::from("Char"),                 // '
      TokenType::FormattedRawString => String::from("Formatted Raw String"), // f``
      TokenType::FormattedString    => String::from("Formatted String"),     // f""
      TokenType::FormattedChar      => String::from("Formatted Char"),       // f''
     
      // single math
      TokenType::Plus     => String::from("+"), // +
      TokenType::Minus    => String::from("-"), // -
      TokenType::Multiply => String::from("*"), // *
      TokenType::Divide   => String::from("/"), // /
      TokenType::Equals   => String::from("="), // =
      TokenType::Modulo   => String::from("%"), // %
      TokenType::Exponent => String::from("^"), // ^
      
      // double math
      TokenType::UnaryPlus      => String::from("++"), // ++
      TokenType::PlusEquals     => String::from("+="), // +=

      TokenType::UnaryMinus     => String::from("--"), // --
      TokenType::MinusEquals    => String::from("-="), // -=

      TokenType::UnaryMultiply  => String::from("**"), // **
      TokenType::MultiplyEquals => String::from("*="), // *=

      TokenType::UnaryDivide    => String::from("//"), // //
      TokenType::DivideEquals   => String::from("/="), // /=

      TokenType::UnaryModulo    => String::from("%%"), // %%
      TokenType::ModuloEquals   => String::from("%="), // %=

      TokenType::UnaryExponent  => String::from("^^"), // ^^
      TokenType::ExponentEquals => String::from("^="), // ^=

      // single logical
      TokenType::GreaterThan => String::from(">"), // >
      TokenType::LessThan    => String::from("<"), // <
      TokenType::Question    => String::from("?"), // ?
      TokenType::Not         => String::from("!"), // !
      
      // double logical
      TokenType::GreaterThanOrEquals => String::from(">="),  // >=
      TokenType::LessThanOrEquals    => String::from("<="),  // <=
      TokenType::NotEquals           => String::from("!="),  // !=
      
      // brackets
      TokenType::CircleBracketBegin => String::from("("), // (
      TokenType::CircleBracketEnd   => String::from(")"), // )
      TokenType::SquareBracketBegin => String::from("["), // [
      TokenType::SquareBracketEnd   => String::from("]"), // ]
      TokenType::FigureBracketBegin => String::from("{"), // {
      TokenType::FigureBracketEnd   => String::from("}"), // }
      
      // other
      TokenType::Colon   => String::from(":"),  // :
      TokenType::Pointer => String::from("->"), // ->
      TokenType::Tilde   => String::from("~"),  // ~
      
      // words
      TokenType::Int      => String::from("Int"),      // Integer
      TokenType::UInt     => String::from("UInt"),     // Unsigned integer
      TokenType::Float    => String::from("Float"),    // Float
      TokenType::UFloat   => String::from("UFloat"),   // Unsigned float
      TokenType::Rational => String::from("Rational"), // Rational
      TokenType::Complex  => String::from("Complex"),  // Complex

      TokenType::Array    => String::from("Array"),    // Array

      TokenType::Bool      => String::from("Bool"),      // Bool
      TokenType::Joint     => String::from("Joint"),     // & (and)
      TokenType::Disjoint  => String::from("Disjoint"),  // ^
      TokenType::Inclusion => String::from("Inclusion"), // | (or)
      TokenType::Exclusion => String::from("Exclusion"), // ! (not)

      TokenType::Loop => String::from("loop"), // while

      TokenType::Custom(value) => value.clone(),
    }
  }
}

#[derive(Clone)]
pub struct Token 
{
  data:     String,     // todo: option !!!
  dataType: TokenType,  // todo: option !!!
  pub tokens: Option< Vec<Token> >,
}
impl Token 
{
  pub const fn newStatic() -> Self 
  {
    Token 
    {
      data: String::new(),
      dataType: TokenType:: None,
      tokens: None,
    }
  }
  pub fn newEmpty(
    dataType: TokenType
  ) -> Self 
  {
    Token 
    {
      data:   String::new(),
      dataType,
      tokens: None,
    }
  }
  pub fn new(
    dataType: TokenType,
    data:     String
  ) -> Self 
  {
    Token 
    {
      data,
      dataType,
      tokens: None,
    }
  }
  pub fn newFull(
    dataType: TokenType,
    data:     String,
    tokens:   Option< Vec<Token> >
  ) -> Self 
  {
    Token 
    {
      data,
      dataType,
      tokens,
    }
  }
  pub fn newNesting(
    tokens: Option< Vec<Token> >
  ) -> Self 
  {
    Token 
    {
      data:     String::new(),
      dataType: TokenType::None,
      tokens,
    }
  }

  // convert type
  // todo:
  /*
  fn convertType(&mut self) -> ()
  {
    if self.data.chars().nth(0) == Some('-') 
    {
      self.dataType = 
        match self.dataType 
        {
          TokenType::UInt => TokenType::Int,
          TokenType::UFloat => TokenType::Float,
          _ => self.dataType.clone(),
        }
    }
  }
  */

  // convert data
  fn convertData(&mut self) -> ()
  {
    if self.data.chars().nth(0) == Some('-') 
    {
      if self.dataType == TokenType::UInt || self.dataType == TokenType::UFloat 
      {
        self.data = self.data[1..].to_string()
      }
    }
  }

  //
  pub fn getDataType(&self) -> &TokenType 
  {
    &self.dataType
  }
  //
  pub fn setDataType(&mut self, newDataType: TokenType) -> ()
  {
    self.dataType = newDataType;
    self.convertData();
  }

  //
  pub fn getData(&self) -> &str 
  {
    &self.data
  }
  //
  pub fn setData(&mut self, newData: String) -> ()
  {
    self.data = newData;
    self.convertData();
  }
}
impl fmt::Display for Token 
{ // todo: debug only ?
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result 
  {
    write!(f, "{}", self.data)
  }
}
impl fmt::Debug for Token 
{ // todo: debug only ?
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result 
  {
    write!(f, "{}", self.data)
  }
}