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
            return Token::new(TokenType::Float, result);
        }
        Token::new(TokenType::Int, result)
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
            Token::new_empty(TokenType::If)
        } else if result == "else" {
            Token::new_empty(TokenType::Else)
        } else if result == "elif" {
            Token::new_empty(TokenType::Elif)
        } else if result == "while" {
            Token::new_empty(TokenType::While)
        } else if result == "for" {
            Token::new_empty(TokenType::For)
        } else if result == "final" {
            Token::new_empty(TokenType::Final)
        } else if result == "const" {
            Token::new_empty(TokenType::Const)
        } else {
            Token::new(TokenType::Word, result)
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
        return if quote == b'\'' {
            Token::new(TokenType::SingleQuote, result.clone())
        } else if quote == b'"' {
            Token::new(TokenType::DoubleQuote, result.clone())
        } else if quote == b'`' {
            Token::new(TokenType::BackQuote, result.clone())
        } else {
            Token::new_empty(TokenType::None)
        }
    }

    fn get_operator(index: &mut usize, buffer: &[u8]) -> Token {
        let next_char = buffer[*index+1] as char;
        match buffer[*index] as char {
            // += ++ +
            '+' => if next_char == '=' {
                *index += 2;
                return Token::new_empty(TokenType::PlusEquals);
            } else if next_char == '+' {
                *index += 2;
                return Token::new_empty(TokenType::Increment);
            } else {
                *index += 1;
                return Token::new_empty(TokenType::Plus);
            },
            // -= -- -
            '-' => if next_char == '=' {
                *index += 2;
                return Token::new_empty(TokenType::MinusEquals);
            } else if next_char == '-' {
                *index += 2;
                return Token::new_empty(TokenType::Decrement);
            } else if next_char == '>' {
                *index += 2;
                return Token::new_empty(TokenType::Pointer);
            } else {
                *index += 1;
                return Token::new_empty(TokenType::Minus);
            },
            // *= *
            '*' => if next_char == '=' {
                *index += 2;
                return Token::new_empty(TokenType::MultiplyEquals);
            } else {
                *index += 1;
                return Token::new_empty(TokenType::Multiply);
            },
            // /= /
            '/' => if next_char == '=' {
                *index += 2;
                return Token::new_empty(TokenType::DivideEquals);
            } else {
                *index += 1;
                return Token::new_empty(TokenType::Divide);
            },
            // >= >
            '>' => if next_char == '=' {
                *index += 2;
                return Token::new_empty(TokenType::GreaterThanOrEquals);
            } else {
                *index += 1;
                return Token::new_empty(TokenType::GreaterThan);
            },
            // <=
            '<' => if next_char == '=' {
                *index += 2;
                return Token::new_empty(TokenType::LessThanOrEquals);
            } else {
                *index += 1;
                return Token::new_empty(TokenType::LessThan);
            },
            // != !
            '!' => if next_char == '=' {
                *index += 2;
                return Token::new_empty(TokenType::NotEquals);
            } else {
                *index += 1;
                return Token::new_empty(TokenType::Not);
            },
            // == =
            '=' => if next_char == '=' {
                *index += 2;
                return Token::new_empty(TokenType::DoubleEquals);
            } else {
                *index += 1;
                return Token::new_empty(TokenType::Equals);
            },
            // &&
            '&' => if next_char == '&' {
                *index += 2;
                return Token::new_empty(TokenType::And);
            },
            // ||
            '|' => if next_char == '|' {
                *index += 2;
                return Token::new_empty(TokenType::Or);
            },
            // single chars
            _ => {
                let c = buffer[*index] as char;

                // block
                if c == '(' {
                    *index += 1;
                    return Token::new_empty(TokenType::CircleBracketBegin);
                } else
                if c == ')' {
                    *index += 1;
                    return Token::new_empty(TokenType::CircleBracketEnd);
                } else
                if c == '{' {
                    *index += 1;
                    return Token::new_empty(TokenType::FigureBracketBegin);
                } else
                if c == '}' {
                    *index += 1;
                    return Token::new_empty(TokenType::FigureBracketEnd);
                } else
                if c == '[' {
                    *index += 1;
                    return Token::new_empty(TokenType::SquareBracketBegin);
                } else
                if c == ']' {
                    *index += 1;
                    return Token::new_empty(TokenType::SquareBracketEnd);
                } else
                // other
                if c == ';' {
                    *index += 1;
                    return Token::new_empty(TokenType::Endline);
                } else
                if c == ':' {
                    *index += 1;
                    return Token::new_empty(TokenType::Colon);
                } else
                if c == ',' {
                    *index += 1;
                    return Token::new_empty(TokenType::Comma);
                } else
                if c == '.' {
                    *index += 1;
                    return Token::new_empty(TokenType::Dot);
                } else
                if c == '%' {
                    *index += 1;
                    return Token::new_empty(TokenType::Modulo);
                } else
                if c == '?' {
                    *index += 1;
                    return Token::new_empty(TokenType::Question);
                }
            },
        }

        *index += 1;
        Token::new(TokenType::None, String::new())
    }

    fn bracket_nesting(tokens: &mut Vec<Token>, begin_type: TokenType, end_type: TokenType) {
        for token in tokens.iter_mut() {
            if token.tokens.len() > 0 {
                bracket_nesting(&mut token.tokens, begin_type.clone(), end_type.clone());
            }
        }
        block_nesting(tokens, begin_type.clone(), end_type.clone());
    }
    fn block_nesting(tokens: &mut Vec<Token>, begin_type: TokenType, end_type: TokenType) {
        let mut brackets = Vec::<usize>::new();

        let mut i = 0;
        while i < tokens.len() {
            let token = tokens[i].clone();
            // begin
            if token.data_type == begin_type {
                brackets.push(i);
            // end
            } else if token.data_type == end_type {
                if let Some(penult_bracket) = brackets.len().checked_sub(1) {
                    if penult_bracket > 0 {
                        if let Some(last_bracket) = brackets.last().cloned() {
                            let copy_token = tokens[last_bracket].clone();
                            tokens[penult_bracket].tokens.push( copy_token );

                            tokens.remove(last_bracket);
                            i -= 1;
                        }
                    }
                }
                brackets.pop();
                tokens.remove(i);
                i -= 1;
            // add new childrens to token
            } else if !brackets.is_empty() {
                if let Some(bracket) = brackets.last().cloned() {
                    tokens[bracket].tokens.push(
                        Token::new_full(token.data_type, token.data, token.tokens)
                    );
                }
                tokens.remove(i);
                i -= 1;
            }
            i += 1;
        }
    }

    fn line_nesting(lines: &mut Vec<Line>) {
        let mut lines_len = lines.len();
        let mut i: usize = 0;
        while i < lines_len {
            let ni: usize = i+1;
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

    fn output_tokens(tokens: &Vec<Token>, line_ident: usize, ident: usize) {
        for token in tokens {
            if !token.data.is_empty() {
                println!("{}[{}] [{}]", " ".repeat(line_ident+ident+6), token.data, token.data_type.to_string());
            } else {
                println!("{}{}", " ".repeat(line_ident+ident+6), token.data_type.to_string());
            }
            if (&token.tokens).len() > 0 {
                output_tokens(&token.tokens, line_ident, ident+2)
            }
        }
    }
    fn output_lines(lines: &Vec<Line>, ident: usize) {
        for line in lines {
            println!("{}* Line", " ".repeat(ident+2));
            println!("{}Tokens:", " ".repeat(ident+4));
            output_tokens(&line.tokens, ident, 0);
            if (&line.lines).len() > 0 {
                println!("{}Lines:", " ".repeat(ident+4));
                output_lines(&line.lines, ident+4);
            }
            println!("{}.", " ".repeat(ident+2));
        }
    }

    pub fn read_tokens(buffer: Vec<u8>) {
        let mut lines: Vec<Line> = Vec::new();
        let mut tokens: Vec<Token> = Vec::new();
        let mut line_ident: usize = 0;
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
                    tokens.push( Token::new_empty(TokenType::Endline) );
                    // bracket nesting
                    bracket_nesting(&mut tokens, TokenType::CircleBracketBegin, TokenType::CircleBracketEnd);
                    bracket_nesting(&mut tokens, TokenType::SquareBracketBegin, TokenType::SquareBracketEnd);
                    bracket_nesting(&mut tokens, TokenType::FigureBracketBegin, TokenType::FigureBracketEnd);

                    // add new line
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
                // get int/float
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