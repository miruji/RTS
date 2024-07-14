/*
    tokenizer
*/

use crate::logger::*;
use crate::_filePath;
use crate::_debugMode;

pub mod token; use crate::tokenizer::token::*;
pub mod line;  use crate::tokenizer::line::*;

// prevariables
// in fact, i moved the most used variables here so that reading happens faster, 
// without re-declaring identical memory areas
static mut __index:  usize  = 0;                               // index  buffer
static mut __length: usize  = 0;                               // length buffer

static mut __byte1:  u8     = b'\0';                           // byte 1 buffer
static mut __byte2:  u8     = b'\0';                           // byte 2 buffer
static mut __char:   char   = '\0';                            // char   buffer
static mut __result: String = String::new();                   // result buffer
static mut __bool1:  bool   = false;                           // bool 1 buffer
static mut __bool2:  bool   = false;                           // bool 2 buffer
static mut __bool3:  bool   = false;                           // bool 3 buffer

static mut __token:     Token  = Token::newStatic();           // Token     buffer
static mut __tokenType: &mut TokenType = &mut TokenType::None; // TokenType buffer
static mut __brackets:  Vec::<usize> = Vec::new();             // brackets  buffer

// delete comment
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
    __index = _index; // index buffer
    __result = String::new();

    __bool1 = false; // dot check
    __bool2 = false; // negative check
    __bool3 = false; // reational checl

    while __index < _bufferLength {
        __byte1 = buffer[__index]; // current char
        __byte2 =               // next char
            if __index+1 < _bufferLength {
                buffer[__index+1]
            } else {
                b'\0'
            };

        if !__bool2 && buffer[_index] == b'-' {
            __result.push(__byte1 as char);
            __bool2 = true;
            __index += 1;
        } else
        if isDigit(__byte1) {
            __result.push(__byte1 as char);
            __index += 1;
        } else 
        if __byte1 == b'.' && !__bool1 && isDigit(__byte2) {
            if __bool3 { // Rational number use Int-UInt/Int-UInt
                break;
            }
            __bool1 = true;
            __result.push(__byte1 as char);
            __index += 1;
        } else
        if __byte1 == b'/' && __byte2 == b'/' && !__bool1 && 
           (__index+2 < _bufferLength && isDigit(buffer[__index+2])) {
            __bool3 = true;
            __result.push('/');
            __result.push('/');
            __index += 2;
        } else {
            break;
        }
    }

    if !__result.is_empty() {
        _index = __index;
        _indexCount += __result.len();
    }

    match (__bool3, __bool1, __bool2) { // rational, dot, negative
        (true, _, _)     => Token::new(TokenType::Rational, __result.clone()),
        (_, true, true)  => Token::new(TokenType::Float,    __result.clone()),
        (_, true, false) => Token::new(TokenType::UFloat,   __result.clone()),
        (_, false, true) => Token::new(TokenType::Int,      __result.clone()),
        _                => Token::new(TokenType::UInt,     __result.clone()),
    }
}
// get word token by buffer-index
fn isLetter(c: u8) -> bool {
    (c >= b'a' && c <= b'z') ||
    (c >= b'A' && c <= b'Z')
}
unsafe fn getWord(buffer: &[u8]) -> Token {
    __index = _index;
    __result = String::new();

    while __index < _bufferLength {
        __byte1 = buffer[__index]; // current char
        __byte2 =               // next char
            if __index+1 < _bufferLength {
                buffer[__index+1]
            } else {
                b'\0'
            };

        if isLetter(__byte1) || (__byte1 == b'-' && !__result.is_empty() && isLetter(__byte2)) {
            __result.push(__byte1 as char);
            __index += 1;
        } else {
            break;
        }
    }

    if !__result.is_empty() {
        _index = __index;
        _indexCount += __result.len();
    }

    match &__result[..] {
        "Int"      => Token::newEmpty(TokenType::Int),
        "UInt"     => Token::newEmpty(TokenType::UInt),
        "Float"    => Token::newEmpty(TokenType::Float),
        "UFloat"   => Token::newEmpty(TokenType::UFloat),
        "Rational" => Token::newEmpty(TokenType::Rational),
        "and"      => Token::newEmpty(TokenType::And),
        "or"       => Token::newEmpty(TokenType::Or),
        "true"     => Token::newEmpty(TokenType::True),
        "false"    => Token::newEmpty(TokenType::False),
        "loop"     => Token::newEmpty(TokenType::Loop),
        _          => Token::new(TokenType::Word, __result.clone()),
    }
}
// get quotes token by buffer-index
unsafe fn getQuotes(buffer: &[u8]) -> Token {
    __byte1 = buffer[_index]; // quote

    __length = buffer.len();
    __result = String::new();

    if buffer[_index] == __byte1 {
        let mut open:        bool = false;
        let mut noSlash:          bool;
        let mut backslashCounter: usize;

        while _index < __length {
            __byte2 = buffer[_index]; // current char

            // check endline error
            if __byte2 == b'\n' {
                // quotes were not closed
                // skipped it!
                return Token::newEmpty(TokenType::None);
            }

            // read quote
            if __byte2 != __byte1 {
                __result.push(__byte2 as char);
            } else
            if __byte2 == __byte1 {
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
                        __result.push(__byte2 as char);
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
    return if __byte1 == b'\'' {
        return if __result.len() > 1 {
            // single quotes can only contain 1 character
            // skipped it!
            Token::newEmpty(TokenType::None)
        } else {
            Token::new(TokenType::Char, __result.clone())
        }
    } else if __byte1 == b'"' {
        Token::new(TokenType::String, __result.clone())
    } else if __byte1 == b'`' {
        Token::new(TokenType::SpecialString, __result.clone())
    } else {
        Token::newEmpty(TokenType::None)
    }
}
// get operator token by buffer-index
unsafe fn getOperator(buffer: &[u8]) -> Token {
    __byte1 = // next char
        if _index+1 < _bufferLength {
            buffer[_index+1]
        } else {
            b'\0'
        };
    return match buffer[_index] {
        // += ++ +
        b'+' => if __byte1 == b'=' {
            _index += 2;
            _indexCount += 2;
            Token::newEmpty(TokenType::PlusEquals)
        } else if __byte1 == b'+' {
            _index += 2;
            _indexCount += 2;
            Token::newEmpty(TokenType::UnaryPlus)
        } else {
            _index += 1;
            _indexCount += 1;
            Token::newEmpty(TokenType::Plus)
        },
        // -= -- -
        b'-' => if __byte1 == b'=' {
            _index += 2;
            _indexCount += 2;
            Token::newEmpty(TokenType::MinusEquals)
        } else if __byte1 == b'-' {
            _index += 2;
            _indexCount += 2;
            Token::newEmpty(TokenType::UnaryMinus)
        } else if __byte1 == b'>' {
            _index += 2;
            _indexCount += 2;
            Token::newEmpty(TokenType::Pointer)
        } else {
            _index += 1;
            _indexCount += 1;
            Token::newEmpty(TokenType::Minus)
        },
        // *= *
        b'*' => if __byte1 == b'=' {
            _index += 2;
            _indexCount += 2;
            Token::newEmpty(TokenType::MultiplyEquals)
        } else if __byte1 == b'*' {
            _index += 2;
            _indexCount += 2;
            Token::newEmpty(TokenType::UnaryMultiply)
        } else {
            _index += 1;
            _indexCount += 1;
            Token::newEmpty(TokenType::Multiply)
        },
        // /= /
        b'/' => if __byte1 == b'=' {
            _index += 2;
            _indexCount += 2;
            Token::newEmpty(TokenType::DivideEquals)
        } else if __byte1 == b'/' {
            _index += 2;
            _indexCount += 2;
            Token::newEmpty(TokenType::UnaryDivide)
        } else {
            _index += 1;
            _indexCount += 1;
            Token::newEmpty(TokenType::Divide)
        },
        // >= >
        b'>' => if __byte1 == b'=' {
            _index += 2;
            _indexCount += 2;
            Token::newEmpty(TokenType::GreaterThanOrEquals)
        } else {
            _index += 1;
            _indexCount += 1;
            Token::newEmpty(TokenType::GreaterThan)
        },
        // <=
        b'<' => if __byte1 == b'=' {
            _index += 2;
            _indexCount += 2;
            Token::newEmpty(TokenType::LessThanOrEquals)
        } else {
            _index += 1;
            _indexCount += 1;
            Token::newEmpty(TokenType::LessThan)
        },
        // != !
        b'!' => if __byte1 == b'=' {
            _index += 2;
            _indexCount += 2;
            Token::newEmpty(TokenType::NotEquals)
        } else {
            _index += 1;
            _indexCount += 1;
            Token::newEmpty(TokenType::Not)
        },
        // &&
        b'&' => if __byte1 == b'&' {
            _index += 2;
            _indexCount += 2;
            Token::newEmpty(TokenType::And)
        } else {
            _index += 1;
            _indexCount += 1;
            Token::newEmpty(TokenType::And) // todo: single and
        },
        // ||
        b'|' => if __byte1 == b'|' {
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

// bracket nasting [begin bracket -> end bracket]
//                [+ recall in token child tokens]
// 1 () no tokens childrens -> 
// 2 [] tokens childrens 1  ->
// 3 {} tokens childres 1+2
unsafe fn bracketNesting(tokens: &mut Vec<Token>, beginType: &TokenType, endType: &TokenType) {
    for token in tokens.iter_mut() {
        if token.tokens.len() > 0 {
            bracketNesting(&mut token.tokens, beginType, endType);
        }
    }
    blockNesting(tokens, beginType, endType);
}
// block nasting [begin token -> end token]
unsafe fn blockNesting(tokens: &mut Vec<Token>, beginType: &TokenType, endType: &TokenType) {
    __brackets = Vec::new();
    __length = tokens.len();

    __index = 0; // index buffer
    while __index < __length {
        *__tokenType = tokens[__index].dataType.clone();
        if __tokenType == beginType {
            __brackets.push(__index);
        } else if __tokenType == endType {
            if let Some(penultBracket) = __brackets.pop() {
                if !__brackets.is_empty() {
                    __token = tokens[penultBracket].clone();
                    tokens[ __brackets[__brackets.len()-1] ]
                        .tokens.push( __token.clone() );

                    tokens.remove(penultBracket);
                    __length -= 1;

                    if penultBracket < __index {
                        __index -= 1;
                    }
                }
            }

            tokens.remove(__index);
            __length -= 1;
            continue;
        } else if !__brackets.is_empty() {
            __token = tokens.remove(__index);
            __length -= 1;

            tokens[ __brackets[__brackets.len()-1] ]
                .tokens.push( __token.clone() );
            continue;
        }
        __index += 1;
    }
}
// line nesting [line -> line]
//          [recall for line lines]
unsafe fn lineNesting(lines: &mut Vec<Line>, mut k: usize) -> usize {
    let mut nextLine: Line;

    __index = 0;           // index buffer
    __length = lines.len(); // lines length
    while __index < __length {
        if __index+1 < __length && lines[__index].indent < lines[__index+1].indent {
            nextLine = lines.remove(__index+1);                // clone and remove next line
            __length -= 1;

            lines[__index].lines.push(nextLine);               // nesting
            __length = lineNesting(&mut lines[__index].lines, __length); // cycle
        } else {
            __index += 1;
        }
    }

    return k;
}

// delete DoubleComment
unsafe fn deleteDoubleComment(lines: &mut Vec<Line>, mut ib: usize) {
    let mut lastTokenIndex: usize;

    while ib < lines.len() {
        if !lines[ib].lines.is_empty() {
            deleteDoubleComment(&mut lines[ib].lines, ib);
        }

        if lines[ib].tokens.is_empty() {
            if lines[ib].lines.is_empty() {
                ib += 1;
            } else {
                lines.remove(ib);
            }
            continue;
        }

        lastTokenIndex = lines[ib].tokens.len()-1;
        if lines[ib].tokens[lastTokenIndex].dataType == TokenType::Comment {
            lines[ib].tokens.remove(lastTokenIndex);
            if lines[ib].tokens.is_empty() {
                lines.remove(ib);
                continue;
            }
        }

        ib += 1;
    }
}

// output token and its tokens
pub unsafe fn outputTokens(tokens: &Vec<Token>, lineIdent: usize, indent: usize) {
    let lineIdentString: String = " ".repeat(lineIdent*2+1);
    let identString:     String = " ".repeat(indent*2+1);

    let tokenCount: usize = tokens.len();
    for (i, token) in tokens.iter().enumerate() {
        __char = 
            if i == tokenCount-1 {
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
                    __char,
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
                    __char,
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
                    __char,
                    identString,
                    token.data,
                    token.dataType.to_string()
                ));
        // basic
            } else {
                log("parserToken",&format!(
                    "{}{}{}{}  |{}",
                    lineIdentString,
                    __char,
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
                __char,
                identString,
                token.dataType.to_string()
            );
        }
        if (&token.tokens).len() > 0 {
            outputTokens(&token.tokens, lineIdent, indent+1)
        }
    }
}
// output line info
pub unsafe fn outputLines(lines: &Vec<Line>, indent: usize) {
    let identStr1: String = " ".repeat(indent*2);
    let identStr2: String = " ".repeat(indent*2+1);
    for (i, line) in lines.iter().enumerate() {
        log("parserBegin", &format!("{}+{}",identStr1,i));

        if (&line.tokens).len() == 0 {
            log("parserHeader", &format!("{}┗ Separator",identStr2));
        } else {
            log("parserHeader", &format!("{}┣ Tokens",identStr2));
        }

        outputTokens(&line.tokens, indent, 1);
        if (&line.lines).len() > 0 {
            log("parserHeader", &format!("{}┗ Lines",identStr2));
            outputLines(&line.lines, indent+1);
        }
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
    while _index < _bufferLength {
        __byte1 = buffer[_index]; // current char

        // indent
        if __byte1 == b' ' && _index+1 < _bufferLength && buffer[_index+1] == b' ' && readLineIdent {
            _index += 2;
            _indexCount += 2;

            _linesIdent += 1;
        } else {
            readLineIdent = false;
            // get endline
            if __byte1 == b'\n' || __byte1 == b';' {
                // bracket nesting
                bracketNesting(
                    &mut _lineTokens,
                    &TokenType::CircleBracketBegin, 
                    &TokenType::CircleBracketEnd
                );
                bracketNesting(
                    &mut _lineTokens,
                    &TokenType::SquareBracketBegin, 
                    &TokenType::SquareBracketEnd
                );
                bracketNesting(
                    &mut _lineTokens,
                    &TokenType::FigureBracketBegin, 
                    &TokenType::FigureBracketEnd
                );

                // add new line
                lines.push( Line {
                    tokens:       _lineTokens.clone(),
                    indent:        _linesIdent,
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
            if __byte1 == b'#' {
                deleteComment(&buffer);
            } else
            // get int-float
            if isDigit(__byte1) || (__byte1 == b'-' && _index+1 < _bufferLength && isDigit(buffer[_index+1])) {
                _lineTokens.push( getNumber(&buffer) );
            } else
            // get word
            if isLetter(__byte1) {
                _lineTokens.push( getWord(&buffer) );
            } else
            // get quotes ' " `
            if __byte1 == b'\'' || __byte1 == b'"' || __byte1 == b'`' {
                __token = getQuotes(&buffer);
                if __token.dataType != TokenType::None {
                    _lineTokens.push(__token.clone()); // todo: remove copy
                } else {
                    _index += 1;
                    _indexCount += 1;
                }
            } else
            // get single and double chars
            if getSingleChar(__byte1) {
                __token = getOperator(&buffer);
                if __token.dataType != TokenType::None {
                    _lineTokens.push(__token.clone()); // todo: remove copy
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
    lineNesting(&mut lines, 0);

    // delete DoubleComment
    __index = 0;
    deleteDoubleComment(&mut lines, __index);

    //
    if _debugMode {
        outputLines(&lines,0);
    }
    lines
}