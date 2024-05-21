/*
    token
*/

#[derive(PartialEq)]
#[derive(Clone)]
pub enum TokenType {
// basic
    None,    // None
    Word,    // Word
    Endline, // Endline
    Comma,   // ,
    Dot,     // .
// quotes
    BackQuote,   // `
    DoubleQuote, // "
    SingleQuote, // '
// single math
    Plus,     // +
    Minus,    // -
    Multiply, // *
    Divide,   // /
    Equals,   // =
    Modulo,   // %
// double math
    Increment,      // ++
    PlusEquals,     // +=
    Decrement,      // --
    MinusEquals,    // -=
    MultiplyEquals, // *=
    DivideEquals,   // /=
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

    And, // and
    Or,  // or
    
    Loop, // loop

    MethodCall, // Method Call
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
            // quotes
            TokenType::BackQuote   => String::from("Back quote"),   // `
            TokenType::DoubleQuote => String::from("Double quote"), // "
            TokenType::SingleQuote => String::from("Single quote"), // '
            // single math
            TokenType::Plus     => String::from("+"), // +
            TokenType::Minus    => String::from("-"), // -
            TokenType::Multiply => String::from("*"), // *
            TokenType::Divide   => String::from("/"), // /
            TokenType::Equals   => String::from("="), // =
            TokenType::Modulo   => String::from("%"), // %
            // todo: ^ ???
            // double math
            TokenType::Increment      => String::from("++"), // ++
            TokenType::PlusEquals     => String::from("+="), // +=
            TokenType::Decrement      => String::from("--"), // --
            TokenType::MinusEquals    => String::from("-="), // -=
            TokenType::MultiplyEquals => String::from("*="), // *=
            TokenType::DivideEquals   => String::from("/="), // /=
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

            TokenType::And => String::from("and"), // and
            TokenType::Or  => String::from("or"),  // or

            TokenType::Loop => String::from("loop"), // while

            TokenType::MethodCall => String::from("Method Call"), // Method call
        }
    }
}

#[derive(Clone)]
pub struct Token {
    pub data: String,
    pub dataType: TokenType,
    pub tokens: Vec<Token>,
}
impl Token {
    pub fn newEmpty(
        dataType: TokenType
    ) -> Self {
        Token {
            data: String::new(),
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
        dataType: TokenType,
        tokens:   Vec<Token>
    ) -> Self {
        Token {
            data: String::new(),
            dataType,
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