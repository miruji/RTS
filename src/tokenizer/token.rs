/*
    token + methods for it
*/

pub mod token {
    #[derive(PartialEq)]
    pub enum TokenType {
        // basic
        None,    // none
        Word,    // word
        Number,  // number
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
        // parameter
        CircleBlockBegin, // (
        CircleBlockEnd,   // )
        // array
        SquareBlockBegin, // [
        SquareBlockEnd,   // ]
        // class
        FigureBlockBegin, // {
        FigureBlockEnd,   // }
        // block
        Begin, // :
    }

    impl ToString for TokenType {
        fn to_string(&self) -> String {
            match self {
                // basic
                TokenType::None    => "NONE".to_string(),    // none
                TokenType::Word    => "WORD".to_string(),    // word
                TokenType::Number  => "NUMBER".to_string(),  // number
                TokenType::Float   => "FLOAT".to_string(),   // float number
                TokenType::Endline => "ENDLINE".to_string(), // endline
                TokenType::Comma   => "COMMA".to_string(),   // ,
                TokenType::Dot     => "DOT".to_string(),     // .
                // quotes
                TokenType::BackQuote   => "BACK_QUOTE".to_string(),   // `
                TokenType::DoubleQuote => "DOUBLE_QUOTE".to_string(), // "
                TokenType::SingleQuote => "SINGLE_QUOTE".to_string(), // '
                // single math
                TokenType::Plus     => "PLUS".to_string(),     // +
                TokenType::Minus    => "MINUS".to_string(),    // -
                TokenType::Multiply => "MULTIPLY".to_string(), // *
                TokenType::Divide   => "DIVIDE".to_string(),   // /
                TokenType::Equals   => "EQUALS".to_string(),   // =
                TokenType::Modulo   => "MODULO".to_string(),   // %
                // double math
                TokenType::Increment      => "INCREMENT".to_string(),       // ++
                TokenType::PlusEquals     => "PLUS_EQUALS".to_string(),     // +=
                TokenType::Decrement      => "DECREMENT".to_string(),       // --
                TokenType::MinusEquals    => "MINUS_EQUALS".to_string(),    // -=
                TokenType::MultiplyEquals => "MULTIPLY_EQUALS".to_string(), // *=
                TokenType::DivideEquals   => "DIVIDE_EQUALS".to_string(),   // /=
                // single logical
                TokenType::GreaterThan => "GREATER_THAN".to_string(), // >
                TokenType::LessThan    => "LESS_THAN".to_string(),    // <
                TokenType::Question    => "QUESTION".to_string(),     // ?
                TokenType::Not         => "NOT".to_string(),          // !
                // double logical
                TokenType::GreaterThanOrEquals => "GREATER_THAN_OR_EQUALS".to_string(), // >=
                TokenType::LessThanOrEquals    => "LESS_THAN_OR_EQUALS".to_string(),    // <=
                TokenType::NotEquals           => "NOT_EQUALS".to_string(),             // !=
                TokenType::DoubleEquals        => "DOUBLE_EQUALS".to_string(),          // ==
                TokenType::And                 => "AND".to_string(),                    // &&
                TokenType::Or                  => "OR".to_string(),                     // ||
                // parameter
                TokenType::CircleBlockBegin => "CIRCLE_BLOCK_BEGIN".to_string(), // (
                TokenType::CircleBlockEnd   => "CIRCLE_BLOCK_END".to_string(),   // )
                // array
                TokenType::SquareBlockBegin => "SQUARE_BLOCK_BEGIN".to_string(), // [
                TokenType::SquareBlockEnd   => "SQUARE_BLOCK_END".to_string(),   // ]
                // class
                TokenType::FigureBlockBegin => "FIGURE_BLOCK_BEGIN".to_string(), // {
                TokenType::FigureBlockEnd   => "FIGURE_BLOCK_END".to_string(),   // }
                // block
                TokenType::Begin => "BEGIN".to_string(), // :
            }
        }
    }

    pub struct Token {
        pub data: String,
        pub data_type: TokenType,
    }
}