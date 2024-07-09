/*
    token
*/

use std::fmt;

#[derive(PartialEq)]
#[derive(Clone)]
pub enum TokenType {
// basic
    None,    // None
    Word,    // Word
    Endline, // Endline
    Comma,   // ,
    Dot,     // .

    Comment, // #
// quotes
    SpecialString, // `
    String,        // "
    Char,          // '
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

    Bool,  // Bool
    True,  // true  = 1
    False, // false = 0
    And, // and
    Or,  // or
    
    Loop, // loop
}

impl ToString for TokenType {
    fn to_string(&self) -> String {
        match self {
            // basic
            TokenType::None    => String::from("None"),    // None
            TokenType::Word    => String::from("Word"),    // Word
            TokenType::Endline => String::from("\\n"),     // Endline
            TokenType::Comma   => String::from(","),       // ,
            TokenType::Dot     => String::from("."),       // .

            TokenType::Comment => String::from("Comment"), // #
            
            // quotes
            TokenType::SpecialString => String::from("Special String"), // `
            TokenType::String        => String::from("String"),         // "
            TokenType::Char          => String::from("Char"),           // '
           
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

            TokenType::Array  => String::from("Array"),      // Array

            TokenType::Bool  => String::from("Bool"),  // Bool
            TokenType::True  => String::from("true"),  // true  = 1
            TokenType::False => String::from("false"), // false = 0
            TokenType::And => String::from("and"), // and
            TokenType::Or  => String::from("or"),  // or

            TokenType::Loop => String::from("loop"), // while
        }
    }
}

#[derive(Clone)]
pub struct Token {
    pub data:     String,
    pub dataType: TokenType,
    pub tokens:   Vec<Token>,
}
impl Token {
    pub const fn newStatic() -> Self {
        Token {
            data: String::new(),
            dataType: TokenType:: None,
            tokens: Vec::new(),
        }
    }
    pub fn newEmpty(
        dataType: TokenType
    ) -> Self {
        Token {
            data:   String::new(),
            dataType,
            tokens: Vec::new(),
        }
    }
    pub fn new(
        dataType: TokenType,
        data:     String
    ) -> Self {
        Token {
            data,
            dataType,
            tokens: Vec::new(),
        }
    }
    pub fn newFull(
        dataType: TokenType,
        data:     String,
        tokens:   Vec<Token>
    ) -> Self {
        Token {
            data,
            dataType,
            tokens,
        }
    }
    pub fn newNesting(
        tokens: Vec<Token>
    ) -> Self {
        Token {
            data:     String::new(),
            dataType: TokenType::None,
            tokens,
        }
    }
    pub fn getData(token: &Token) -> String {
        return 
            if token.data.is_empty() {
               token.dataType.to_string()
            } else {
                token.data.clone()
            }
    }
}
impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.data)
    }
}
impl fmt::Debug for Token { // todo: remove this ?
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.data)
    }
}