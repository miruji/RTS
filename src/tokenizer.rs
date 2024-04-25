/*
    tokenizer + methods for it
*/
pub mod token;
pub mod tokenizer {
    use crate::tokenizer::token::token::*;

    pub fn delete_comment(index: &mut usize, buffer: &[u8], buffer_length: &usize) {
        if buffer[*index] != b'#' {
            return;
        }
        *index += 1; // skip first #
        if buffer[*index] == b'#' {
            // double comment
            *index += 1;
            while *index < *buffer_length {
                *index += 1;
                if buffer[*index] == b'#' && *index + 1 < *buffer_length && buffer[*index + 1] == b'#' {
                    if *index + 2 < *buffer_length && buffer[*index + 2] == b'\n' {
                        // skip ##\n
                        *index += 3;
                    } else {
                        // skip ##text\n
                        *index += 2;
                    }
                    return;
                }
            }
        } else {
            // single comment
            *index += 1;
            while *index < *buffer_length {
                *index += 1;
                if buffer[*index] == b'\n' {
                    *index += 1; // skip \n
                    return;
                }
            }
        }
    }

    pub fn get_single_char(c: char) -> bool {
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

    pub fn get_number(index: &mut usize, buffer: &[u8], buffer_length: &usize) -> Token {
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

    pub fn get_word(index: &mut usize, buffer: &[u8], buffer_length: &usize) -> Token {
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

    pub fn get_quotes(quote: u8, index: &mut usize, buffer: &[u8]) -> Token {
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

    pub fn get_operator(index: &mut usize, buffer: &[u8]) -> Token {
        let next_char = buffer[*index+1] as char;
        match buffer[*index] as char {
            // += ++ +
            '+' => if next_char == '=' {
                *index += 2;
                return Token { data_type: TokenType::PlusEquals, data: "".to_string() };
            } else if next_char == '+' {
                *index += 2;
                return Token { data_type: TokenType::Increment, data: "".to_string() };
            } else {
                *index += 1;
                return Token { data_type: TokenType::Plus, data: "".to_string() };
            },
            // -= -- -
            '-' => if next_char == '=' {
                *index += 2;
                return Token { data_type: TokenType::MinusEquals, data: "".to_string() };
            } else if next_char == '-' {
                *index += 2;
                return Token { data_type: TokenType::Decrement, data: "".to_string() };
            } else {
                *index += 1;
                return Token { data_type: TokenType::Minus, data: "".to_string() };
            },
            // *= *
            '*' => if next_char == '=' {
                *index += 2;
                return Token { data_type: TokenType::MultiplyEquals, data: "".to_string() };
            } else {
                *index += 1;
                return Token { data_type: TokenType::Multiply, data: "".to_string() };
            },
            // /= /
            '/' => if next_char == '=' {
                *index += 2;
                return Token { data_type: TokenType::DivideEquals, data: "".to_string() };
            } else {
                *index += 1;
                return Token { data_type: TokenType::Divide, data: "".to_string() };
            },
            // >= >
            '>' => if next_char == '=' {
                *index += 2;
                return Token { data_type: TokenType::GreaterThanOrEquals, data: "".to_string() };
            } else {
                *index += 1;
                return Token { data_type: TokenType::GreaterThan, data: "".to_string() };
            },
            // <=
            '<' => if next_char == '=' {
                *index += 2;
                return Token { data_type: TokenType::LessThanOrEquals, data: "".to_string() };
            } else {
                *index += 1;
                return Token { data_type: TokenType::LessThan, data: "".to_string() };
            },
            // != !
            '!' => if next_char == '=' {
                *index += 2;
                return Token { data_type: TokenType::NotEquals, data: "".to_string() };
            } else {
                *index += 1;
                return Token { data_type: TokenType::Not, data: "".to_string() };
            },
            // == =
            '=' => if next_char == '=' {
                *index += 2;
                return Token { data_type: TokenType::DoubleEquals, data: "".to_string() };
            } else {
                *index += 1;
                return Token { data_type: TokenType::Equals, data: "".to_string() };
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
                } else
                if c == '%' {
                    *index += 1;
                    return Token { data_type: TokenType::Modulo, data: "".to_string() };
                } else
                if c == '?' {
                    *index += 1;
                    return Token { data_type: TokenType::Question, data: "".to_string() };
                }
            },
        }

        *index += 1;
        Token { data_type: TokenType::None, data: "".to_string() }
    }
}