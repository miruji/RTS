/*
    tokenizer
*/

use crate::logger::*;
use crate::_filePath;
use crate::_debugMode;

pub mod token; use crate::tokenizer::token::*;
pub mod line;  use crate::tokenizer::line::*;

unsafe fn deleteComment(buffer: &[u8]) {
    _index += 1;
    _indexCount += 2;
    
    _lineTokens.push( Token::newEmpty(TokenType::Comment) );
    while _index < _bufferLength && buffer[_index] != b'\n' {
        _index += 1;
        _indexCount += 1;
    }
}
// get single char token
fn getSingleChar(c: u8) -> bool {
    match c {
        b'+' | b'-' | b'*' | b'/' | b'=' | b'%' | b'^' |
        b'>' | b'<' | b'?' | b'!' | b'&' | b'|' | 
        b'(' | b')' | b'{' | b'}' | b'[' | b']' | 
        b':' | b',' | b'.' | b'~' => true,
        _ => false,
    }
}
// get int-float token by buffer-index
fn isDigit(c: u8) -> bool {
    c >= b'0' && c <= b'9'
}
unsafe fn getNumber(buffer: &[u8]) -> Token {
    let mut indexBuffer: usize = _index;
    let mut result = String::new();

    let mut dotCheck:      bool = false;
    let mut negativeCheck: bool = false;
    let mut rationalCheck: bool = false;

    let mut currentChar: u8;
    let mut nextChar:    u8;

    while indexBuffer < _bufferLength {
        currentChar = buffer[indexBuffer];
        nextChar = 
            if indexBuffer+1 < _bufferLength {
                buffer[indexBuffer+1]
            } else {
                b'\0'
            };

        if !negativeCheck && buffer[_index] == b'-' {
            result.push(currentChar as char);
            negativeCheck = true;
            indexBuffer += 1;
        } else
        if isDigit(currentChar) {
            result.push(currentChar as char);
            indexBuffer += 1;
        } else 
        if currentChar == b'.' && !dotCheck && isDigit(nextChar) {
            if rationalCheck { // Rational number use only Int/Int
                break;
            }
            dotCheck = true;
            result.push(currentChar as char);
            indexBuffer += 1;
        } else
        if currentChar == b'/' && nextChar == b'/' && !dotCheck && 
           (indexBuffer+2 < _bufferLength && isDigit(buffer[indexBuffer+2])) {
            rationalCheck = true;
            result.push('/');
            result.push('/');
            indexBuffer += 2;
        } else {
            break;
        }
    }

    if !result.is_empty() {
        _index = indexBuffer;
        _indexCount += result.len();
    }

    match (rationalCheck, dotCheck, negativeCheck) {
        (true, _, _)     => Token::new(TokenType::Rational, result),
        (_, true, true)  => Token::new(TokenType::Float,    result),
        (_, true, false) => Token::new(TokenType::UFloat,   result),
        (_, false, true) => Token::new(TokenType::Int,      result),
        _                => Token::new(TokenType::UInt,     result),
    }
}
// get word token by buffer-index
fn isLetter(c: u8) -> bool {
    (c >= b'a' && c <= b'z') ||
    (c >= b'A' && c <= b'Z')
}
unsafe fn getWord(buffer: &[u8]) -> Token {
    let mut indexBuffer: usize = _index;
    let mut result = String::new();

    let mut currentChar: u8;
    let mut nextChar:    u8;
    while indexBuffer < _bufferLength {
        currentChar = buffer[indexBuffer];
        nextChar = 
            if indexBuffer+1 < _bufferLength {
                buffer[indexBuffer+1]
            } else {
                b'\0'
            };

        if isLetter(currentChar) || (currentChar == b'-' && !result.is_empty() && isLetter(nextChar)) {
            result.push(currentChar as char);
            indexBuffer += 1;
        } else {
            break;
        }
    }

    if !result.is_empty() {
        _index = indexBuffer;
        _indexCount += result.len();
    }

    match &result[..] {
        "Int"      => Token::newEmpty(TokenType::Int),
        "UInt"     => Token::newEmpty(TokenType::UInt),
        "Float"    => Token::newEmpty(TokenType::Float),
        "UFloat"   => Token::newEmpty(TokenType::UFloat),
        "Rational" => Token::newEmpty(TokenType::Rational),
        "and"      => Token::newEmpty(TokenType::And),
        "or"       => Token::newEmpty(TokenType::Or),
        "loop"     => Token::newEmpty(TokenType::Loop),
        _          => Token::new(TokenType::Word, result),
    }
}
// get quotes token by buffer-index
// todo: fix quotes
unsafe fn getQuotes(buffer: &[u8]) -> Token {
    let quote: u8 = buffer[_index];

    let inputLength: usize = buffer.len();
    let mut result = String::new();

    if buffer[_index] == quote {
        let mut open:        bool = false;
        let mut currentChar: u8;
        let mut noSlash:          bool;
        let mut backslashCounter: usize;

        while _index < inputLength {
            currentChar = buffer[_index];
            // check endline error
            if currentChar == b'\n' {
                /*
                log("syntax","");
                log("path",&format!(
                    "{}:{}:{}", 
                    _filePath,
                    _linesCount,
                    _indexCount
                ));
                log("note","Quotes were not closed!");
                logExit();
                */
                return Token::newEmpty(TokenType::None);
            }
            // read quote
            if currentChar != quote {
                result.push(currentChar as char);
            } else
            if currentChar == quote {
                noSlash = true;
                // check back slash of end quote
                if buffer[_index-1] == b'\\' {
                    backslashCounter = 1;
                    for i in (0.._index-1).rev() {
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
                    _index += 1;
                    _indexCount += 1;
                    break;
                } else {
                    open = true;
                }
            }
            _index += 1;
            _indexCount += 1;
        }
    }
    return if quote == b'\'' {
        return if result.len() > 1 {
            log("syntax","");
            log("path",&format!(
                "{}:{}:{}", 
                _filePath,
                _linesCount,
                _indexCount
            ));
            log("note","Single quotes can only contain 1 character!");
            logExit();
            std::process::exit(1)
        } else {
            Token::new(TokenType::Char, result.clone())
        }
    } else if quote == b'"' {
        Token::new(TokenType::String, result.clone())
    } else if quote == b'`' {
        Token::new(TokenType::SpecialString, result.clone())
    } else {
        Token::newEmpty(TokenType::None)
    }
}
// get operator token by buffer-index
unsafe fn getOperator(buffer: &[u8]) -> Token {
    let nextChar: u8 = 
        if _index+1 < _bufferLength {
            buffer[_index+1]
        } else {
            b'\0'
        };
    return match buffer[_index] {
        // += ++ +
        b'+' => if nextChar == b'=' {
            _index += 2;
            _indexCount += 2;
            Token::newEmpty(TokenType::PlusEquals)
        } else if nextChar == b'+' {
            _index += 2;
            _indexCount += 2;
            Token::newEmpty(TokenType::UnaryPlus)
        } else {
            _index += 1;
            _indexCount += 1;
            Token::newEmpty(TokenType::Plus)
        },
        // -= -- -
        b'-' => if nextChar == b'=' {
            _index += 2;
            _indexCount += 2;
            Token::newEmpty(TokenType::MinusEquals)
        } else if nextChar == b'-' {
            _index += 2;
            _indexCount += 2;
            Token::newEmpty(TokenType::UnaryMinus)
        } else if nextChar == b'>' {
            _index += 2;
            _indexCount += 2;
            Token::newEmpty(TokenType::Pointer)
        } else {
            _index += 1;
            _indexCount += 1;
            Token::newEmpty(TokenType::Minus)
        },
        // *= *
        b'*' => if nextChar == b'=' {
            _index += 2;
            _indexCount += 2;
            Token::newEmpty(TokenType::MultiplyEquals)
        } else if nextChar == b'*' {
            _index += 2;
            _indexCount += 2;
            Token::newEmpty(TokenType::UnaryMultiply)
        } else {
            _index += 1;
            _indexCount += 1;
            Token::newEmpty(TokenType::Multiply)
        },
        // /= /
        b'/' => if nextChar == b'=' {
            _index += 2;
            _indexCount += 2;
            Token::newEmpty(TokenType::DivideEquals)
        } else if nextChar == b'/' {
            _index += 2;
            _indexCount += 2;
            Token::newEmpty(TokenType::UnaryDivide)
        } else {
            _index += 1;
            _indexCount += 1;
            Token::newEmpty(TokenType::Divide)
        },
        // >= >
        b'>' => if nextChar == b'=' {
            _index += 2;
            _indexCount += 2;
            Token::newEmpty(TokenType::GreaterThanOrEquals)
        } else {
            _index += 1;
            _indexCount += 1;
            Token::newEmpty(TokenType::GreaterThan)
        },
        // <=
        b'<' => if nextChar == b'=' {
            _index += 2;
            _indexCount += 2;
            Token::newEmpty(TokenType::LessThanOrEquals)
        } else {
            _index += 1;
            _indexCount += 1;
            Token::newEmpty(TokenType::LessThan)
        },
        // != !
        b'!' => if nextChar == b'=' {
            _index += 2;
            _indexCount += 2;
            Token::newEmpty(TokenType::NotEquals)
        } else {
            _index += 1;
            _indexCount += 1;
            Token::newEmpty(TokenType::Not)
        },
        // &&
        b'&' => if nextChar == b'&' {
            _index += 2;
            _indexCount += 2;
            Token::newEmpty(TokenType::And)
        } else {
            _index += 1;
            _indexCount += 1;
            Token::newEmpty(TokenType::And) // todo: single and
        },
        // ||
        b'|' => if nextChar == b'|' {
            _index += 2;
            _indexCount += 2;
            Token::newEmpty(TokenType::Or)
        } else {
            _index += 1;
            _indexCount += 1;
            Token::newEmpty(TokenType::Or) // todo: single or
        },
        // single chars
            // =
            b'=' => {
                _index += 1;
                _indexCount += 1;
                Token::newEmpty(TokenType::Equals)
            },
            // block
            b'(' => {
                _index += 1;
                _indexCount += 1;
                Token::newEmpty(TokenType::CircleBracketBegin)
            },
            b')' => {
                _index += 1;
                _indexCount += 1;
                Token::newEmpty(TokenType::CircleBracketEnd)
            },
            b'{' => {
                _index += 1;
                _indexCount += 1;
                Token::newEmpty(TokenType::FigureBracketBegin)
            },
            b'}' => {
                _index += 1;
                _indexCount += 1;
                Token::newEmpty(TokenType::FigureBracketEnd)
            },
            b'[' => {
                _index += 1;
                _indexCount += 1;
                Token::newEmpty(TokenType::SquareBracketBegin)
            },
            b']' => {
                _index += 1;
                _indexCount += 1;
                Token::newEmpty(TokenType::SquareBracketEnd)
            },
            // other
            b';' => {
                _index += 1;
                _indexCount += 1;
                Token::newEmpty(TokenType::Endline)
            },
            b':' => {
                _index += 1;
                _indexCount += 1;
                Token::newEmpty(TokenType::Colon)
            },
            b',' => {
                _index += 1;
                _indexCount += 1;
                Token::newEmpty(TokenType::Comma)
            },
            b'.' => {
                _index += 1;
                _indexCount += 1;
                Token::newEmpty(TokenType::Dot)
            },
            b'%' => {
                _index += 1;
                _indexCount += 1;
                Token::newEmpty(TokenType::Modulo)
            },
            b'^' => {
                _index += 1;
                _indexCount += 1;
                Token::newEmpty(TokenType::Exponent)
            },
            b'?' => {
                _index += 1;
                _indexCount += 1;
                Token::newEmpty(TokenType::Question)
            },
            b'~' => {
                _index += 1;
                _indexCount += 1;
                Token::newEmpty(TokenType::Tilde)
            },
            _ => {
                _index += 1;
                _indexCount += 1;
                Token::new(TokenType::None, String::new())
            }
    }
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

    let mut token: Token;
    while i < tokens.len() {
        token = tokens[i].clone();
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
    let mut linesLen: usize = lines.len();
    let mut i: usize = 0;

    let mut ni:       usize;
    let mut nextLine: Line;
    while i < linesLen {
        ni = i+1;
        if ni < linesLen {
            if lines[i].ident < lines[ni].ident {
                nextLine = lines[ni].clone();     // clone next line
                lines[i].lines.push(nextLine);    // nesting
                lines.remove(ni);                 // delete next
                linesLen -= 1;                    // update vec len
                lineNesting(&mut lines[i].lines); // cycle
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
    let lineIdentString: String = " ".repeat(lineIdent*2+1);
    let identString:     String = " ".repeat(ident*2+1);

    let tokenCount = tokens.len();
    for (i, token) in tokens.iter().enumerate() {
        let isLast = i == tokenCount - 1;
        let treeChar = 
            if isLast {
                'X'
            } else {
                '┃'
            };

        if !token.data.is_empty() {
        // single quote
            if token.dataType == TokenType::Char {
                log("parserToken",&format!(
                    "{}{}{}\\fg(#f0f8ff)\\b'\\c{}\\fg(#f0f8ff)\\b'\\c  |{}",
                    lineIdentString,
                    treeChar,
                    identString,
                    token.data,
                    token.dataType.to_string()
                ));
        // double quote
            } else
            if token.dataType == TokenType::String {
                log("parserToken",&format!(
                    "{}{}{}\\fg(#f0f8ff)\\b\"\\c{}\\fg(#f0f8ff)\\b\"\\c  |{}",
                    lineIdentString,
                    treeChar,
                    identString,
                    token.data,
                    token.dataType.to_string()
                ));
        // back quote
            } else
            if token.dataType == TokenType::SpecialString {
                log("parserToken",&format!(
                    "{}{}{}\\fg(#f0f8ff)\\b`\\c{}\\fg(#f0f8ff)\\b`\\c  |{}",
                    lineIdentString,
                    treeChar,
                    identString,
                    token.data,
                    token.dataType.to_string()
                ));
        // basic
            } else {
                log("parserToken",&format!(
                    "{}{}{}{}  |{}",
                    lineIdentString,
                    treeChar,
                    identString,
                    token.data,
                    token.dataType.to_string()
                ));
            }
        // type only
        } else {
            println!(
                "{}{}{}{}",
                lineIdentString,
                treeChar,
                identString,
                token.dataType.to_string()
            );
        }
        if (&token.tokens).len() > 0 {
            outputTokens(&token.tokens, lineIdent, ident+1)
        }
    }
}
// output line info
pub fn outputLines(lines: &Vec<Line>, ident: usize) {
    let identStr1: String = " ".repeat((ident)*2);
    let identStr2: String = " ".repeat((ident)*2+1);
    for (i, line) in lines.iter().enumerate() {
        log("parserBegin", &format!("{}+{}",identStr1,i));

        if (&line.tokens).len() == 0 {
            log("parserHeader", &format!("{}┗ Separator",identStr2));
        } else {
            log("parserHeader", &format!("{}┣ Tokens",identStr2));
        }

        outputTokens(&line.tokens, ident, 1);
        if (&line.lines).len() > 0 {
            log("parserHeader", &format!("{}┗ Lines",identStr2));
            outputLines(&line.lines, ident+1);
        }
        //log("parserEnd", &format!("{}-{}",identStr1,i));
    }
}

// tokens reader cycle
static mut _index:        usize = 0; // it is more profitable to contain the value here,
static mut _bufferLength: usize = 0; // than to shove it into methods every time.
                                     // bufferLength would be better if it were final, 
                                     // but it is not :( and i like unsafe!
static mut _linesCount:   usize = 1; // even if these variables are not used,
static mut _indexCount:   usize = 0; // their use is better than a vector of strings
static mut _linesDeleted: usize = 0; // <- save deleted lines num for logger

static mut _linesIdent: usize = 0;
static mut _lineTokens: Vec<Token> = Vec::new();

pub unsafe fn readTokens(buffer: Vec<u8>) -> Vec<Line> {
    let mut lines:         Vec<Line> = Vec::new();
    let mut readLineIdent: bool      = true;

    _bufferLength = buffer.len();
    let mut c: u8;
    while _index < _bufferLength {
        c = buffer[_index];

        // ident
        if c == b' ' && _index+1 < _bufferLength && buffer[_index+1] == b' ' && readLineIdent {
            _index += 2;
            _indexCount += 2;

            _linesIdent += 1;
        } else {
            readLineIdent = false;
            // get endline
            if c == b'\n' || c == b';' {
                // bracket nesting
                bracketNesting(
                    &mut _lineTokens,
                    TokenType::CircleBracketBegin, 
                    TokenType::CircleBracketEnd
                );
                bracketNesting(
                    &mut _lineTokens,
                    TokenType::SquareBracketBegin, 
                    TokenType::SquareBracketEnd
                );
                bracketNesting(
                    &mut _lineTokens,
                    TokenType::FigureBracketBegin, 
                    TokenType::FigureBracketEnd
                );

                // add new line
                lines.push( Line {
                    tokens:       _lineTokens.clone(),
                    ident:        _linesIdent,
                    lines:        Vec::new(),
                    linesDeleted: _linesDeleted+_linesCount
                } );
                _linesDeleted = 0;
                _linesIdent = 0;

                readLineIdent = true;
                _lineTokens.clear();
                _index += 1;

                _linesCount += 1;
                _indexCount = 0;
            } else
            // delete comment
            if c == b'#' {
                deleteComment(&buffer);
            } else
            // get int-float
            if isDigit(c) || (c == b'-' && _index+1 < _bufferLength && isDigit(buffer[_index+1])) {
                _lineTokens.push( getNumber(&buffer) );
            } else
            // get word
            if isLetter(c) {
                _lineTokens.push( getWord(&buffer) );
            } else
            // get quotes ' " `
            if c == b'\'' || c == b'"' || c == b'`' {
                let token: Token = getQuotes(&buffer);
                if token.dataType != TokenType::None {
                    _lineTokens.push(token);
                } else {
                    _index += 1;
                    _indexCount += 1;
                }
            } else
            // get single and double chars
            if getSingleChar(c) {
                let token: Token = getOperator(&buffer);
                if token.dataType != TokenType::None {
                    _lineTokens.push(token);
                } else {
                    _index += 1;
                    _indexCount += 1;
                }
                // skip
            } else {
                _index += 1;
                _indexCount += 1;
            }
        }
    }
    // line nesting
    lineNesting(&mut lines);
    // delete DoubleComment
    {
        let mut i: usize = 0;
        let mut linesLen: usize = lines.len();
        let mut lineTokens: Vec<Token>;

        while i < linesLen {
            lineTokens = lines[i].tokens.clone();
            let mut lastLineToken: usize = lineTokens.len();
            if lineTokens.len() == 0 {
                i += 1;
                continue;
            } else {
                lastLineToken -= 1;
            }

            if lineTokens[lastLineToken].dataType == TokenType::Comment {
                lines[i].lines.clear();
                lines[i].tokens.remove(lastLineToken);
                if lines[i].tokens.len() == 0 {
                    lines.remove(i);
                    linesLen -= 1;
                    continue;
                }
            }
            i += 1;
        }
    }
    //
    if _debugMode {
        outputLines(&lines,0);
    }
    lines
}