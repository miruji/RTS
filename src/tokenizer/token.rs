/*
    token + methods for it
*/

pub mod token {
    #[derive(PartialEq)]
    #[derive(Clone)]
    pub enum TokenType {
        // basic
        None,    // none
        Word,    // word
        Int,     // integer
        Float,   // float number
        Endline, // endline
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
        DoubleEquals,        // ==
        And,                 // &&
        Or,                  // ||
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
        // words
        If,   // if
        Else, // else
        Elif, // else if
        
        While, // while
        For,   // for

        Final, // final
        Const, // const
    }

    impl ToString for TokenType {
        fn to_string(&self) -> String {
            match self {
                // basic
                TokenType::None    => "NONE".to_string(),  // none
                TokenType::Word    => "WORD".to_string(),  // word
                TokenType::Int     => "INT".to_string(),   // integer
                TokenType::Float   => "FLOAT".to_string(), // float number
                TokenType::Endline => "\\n".to_string(),   // endline
                TokenType::Comma   => ",".to_string(),     // ,
                TokenType::Dot     => ".".to_string(),     // .
                // quotes
                TokenType::BackQuote   => "BACK_QUOTE".to_string(),   // `
                TokenType::DoubleQuote => "DOUBLE_QUOTE".to_string(), // "
                TokenType::SingleQuote => "SINGLE_QUOTE".to_string(), // '
                // single math
                TokenType::Plus     => "+".to_string(), // +
                TokenType::Minus    => "-".to_string(), // -
                TokenType::Multiply => "*".to_string(), // *
                TokenType::Divide   => "/".to_string(), // /
                TokenType::Equals   => "=".to_string(), // =
                TokenType::Modulo   => "%".to_string(), // %
                // TO:DO: ^ ???
                // double math
                TokenType::Increment      => "++".to_string(), // ++
                TokenType::PlusEquals     => "+=".to_string(), // +=
                TokenType::Decrement      => "--".to_string(), // --
                TokenType::MinusEquals    => "-=".to_string(), // -=
                TokenType::MultiplyEquals => "*=".to_string(), // *=
                TokenType::DivideEquals   => "/=".to_string(), // /=
                // single logical
                TokenType::GreaterThan => ">".to_string(), // >
                TokenType::LessThan    => "<".to_string(), // <
                TokenType::Question    => "?".to_string(), // ?
                TokenType::Not         => "!".to_string(), // !
                // double logical
                TokenType::GreaterThanOrEquals => ">=".to_string(),  // >=
                TokenType::LessThanOrEquals    => "<=".to_string(),  // <=
                TokenType::NotEquals           => "!=".to_string(),  // !=
                TokenType::DoubleEquals        => "==".to_string(),  // ==
                TokenType::And                 => "AND".to_string(), // &&
                TokenType::Or                  => "OR".to_string(),  // ||
                // brackets
                TokenType::CircleBracketBegin => "(".to_string(), // (
                TokenType::CircleBracketEnd   => ")".to_string(), // )
                TokenType::SquareBracketBegin => "[".to_string(), // [
                TokenType::SquareBracketEnd   => "]".to_string(), // ]
                TokenType::FigureBracketBegin => "{".to_string(), // {
                TokenType::FigureBracketEnd   => "}".to_string(), // }
                // other
                TokenType::Colon => ":".to_string(),    // :
                TokenType::Pointer => "->".to_string(), // ->
                // words
                TokenType::If => "IF".to_string(),     // if
                TokenType::Else => "ELSE".to_string(), // else
                TokenType::Elif => "ELIF".to_string(), // else if

                TokenType::While => "WHILE".to_string(), // while
                TokenType::For => "FOR".to_string(),     // for

                TokenType::Final => "FINAL".to_string(), // final
                TokenType::Const => "CONST".to_string(), // const
            }
        }
    }

    #[derive(Clone)]
    pub struct Token {
        pub data: String,
        pub data_type: TokenType,
        pub tokens: Vec<Token>,
    }
    impl Token {
        pub fn new_empty(data_type: TokenType) -> Self {
            Token {
                data: String::new(),
                data_type,
                tokens: Vec::new(),
            }
        }
        pub fn new(data_type: TokenType, data: String) -> Self {
            Token {
                data,
                data_type,
                tokens: Vec::new(),
            }
        }
        pub fn new_full(data_type: TokenType, data: String, tokens: Vec<Token>) -> Self {
            Token {
                data,
                data_type,
                tokens,
            }
        }
    }
}