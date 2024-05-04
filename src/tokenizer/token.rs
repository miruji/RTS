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
        If, // if
        El, // else
        Ef, // else if

        And, // and
        Or,  // or
        
        Loop, // loop

        Final, // final
        Const, // const
    }

    impl ToString for TokenType {
        fn to_string(&self) -> String {
            match self {
                // basic
                TokenType::None    => "none".to_string(),  // none
                TokenType::Word    => "word".to_string(),  // word
                TokenType::Int     => "int".to_string(),   // integer
                TokenType::Float   => "float".to_string(), // float number
                TokenType::Endline => "\\n".to_string(),   // endline
                TokenType::Comma   => ",".to_string(),     // ,
                TokenType::Dot     => ".".to_string(),     // .
                // quotes
                TokenType::BackQuote   => "back quote".to_string(),   // `
                TokenType::DoubleQuote => "double quote".to_string(), // "
                TokenType::SingleQuote => "single quote".to_string(), // '
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
                TokenType::If   => "if".to_string(), // if
                TokenType::El => "el".to_string(),   // else
                TokenType::Ef => "ef".to_string(),   // else if

                TokenType::And => "and".to_string(), // and
                TokenType::Or  => "or".to_string(),  // or

                TokenType::Loop => "loop".to_string(), // while

                TokenType::Final => "final".to_string(), // final
                TokenType::Const => "const".to_string(), // const
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