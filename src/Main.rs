use std::fs::File;
use std::io::{self, Read};

fn delete_comment(index: &mut usize, buffer: &[u8], buffer_length: &usize) {
    if buffer[*index] == b'#' {
        *index += 1;
        if buffer[*index] == b'#' {
            // double comment
            *index += 1;
            while *index < *buffer_length {
                *index += 1;
                if buffer[*index] == b'#' && buffer[*index+1] == b'#' {
                    *index += 3; // ##ENDLINE
                    return;
                }
            }
        } else {
            // single comment
            *index += 1;
            while *index < *buffer_length {
                *index += 1;
                if buffer[*index] == b'\n' {
                    *index += 1; // ENDLINE
                    return;
                }
            }
        }
    }
}

fn get_single_char(c: char) -> bool {
    // signle math
    c == '+' || c == '-' || c == '*' || c == '/' || c == '=' || c == '%' ||
        // single logical
        c == '>' || c == '<' || c == '?' || c == '!' ||
        // block
        c == '(' || c == ')' ||
        c == '{' || c == '}' ||
        c == '[' || c == ']' ||
        // other
        c == ':' ||
        c == ';' ||
        c == ',' ||
        c == '.'
}

#[derive(PartialEq)]
enum TokenType {
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

struct Token {
    data: String,
    data_type: TokenType,
}

fn get_number(index: &mut usize, buffer: &[u8], buffer_length: &usize) -> Token {
    let mut index_buffer = *index;
    let mut result = String::new();

    let mut is_dot_checked = false;
    while index_buffer < *buffer_length {
        let current_char = buffer[index_buffer] as char;
        let next_char = if index_buffer+1 < *buffer_length {
            buffer[index_buffer+1] as char
        } else {
            '\0'
        };

        if current_char.is_digit(10) {
            result.push(current_char);
            index_buffer += 1;
        } else if current_char == '.' && !is_dot_checked && next_char.is_digit(10) {
            is_dot_checked = true;
            result.push(current_char);
            index_buffer += 1;
        } else {
            break;
        }
    }

    if !result.is_empty() {
        *index = index_buffer;
    }
    if is_dot_checked {
        return Token { data_type: TokenType::Float, data: result };
    }
    Token { data_type: TokenType::Number, data: result }
}

fn get_word(index: &mut usize, buffer: &[u8], buffer_length: &usize) -> Token {
    let mut index_buffer = *index;
    let mut result = String::new();

    while index_buffer < *buffer_length {
        let current_char = buffer[index_buffer] as char;
        let next_char = if index_buffer + 1 < *buffer_length {
            buffer[index_buffer + 1] as char
        } else {
            '\0'
        };

        if current_char.is_alphanumeric() || (current_char == '_' && !result.is_empty() && next_char.is_alphanumeric()) {
            result.push(current_char);
            index_buffer += 1;
        } else {
            break;
        }
    }

    if !result.is_empty() {
        *index = index_buffer;
    }
    Token { data_type: TokenType::Word, data: result }
}

fn get_quotes(quote: u8, index: &mut usize, buffer: &[u8]) -> Token {
    let input_length = buffer.len();
    let mut result = String::new();
    if buffer[*index] == quote {
        // if (counter+1 >= input_length) new Log(LogType.error,"[Tokenizer]: Quote was not closed at the end");

        let mut open_single_comment = false;

        while *index < input_length {
            let current_char = buffer[*index] as char;
            result.push(current_char);
            if current_char == quote as char {
                let mut no_slash = true;
                // check back slash of end quote
                if buffer[*index-1] == b'\\' {
                    let mut backslash_counter = 0;
                    for i in (*index-1)..0 {
                        if buffer[i] == b'\\' {
                            backslash_counter += 1;
                        } else {
                            break;
                        }
                    }
                    if backslash_counter % 2 == 1 {
                        no_slash = false;
                    }
                }
                //
                if open_single_comment && no_slash {
                    *index += 1;
                    break;
                } else {
                    open_single_comment = true;
                }
            }
            *index += 1;
        }
        // if (open_single_comment) new Log(LogType.error,"[Tokenizer]: Quote was not closed at the end");
    }
    if quote == b'\'' {
        return Token { data_type: TokenType::SingleQuote, data: result.clone() };
    } else if quote == b'"' {
        return Token { data_type: TokenType::DoubleQuote, data: result.clone() };
    } else if quote == b'`' {
        return Token { data_type: TokenType::BackQuote, data: result.clone() };
    }
    Token { data_type: TokenType::None, data: String::new() }
}

fn get_operator(index: &mut usize, buffer: &[u8]) -> Token {
    let next_char = buffer[*index+1] as char;
    match buffer[*index] as char {
        // += ++
        '+' => if next_char == '=' {
            *index += 2;
            return Token { data_type: TokenType::PlusEquals, data: "".to_string() };
        } else if next_char == '+' {
            *index += 2;
            return Token { data_type: TokenType::Increment, data: "".to_string() };
        },
        // -= --
        '-' => if next_char == '=' {
            *index += 2;
            return Token { data_type: TokenType::MinusEquals, data: "".to_string() };
        } else if next_char == '-' {
            *index += 2;
            return Token { data_type: TokenType::Decrement, data: "".to_string() };
        },
        // *=
        '*' => if next_char == '=' {
            *index += 2;
            return Token { data_type: TokenType::MultiplyEquals, data: "".to_string() };
        },
        // /=
        '/' => if next_char == '=' {
            *index += 2;
            return Token { data_type: TokenType::DivideEquals, data: "".to_string() };
        },
        // >=
        '>' => if next_char == '=' {
            *index += 2;
            return Token { data_type: TokenType::GreaterThanOrEquals, data: "".to_string() };
        },
        // <=
        '<' => if next_char == '=' {
            *index += 2;
            return Token { data_type: TokenType::LessThanOrEquals, data: "".to_string() };
        },
        // !=
        '!' => if next_char == '=' {
            *index += 2;
            return Token { data_type: TokenType::NotEquals, data: "".to_string() };
        },
        // ==
        '=' => if next_char == '=' {
            *index += 2;
            return Token { data_type: TokenType::DoubleEquals, data: "".to_string() };
        },
        // &&
        '&' => if next_char == '&' {
            *index += 2;
            return Token { data_type: TokenType::And, data: "".to_string() };
        },
        // ||
        '|' => if next_char == '|' {
            *index += 2;
            return Token { data_type: TokenType::Or, data: "".to_string() };
        },
        // single chars
        _ => {
            let c = buffer[*index] as char;

            // signle math
            if c == '+' {
                *index += 1;
                return Token { data_type: TokenType::Plus, data: "".to_string() };
            } else
            if c == '-' {
                *index += 1;
                return Token { data_type: TokenType::Minus, data: "".to_string() };
            } else
            if c == '*' {
                *index += 1;
                return Token { data_type: TokenType::Multiply, data: "".to_string() };
            } else
            if c == '/' {
                *index += 1;
                return Token { data_type: TokenType::Divide, data: "".to_string() };
            } else
            if c == '=' {
                *index += 1;
                return Token { data_type: TokenType::Equals, data: "".to_string() };
            } else
            if c == '%' {
                *index += 1;
                return Token { data_type: TokenType::Modulo, data: "".to_string() };
            } else
            // single logical
            if c == '>' {
                *index += 1;
                return Token { data_type: TokenType::GreaterThan, data: "".to_string() };
            } else
            if c == '<' {
                *index += 1;
                return Token { data_type: TokenType::LessThan, data: "".to_string() };
            } else
            if c == '?' {
                *index += 1;
                return Token { data_type: TokenType::Question, data: "".to_string() };
            } else
            if c == '!' {
                *index += 1;
                return Token { data_type: TokenType::Not, data: "".to_string() };
            } else
            // block
            if c == '(' {
                *index += 1;
                return Token { data_type: TokenType::CircleBlockBegin, data: "".to_string() };
            } else
            if c == ')' {
                *index += 1;
                return Token { data_type: TokenType::CircleBlockEnd, data: "".to_string() };
            } else
            if c == '{' {
                *index += 1;
                return Token { data_type: TokenType::FigureBlockBegin, data: "".to_string() };
            } else
            if c == '}' {
                *index += 1;
                return Token { data_type: TokenType::FigureBlockEnd, data: "".to_string() };
            } else
            if c == '[' {
                *index += 1;
                return Token { data_type: TokenType::SquareBlockBegin, data: "".to_string() };
            } else
            if c == ']' {
                *index += 1;
                return Token { data_type: TokenType::SquareBlockEnd, data: "".to_string() };
            } else
            // other
            if c == ':' {
                *index += 1;
                return Token { data_type: TokenType::Begin, data: "".to_string() };
            } else
            if c == ';' {
                *index += 1;
                return Token { data_type: TokenType::Endline, data: "".to_string() };
            } else
            if c == ',' {
                *index += 1;
                return Token { data_type: TokenType::Dot, data: "".to_string() };
            } else
            if c == '.' {
                *index += 1;
                return Token { data_type: TokenType::Comma, data: "".to_string() };
            }
        },
    }

    *index += 1;
    Token { data_type: TokenType::None, data: "".to_string() }
}

fn main() -> io::Result<()> {
    let file_path = "Main.s";

    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let mut tokens: Vec<Token> = vec![];

    let buffer_length = buffer.len();
    let mut index = 0;
    while index < buffer_length {
        let c = buffer[index] as char;

        // delete comment
        if c == '#' {
            delete_comment(&mut index, &buffer, &buffer_length);
        } else
        // get endline + end of file
        if c == '\n' {
            tokens.push( Token { data_type: TokenType::Endline, data: "".to_string()} );
            index += 1;
        } else
        // get number
        if c.is_digit(10) {
            tokens.push( get_number(&mut index, &buffer, &buffer_length) );
        } else
        // get word
        if c.is_alphabetic() {
            tokens.push( get_word(&mut index, &buffer, &buffer_length) );
        } else
        // get quotes ' " `
        if c == '\'' || c == '"' || c == '`' {
            let token = get_quotes(buffer[index], &mut index, &buffer);
            if token.data_type != TokenType::None {
                tokens.push(token);
            } else {
                index += 1;
            }
        } else
        // get single and double chars
        if get_single_char(c) {
            let token = get_operator(&mut index, &buffer);
            if token.data_type != TokenType::None {
                tokens.push(token);
            } else {
                index += 1;
            }
            // skip
        } else {
            index += 1;
        }
    }

    // output tokens
    for token in &tokens {
        if !token.data.is_empty() {
            println!("[{}]: [{}]", token.data_type.to_string(), token.data);
        } else {
            println!("[{}]", token.data_type.to_string());
        }
    }

    Ok(())
}