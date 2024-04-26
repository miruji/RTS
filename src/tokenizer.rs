/*
    tokenizer + methods for it
*/

pub mod token;
pub mod line;
pub mod tokenizer {
    use crate::tokenizer::token::token::*;
    use crate::tokenizer::line::line::*;

    fn delete_comment(index: &mut usize, buffer: &[u8], buffer_length: &usize) {
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
        //
        return if result == "if" {
            Token { data_type: TokenType::If, data: String::new() }
        } else if result == "else" {
            Token { data_type: TokenType::Else, data: String::new() }
        } else if result == "elif" {
            Token { data_type: TokenType::Elif, data: String::new() }
        } else if result == "while" {
            Token { data_type: TokenType::While, data: String::new() }
        } else if result == "for" {
            Token { data_type: TokenType::For, data: String::new() }
        } else if result == "final" {
            Token { data_type: TokenType::Final, data: String::new() }
        } else if result == "const" {
            Token { data_type: TokenType::Const, data: String::new() }
        } else {
            Token { data_type: TokenType::Word, data: result }
        };
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
            // += ++ +
            '+' => if next_char == '=' {
                *index += 2;
                return Token { data_type: TokenType::PlusEquals, data: String::new() };
            } else if next_char == '+' {
                *index += 2;
                return Token { data_type: TokenType::Increment, data: String::new() };
            } else {
                *index += 1;
                return Token { data_type: TokenType::Plus, data: String::new() };
            },
            // -= -- -
            '-' => if next_char == '=' {
                *index += 2;
                return Token { data_type: TokenType::MinusEquals, data: String::new() };
            } else if next_char == '-' {
                *index += 2;
                return Token { data_type: TokenType::Decrement, data: String::new() };
            } else if next_char == '>' {
                *index += 2;
                return Token { data_type: TokenType::Pointer, data: String::new() };
            } else {
                *index += 1;
                return Token { data_type: TokenType::Minus, data: String::new() };
            },
            // *= *
            '*' => if next_char == '=' {
                *index += 2;
                return Token { data_type: TokenType::MultiplyEquals, data: String::new() };
            } else {
                *index += 1;
                return Token { data_type: TokenType::Multiply, data: String::new() };
            },
            // /= /
            '/' => if next_char == '=' {
                *index += 2;
                return Token { data_type: TokenType::DivideEquals, data: String::new() };
            } else {
                *index += 1;
                return Token { data_type: TokenType::Divide, data: String::new() };
            },
            // >= >
            '>' => if next_char == '=' {
                *index += 2;
                return Token { data_type: TokenType::GreaterThanOrEquals, data: String::new() };
            } else {
                *index += 1;
                return Token { data_type: TokenType::GreaterThan, data: String::new() };
            },
            // <=
            '<' => if next_char == '=' {
                *index += 2;
                return Token { data_type: TokenType::LessThanOrEquals, data: String::new() };
            } else {
                *index += 1;
                return Token { data_type: TokenType::LessThan, data: String::new() };
            },
            // != !
            '!' => if next_char == '=' {
                *index += 2;
                return Token { data_type: TokenType::NotEquals, data: String::new() };
            } else {
                *index += 1;
                return Token { data_type: TokenType::Not, data: String::new() };
            },
            // == =
            '=' => if next_char == '=' {
                *index += 2;
                return Token { data_type: TokenType::DoubleEquals, data: String::new() };
            } else {
                *index += 1;
                return Token { data_type: TokenType::Equals, data: String::new() };
            },
            // &&
            '&' => if next_char == '&' {
                *index += 2;
                return Token { data_type: TokenType::And, data: String::new() };
            },
            // ||
            '|' => if next_char == '|' {
                *index += 2;
                return Token { data_type: TokenType::Or, data: String::new() };
            },
            // single chars
            _ => {
                let c = buffer[*index] as char;

                // block
                if c == '(' {
                    *index += 1;
                    return Token { data_type: TokenType::CircleBracketBegin, data: String::new() };
                } else
                if c == ')' {
                    *index += 1;
                    return Token { data_type: TokenType::CircleBracketEnd, data: String::new() };
                } else
                if c == '{' {
                    *index += 1;
                    return Token { data_type: TokenType::FigureBracketBegin, data: String::new() };
                } else
                if c == '}' {
                    *index += 1;
                    return Token { data_type: TokenType::FigureBracketEnd, data: String::new() };
                } else
                if c == '[' {
                    *index += 1;
                    return Token { data_type: TokenType::SquareBracketBegin, data: String::new() };
                } else
                if c == ']' {
                    *index += 1;
                    return Token { data_type: TokenType::SquareBracketEnd, data: String::new() };
                } else
                // other
                if c == ';' {
                    *index += 1;
                    return Token { data_type: TokenType::Endline, data: String::new() };
                } else
                if c == ':' {
                    *index += 1;
                    return Token { data_type: TokenType::Colon, data: String::new() };
                } else
                if c == ',' {
                    *index += 1;
                    return Token { data_type: TokenType::Comma, data: String::new() };
                } else
                if c == '.' {
                    *index += 1;
                    return Token { data_type: TokenType::Dot, data: String::new() };
                } else
                if c == '%' {
                    *index += 1;
                    return Token { data_type: TokenType::Modulo, data: String::new() };
                } else
                if c == '?' {
                    *index += 1;
                    return Token { data_type: TokenType::Question, data: String::new() };
                }
            },
        }

        *index += 1;
        Token { data_type: TokenType::None, data: String::new() }
    }

    fn output_lines(lines: &Vec<Line>, ident: usize) {
        for line in lines {
            println!("{}* Line", " ".repeat(ident+2));
            println!("{}Tokens:", " ".repeat(ident+4));
            for token in &line.tokens {
                if !token.data.is_empty() {
                    println!("{}{} [{}]", " ".repeat(ident+6), token.data_type.to_string(), token.data);
                } else {
                    println!("{}{}", " ".repeat(ident+6), token.data_type.to_string());
                }
            }
            if (&line.lines).len() > 0 {
                println!("{}Lines:", " ".repeat(ident+4));
                output_lines(&line.lines, ident+4);
            }
            println!("{}.", " ".repeat(ident+4));
        }
    }

    fn line_nesting(lines: &mut Vec<Line>) {
        let mut lines_len = lines.len();
        
        let mut i = 0;
        while i < lines_len {
            let ni = i+1;
            if ni < lines_len {
                if lines[i].ident < lines[ni].ident {
                    let next_line = lines[ni].clone(); // clone next line
                    lines[i].lines.push(next_line);    // nesting
                    lines.remove(ni);                  // delete next
                    lines_len = lines.len();           // update vec len
                    line_nesting(&mut lines[i].lines); // cycle
                } else {
                    i += 1; // next line < current line => skip
                }
            } else {
                break; // if no lines
            }
        }
    }

    pub fn read_tokens(buffer: Vec<u8>) {
        let mut lines: Vec<Line> = Vec::new();
        let mut tokens: Vec<Token> = Vec::new();
        let mut line_ident: u8 = 0;
        let mut read_line_ident: bool = true;

        let buffer_length = buffer.len();
        let mut index = 0;
        while index < buffer_length {
            let c = buffer[index] as char;

            // ident
            if c == ' ' && read_line_ident {
                line_ident += 1;
                index += 1;
            } else {
                read_line_ident = false;
                // get endline
                if c == '\n' {
                    tokens.push( Token { data_type: TokenType::Endline, data: String::new()} );

                    line_ident = if line_ident % 2 == 0 { line_ident / 2 } else { (line_ident - 1) / 2 };
                    lines.push( Line { tokens: tokens.clone(), ident: line_ident, lines: Vec::new() } );
                    line_ident = 0;

                    read_line_ident = true;
                    tokens.clear();
                    index += 1;
                } else
                // delete comment
                if c == '#' {
                    delete_comment(&mut index, &buffer, &buffer_length);
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
        }

        lines.retain(|line| {
            line.tokens.len() >= 1 && line.tokens[0].data_type != TokenType::Endline
        });

        // line nesting
        line_nesting(&mut lines);

        // output tokens
        println!("[LOG][INFO] Lines:");
        output_lines(&lines, 0);
    }
}