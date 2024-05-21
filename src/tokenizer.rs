/*
    tokenizer
*/

use crate::logger::*;
use crate::filePath;

pub mod token;
pub mod line;

use crate::tokenizer::token::*;
use crate::tokenizer::line::*;

unsafe fn deleteComment(buffer: &[u8]) {
    if buffer[index] != b'#' {
        return;
    }
    index += 1; // skip first #
    if buffer[index] == b'#' {
        linesCount += 1;
        // double comment
        index += 1;
        while index < bufferLength {
            index += 1;
            if buffer[index] == b'\n' {
                linesCount += 1;
            } else
            if buffer[index] == b'#' && index+1 < bufferLength && buffer[index+1] == b'#' {
                if index+2 < bufferLength && buffer[index+2] == b'\n' {
                    // skip ##\n
                    index += 3;
                } else {
                    // skip ##text\n
                    index += 2;
                }
                linesCount += 1;
                linesDeleted += 1;
                return;
            }
        }
    } else {
        // single comment
        index += 1;
        while index < bufferLength {
            index += 1;
            if buffer[index] == b'\n' {
                linesDeleted += 1;
                return;
            }
        }
    }
}
// get single char token
fn getSingleChar(c: char) -> bool {
    // math
    c == '+' || c == '-' || c == '*' || c == '/' || c == '=' || c == '%' ||
    // logical
    c == '>' || c == '<' || c == '?' || c == '!' || c == '&' || c == '|' ||
    // bracket
    c == '(' || c == ')' ||
    c == '{' || c == '}' ||
    c == '[' || c == ']' ||
    // other
    c == ':' ||
    c == ';' ||
    c == ',' ||
    c == '.' ||
    c == '~'
}
// get int-float token by buffer-index
unsafe fn getNumber(buffer: &[u8]) -> Token {
    let mut indexBuffer: usize = index;
    let mut result = String::new();

    let mut dotCheck:      bool = false;
    let mut negativeCheck: bool = false;
    let mut rationalCheck: bool = false;
    while indexBuffer < bufferLength {
        let currentChar: char = buffer[indexBuffer] as char;
        let nextChar: char = 
            if indexBuffer+1 < bufferLength {
                buffer[indexBuffer+1] as char
            } else {
                '\0'
            };

        if currentChar == '-' && !negativeCheck {
            negativeCheck = true;
            indexBuffer += 1;
        } else
        if currentChar.is_digit(10) {
            result.push(currentChar);
            indexBuffer += 1;
        } else 
        if currentChar == '.' && !dotCheck && nextChar.is_digit(10) {
            if rationalCheck { // Rational number use only Int/Int
                break;
            }
            dotCheck = true;
            result.push(currentChar);
            indexBuffer += 1;
        } else
        if currentChar == '/' && nextChar == '/' && !dotCheck && 
           (indexBuffer+2 < bufferLength && (buffer[indexBuffer+2] as char).is_digit(10)) {
            rationalCheck = true;
            result.push('/');
            result.push('/');
            indexBuffer += 2;
        } else {
            break;
        }
    }

    if !result.is_empty() {
        index = indexBuffer;
        indexCount += result.len();
    }
    // rational
    if rationalCheck {
        return Token::new(TokenType::Rational, result);
    } else
    // float
    if dotCheck {
        if negativeCheck {
            return Token::new(TokenType::Float, result);
        } else {
            return Token::new(TokenType::UFloat, result);
        }
    } else {
    // integer
        if negativeCheck {
            return Token::new(TokenType::Int, result);
        } else {
            return Token::new(TokenType::UInt, result);
        }
    }
}
// get word token by buffer-index
unsafe fn getWord(buffer: &[u8]) -> Token {
    let mut indexBuffer: usize = index;
    let mut result = String::new();

    while indexBuffer < bufferLength {
        let currentChar: char = buffer[indexBuffer] as char;
        let nextChar: char = 
            if indexBuffer+1 < bufferLength {
                buffer[indexBuffer+1] as char
            } else {
                '\0'
            };

        if currentChar.is_alphanumeric() || (currentChar == '_' && !result.is_empty() && nextChar.is_alphanumeric()) {
            result.push(currentChar);
            indexBuffer += 1;
        } else {
            break;
        }
    }

    if !result.is_empty() {
        index = indexBuffer;
        indexCount += result.len();
    }
    //
    return if result == "Int" {
        Token::newEmpty(TokenType::Int)
    } else if result == "UInt" {
        Token::newEmpty(TokenType::UInt)
    } else if result == "Float" {
        Token::newEmpty(TokenType::Float)
    } else if result == "UFloat" {
        Token::newEmpty(TokenType::UFloat)
    } else if result == "Rational" {
        Token::newEmpty(TokenType::Rational)
        // todo: complex number
        // and other types
    //
    } else if result == "and" {
        Token::newEmpty(TokenType::And)
    } else if result == "or" {
        Token::newEmpty(TokenType::Or)
    //
    } else if result == "loop" {
        Token::newEmpty(TokenType::Loop)
    //
    } else {
        Token::new(TokenType::Word, result)
    }
}
// get quotes token by buffer-index
unsafe fn getQuotes(buffer: &[u8]) -> Token {
    let quote: u8 = buffer[index];
    let inputLength: usize = buffer.len();
    let mut result = String::new();
    if buffer[index] == quote {

        let mut open: bool = false;
        while index < inputLength {
            let currentChar: u8 = buffer[index];
            // check endline error
            if currentChar == b'\n' {
                log("syntax","");
                log("path",&format!(
                    "{}:{}:{}", 
                    filePath,
                    linesCount,
                    indexCount
                ));
                log("note","Quotes were not closed!");
                logExit();
            }
            // read quote
            if currentChar != quote {
                result.push(currentChar as char);
            } else
            if currentChar == quote {
                let mut noSlash: bool = true;
                // check back slash of end quote
                if buffer[index-1] == b'\\' {
                    let mut backslashCounter: usize = 1;
                    for i in (0..index-1).rev() {
                        if buffer[i] == b'\\' {
                            backslashCounter += 1;
                        } else {
                            break;
                        }
                    }
                    if backslashCounter % 2 == 1 {
                        // add slash (\' \" \`)
                        result.push(currentChar as char);
                        noSlash = false;
                    }
                }
                //
                if open && noSlash {
                    index += 1;
                    indexCount += 1;
                    break;
                } else {
                    open = true;
                }
            }
            index += 1;
            indexCount += 1;
        }
        /*
        if !open {
            log("syntax","");
            log("path",&format!(
                "{}:{}:{}", 
                filePath,
                linesCount,
                indexCount
            ));
            log("note","Quotes were not closed at the end!");
            logExit();
        }
        */
    }
    return if quote == b'\'' {
        return if result.len() > 1 {
            log("syntax","");
            log("path",&format!(
                "{}:{}:{}", 
                filePath,
                linesCount,
                indexCount
            ));
            log("note","Single quotes can only contain 1 character!");
            logExit();
            std::process::exit(1)
        } else {
            Token::new(TokenType::SingleQuote, result.clone())
        }
    } else if quote == b'"' {
        Token::new(TokenType::DoubleQuote, result.clone())
    } else if quote == b'`' {
        Token::new(TokenType::BackQuote, result.clone())
    } else {
        Token::newEmpty(TokenType::None)
    }
}
// get operator token by buffer-index
unsafe fn getOperator(buffer: &[u8]) -> Token {
    let nextChar: char = buffer[index+1] as char;
    match buffer[index] as char {
        // += ++ +
        '+' => if nextChar == '=' {
            index += 2;
            indexCount += 2;
            return Token::newEmpty(TokenType::PlusEquals);
        } else if nextChar == '+' {
            index += 2;
            indexCount += 2;
            return Token::newEmpty(TokenType::Increment);
        } else {
            index += 1;
            indexCount += 1;
            return Token::newEmpty(TokenType::Plus);
        },
        // -= -- -
        '-' => if nextChar == '=' {
            index += 2;
            indexCount += 2;
            return Token::newEmpty(TokenType::MinusEquals);
        } else if nextChar == '-' {
            index += 2;
            indexCount += 2;
            return Token::newEmpty(TokenType::Decrement);
        } else if nextChar == '>' {
            index += 2;
            indexCount += 2;
            return Token::newEmpty(TokenType::Pointer);
        } else {
            index += 1;
            indexCount += 1;
            return Token::newEmpty(TokenType::Minus);
        },
        // *= *
        '*' => if nextChar == '=' {
            index += 2;
            indexCount += 2;
            return Token::newEmpty(TokenType::MultiplyEquals);
        } else {
            index += 1;
            indexCount += 1;
            return Token::newEmpty(TokenType::Multiply);
        },
        // /= /
        '/' => if nextChar == '=' {
            index += 2;
            indexCount += 2;
            return Token::newEmpty(TokenType::DivideEquals);
        } else {
            index += 1;
            indexCount += 1;
            return Token::newEmpty(TokenType::Divide);
        },
        // >= >
        '>' => if nextChar == '=' {
            index += 2;
            indexCount += 2;
            return Token::newEmpty(TokenType::GreaterThanOrEquals);
        } else {
            index += 1;
            indexCount += 1;
            return Token::newEmpty(TokenType::GreaterThan);
        },
        // <=
        '<' => if nextChar == '=' {
            index += 2;
            indexCount += 2;
            return Token::newEmpty(TokenType::LessThanOrEquals);
        } else {
            index += 1;
            indexCount += 1;
            return Token::newEmpty(TokenType::LessThan);
        },
        // != !
        '!' => if nextChar == '=' {
            index += 2;
            indexCount += 2;
            return Token::newEmpty(TokenType::NotEquals);
        } else {
            index += 1;
            indexCount += 1;
            return Token::newEmpty(TokenType::Not);
        },
        // == =
        '=' => {
            index += 1;
            indexCount += 1;
            return Token::newEmpty(TokenType::Equals);
        },
        // &&
        '&' => if nextChar == '&' {
            index += 2;
            indexCount += 2;
            return Token::newEmpty(TokenType::And);
        },
        // ||
        '|' => if nextChar == '|' {
            index += 2;
            indexCount += 2;
            return Token::newEmpty(TokenType::Or);
        },
        // single chars
        _ => {
            let c: char = buffer[index] as char;

            // block
            if c == '(' {
                index += 1;
                indexCount += 1;
                return Token::newEmpty(TokenType::CircleBracketBegin);
            } else
            if c == ')' {
                index += 1;
                indexCount += 1;
                return Token::newEmpty(TokenType::CircleBracketEnd);
            } else
            if c == '{' {
                index += 1;
                indexCount += 1;
                return Token::newEmpty(TokenType::FigureBracketBegin);
            } else
            if c == '}' {
                index += 1;
                indexCount += 1;
                return Token::newEmpty(TokenType::FigureBracketEnd);
            } else
            if c == '[' {
                index += 1;
                indexCount += 1;
                return Token::newEmpty(TokenType::SquareBracketBegin);
            } else
            if c == ']' {
                index += 1;
                indexCount += 1;
                return Token::newEmpty(TokenType::SquareBracketEnd);
            } else
            // other
            if c == ';' {
                index += 1;
                indexCount += 1;
                return Token::newEmpty(TokenType::Endline);
            } else
            if c == ':' {
                index += 1;
                indexCount += 1;
                return Token::newEmpty(TokenType::Colon);
            } else
            if c == ',' {
                index += 1;
                indexCount += 1;
                return Token::newEmpty(TokenType::Comma);
            } else
            if c == '.' {
                index += 1;
                indexCount += 1;
                return Token::newEmpty(TokenType::Dot);
            } else
            if c == '%' {
                index += 1;
                indexCount += 1;
                return Token::newEmpty(TokenType::Modulo);
            } else
            if c == '?' {
                index += 1;
                indexCount += 1;
                return Token::newEmpty(TokenType::Question);
            } else
            if c == '~' {
                index += 1;
                indexCount += 1;
                return Token::newEmpty(TokenType::Tilde);
            }
        },
    }

    index += 1;
    indexCount += 1;
    Token::new(TokenType::None, String::new())
}

// bracket nasting [b bracket -> e bracket]
//             [+ recall in token child tokens]
// 1 () no tokens childrens -> 
// 2 [] tokens childrens 1  ->
// 3 {} tokens childres 1+2
fn bracketNesting(tokens: &mut Vec<Token>, beginType: TokenType, endType: TokenType) {
    for token in tokens.iter_mut() {
        if token.tokens.len() > 0 {
            bracketNesting(&mut token.tokens, beginType.clone(), endType.clone());
        }
    }
    blockNesting(tokens, beginType.clone(), endType.clone());
}
// block nasting [b token -> e token]
fn blockNesting(tokens: &mut Vec<Token>, beginType: TokenType, endType: TokenType) {
    let mut brackets = Vec::<usize>::new();
    let mut i: usize = 0;

    while i < tokens.len() {
        let token = tokens[i].clone();
        // begin
        if token.dataType == beginType {
            brackets.push(i);
        // end
        } else if token.dataType == endType {
            if let Some(penultBracket) = brackets.pop() {
                if let Some(&lastBracket) = brackets.last() {
                    let copyToken = tokens[penultBracket].clone();
                    tokens[lastBracket].tokens.push(copyToken);
                    tokens.remove(penultBracket);
                    i -= 1;
                }
            }
            tokens.remove(i);
            continue;
        // add new childrens to token
        } else if !brackets.is_empty() {
            if let Some(&bracket) = brackets.last() {
                tokens[bracket].tokens.push(
                    Token::newFull(token.dataType, token.data, token.tokens)
                );
            }
            tokens.remove(i);
            continue;
        }
        i += 1;
    }
}
// line nesting [line -> line]
fn lineNesting(lines: &mut Vec<Line>) {
    let mut lines_len: usize = lines.len();
    let mut i: usize = 0;
    while i < lines_len {
        let ni: usize = i+1;
        if ni < lines_len {
            if lines[i].ident < lines[ni].ident {
                let next_line = lines[ni].clone(); // clone next line
                lines[i].lines.push(next_line);    // nesting
                lines.remove(ni);                  // delete next
                lines_len = lines.len();           // update vec len
                lineNesting(&mut lines[i].lines);  // cycle
            } else {
                i += 1; // next line < current line => skip
            }
        } else {
            break; // if no lines
        }
    }
}

// output token and its tokens
pub fn outputTokens(tokens: &Vec<Token>, lineIdent: usize, ident: usize) {
    let identStr: String = " ".repeat((lineIdent+ident)*2);
    for token in tokens {
        if !token.data.is_empty() {
            // single quote
            if token.dataType == TokenType::SingleQuote {
                log("parserToken",&format!(
                    "{}'{}'  |{}",
                    identStr,
                    token.data,
                    token.dataType.to_string()
                ));
            // double quote
            } else
            if token.dataType == TokenType::DoubleQuote {
                log("parserToken",&format!(
                    "{}\"{}\"  |{}",
                    identStr,
                    token.data,
                    token.dataType.to_string()
                ));
            // back quote
            } else
            if token.dataType == TokenType::BackQuote {
                log("parserToken",&format!(
                    "{}`{}`  |{}",
                    identStr,
                    token.data,
                    token.dataType.to_string()
                ));
            // basic
            } else {
                log("parserToken",&format!(
                    "{}{}  |{}",
                    identStr,
                    token.data,
                    token.dataType.to_string()
                ));
            }
        } else {
            println!("{}{}", identStr, token.dataType.to_string());
        }
        if (&token.tokens).len() > 0 {
            outputTokens(&token.tokens, lineIdent, ident+1)
        }
    }
}
// output line info
pub fn outputLines(lines: &Vec<Line>, ident: usize) {
    let identStr1: String = " ".repeat((ident)*2);
    let identStr2: String = " ".repeat((ident)*2+2);
    for (i, line) in lines.iter().enumerate() {
        log("parserBegin", &format!("{}+{}", identStr1, i));
        log("parserHeader", &format!("{}Tokens", identStr2));
        outputTokens(&line.tokens, ident+1, 1);
        if (&line.lines).len() > 0 {
            log("parserHeader", &format!("{}Lines", identStr2));
            outputLines(&line.lines, ident+1);
        }
        log("parserEnd", &format!("{}-{}", identStr1, i));
    }
}

// tokens reader cycle
static mut index:        usize = 0; // it is more profitable to contain the value here,
static mut bufferLength: usize = 0; // than to shove it into methods every time.
                                    // bufferLength would be better if it were final, 
                                    // but it is not :( and i like unsafe!
static mut linesCount:   usize = 1; // even if these variables are not used,
static mut indexCount:   usize = 0; // their use is better than a vector of strings
static mut linesDeleted: usize = 0; // <- save deleted lines num for logger
pub unsafe fn readTokens(buffer: Vec<u8>) -> Vec<Line> {
    let mut lines:  Vec<Line>  = Vec::new();
    let mut tokens: Vec<Token> = Vec::new();
    let mut lineIdent:     usize = 0;
    let mut readLineIdent: bool  = true;

    bufferLength = buffer.len();
    while index < bufferLength {
        let c: char = buffer[index] as char;

        // ident
        if c == ' ' && readLineIdent {
            index += 1;

            lineIdent += 1;
            indexCount += 1;
        } else {
            readLineIdent = false;
            // get endline
            if c == '\n' {
                // bracket nesting
                bracketNesting(&mut tokens, TokenType::CircleBracketBegin, TokenType::CircleBracketEnd);
                bracketNesting(&mut tokens, TokenType::SquareBracketBegin, TokenType::SquareBracketEnd);
                bracketNesting(&mut tokens, TokenType::FigureBracketBegin, TokenType::FigureBracketEnd);

                // add new line
                lineIdent = if lineIdent % 2 == 0 { lineIdent / 2 } else { (lineIdent - 1) / 2 };
                lines.push( Line {
                    tokens:       tokens.clone(),
                    ident:        lineIdent,
                    lines:        Vec::new(),
                    linesDeleted: linesDeleted+linesCount
                } );
                linesDeleted = 0;
                lineIdent = 0;

                readLineIdent = true;
                tokens.clear();
                index += 1;

                linesCount += 1;
                indexCount = 0;
            } else
            // delete comment
            if c == '#' {
                deleteComment(&buffer);
            } else
            // get int-float
            if c.is_digit(10) || (c == '-' && index+1 < bufferLength && (buffer[index+1] as char).is_digit(10)) {
                tokens.push( getNumber(&buffer) );
            } else
            // get word
            if c.is_alphabetic() {
                tokens.push( getWord(&buffer) );
            } else
            // get quotes ' " `
            if c == '\'' || c == '"' || c == '`' {
                let token: Token = getQuotes(&buffer);
                if token.dataType != TokenType::None {
                    tokens.push(token);
                } else {
                    index += 1;
                    indexCount += 1;
                }
            } else
            // get single and double chars
            if getSingleChar(c) {
                let token: Token = getOperator(&buffer);
                if token.dataType != TokenType::None {
                    tokens.push(token);
                } else {
                    index += 1;
                    indexCount += 1;
                }
                // skip
            } else {
                index += 1;
                indexCount += 1;
            }
        }
    }

    // delete empty lines
    lines.retain(|line| {
        line.tokens.len() >= 1 && line.tokens[0].dataType != TokenType::Endline
    });
    // line nesting
    lineNesting(&mut lines);
    //
    lines
}