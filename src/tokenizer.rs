/*
    tokenizer
*/

use crate::logger::*;

pub mod token; use crate::tokenizer::token::*;
pub mod line;  use crate::tokenizer::line::*;

use std::time::Instant;

use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

// delete comment
unsafe fn deleteComment(buffer: &[u8], index: &mut usize, bufferLength: usize) -> ()
{
  *index += 1;
  while *index < bufferLength && buffer[*index] != b'\n' 
  {
    *index += 1;
  }
}

// get single char token
fn isSingleChar(c: u8) -> bool 
{
  match c 
  {
    b'+' | b'-' | b'*' | b'/' | b'=' | b'%' | b'^' |
    b'>' | b'<' | b'?' | b'!' | b'&' | b'|' | 
    b'(' | b')' | b'{' | b'}' | b'[' | b']' | 
    b':' | b',' | b'.' | b'~' => true,
    _ => false,
  }
}

// is digit ?
fn isDigit(c: u8) -> bool 
{
  c >= b'0' && c <= b'9'
}
// get int-float token by buffer-index
unsafe fn getNumber(buffer: &[u8], index: &mut usize, bufferLength: usize) -> Token 
{
  let mut savedIndex: usize = *index; // index buffer
  let mut result: String = String::new();

  let mut      dot = false; // dot check
  let mut negative = false; // negative check
  let mut rational = false; // reational check

  while savedIndex < bufferLength 
  {
    let byte1: u8 = buffer[savedIndex]; // current char
    let byte2: u8 =                     // next char
      if savedIndex+1 < bufferLength 
      {
        buffer[savedIndex+1]
      } else 
      {
        b'\0'
      };

    if !negative && buffer[*index] == b'-' 
    { // Int/Float flag
      result.push(byte1 as char);
      negative = true;
      savedIndex += 1;
    } else
    if isDigit(byte1) 
    { // UInt
      result.push(byte1 as char);
      savedIndex += 1;
    } else 
    if byte1 == b'.' && !dot && isDigit(byte2) 
    { // UFloat
      if rational 
      {
          break;
      }
      dot = true;
      result.push(byte1 as char);
      savedIndex += 1;
    } else
    if byte1 == b'/' && byte2 == b'/' && !dot && 
       (savedIndex+2 < bufferLength && isDigit(buffer[savedIndex+2])) 
    { // Rational
      rational = true;
      result.push('/');
      result.push('/');
      savedIndex += 2;
    } else 
    {
      break;
    }
  }

  *index = savedIndex;
  // next return
  match (rational, dot, negative) 
  { //   rational,  dot,  negative
    (true, _, _)     => Token::new(TokenType::Rational, result.clone()),
    (_, true, true)  => Token::new(TokenType::Float,    result.clone()),
    (_, true, false) => Token::new(TokenType::UFloat,   result.clone()),
    (_, false, true) => Token::new(TokenType::Int,      result.clone()),
    _                => Token::new(TokenType::UInt,     result.clone()),
  }
}

// is letter ?
fn isLetter(c: u8) -> bool 
{
  (c >= b'a' && c <= b'z') ||
  (c >= b'A' && c <= b'Z')
}
// get word token by buffer-index
unsafe fn getWord(buffer: &[u8], index: &mut usize, bufferLength: usize) -> Token 
{
  let mut savedIndex: usize = *index; // index buffer
  let mut result: String = String::new();

  while savedIndex < bufferLength 
  {
    let byte1: u8 = buffer[savedIndex]; // current char
    let byte2: u8 =                  // next char
        if savedIndex+1 < bufferLength 
        {
          buffer[savedIndex+1]
        } else 
        {
          b'\0'
        };

    if isLetter(byte1) || 
       (byte1 == b'-' && !result.is_empty() && isLetter(byte2)) ||
       (isDigit(byte1) && !result.is_empty()) 
    {
      result.push(byte1 as char);
      savedIndex += 1;
    } else 
    {
      break;
    }
  }

  *index = savedIndex;
  // next return
  match &result[..] 
  {
    "true"     => Token::new(TokenType::Bool, String::from("1")),
    "false"    => Token::new(TokenType::Bool, String::from("0")),
    "loop"     => Token::newEmpty(TokenType::Loop),
    _          => Token::new(TokenType::Word, result.clone()),
  }
}

// get quotes token by buffer-index
unsafe fn getQuotes(buffer: &[u8], index: &mut usize,) -> Token 
{
  let byte1: u8 = buffer[*index]; // quote

  let length: usize = buffer.len();
  let mut result = String::new();

  if buffer[*index] == byte1 
  {
    let mut open:             bool = false;
    let mut noSlash:          bool;
    let mut backslashCounter: usize;

    while *index < length 
    {
      let byte2: u8 = buffer[*index]; // current char

      // check endline error
      if byte2 == b'\n' 
      {
        // quotes were not closed
        // skipped it!
        return Token::newEmpty(TokenType::None);
      }

      // read quote
      if byte2 != byte1 
      {
        result.push(byte2 as char);
      } else
      if byte2 == byte1 
      {
        noSlash = true;
        // check back slash of end quote
        if buffer[*index-1] == b'\\' 
        {
          backslashCounter = 1;
          for i in (0..*index-1).rev() 
          {
            if buffer[i] == b'\\' 
            {
              backslashCounter += 1;
            } else 
            {
              break;
            }
          }
          if backslashCounter % 2 == 1 
          {
            // add slash (\' \" \`)
            result.push(byte2 as char);
            noSlash = false;
          }
        }
        //
        if open && noSlash 
        {
          *index += 1;
          break;
        } else 
        {
          open = true;
        }
      }
      *index += 1;
    }
  }
  // next return
  if byte1 == b'\'' 
  {
    return if result.len() > 1 
    {
      // single quotes can only contain 1 character
      // skipped it!
      Token::newEmpty(TokenType::None)
    } else 
    {
      Token::new(TokenType::Char, result.clone())
    }
  } else if byte1 == b'"' 
  {
    Token::new(TokenType::String, result.clone())
  } else if byte1 == b'`' 
  {
    Token::new(TokenType::RawString, result.clone())
  } else 
  {
    Token::newEmpty(TokenType::None)
  }
}

// get operator token by buffer-index
unsafe fn getOperator(buffer: &[u8], index: &mut usize, bufferLength: usize) -> Token 
{
  let currentChar = buffer[*index];
  let nextChar = 
    if *index+1 < bufferLength 
    { 
      buffer[*index+1]
    } else 
    { 
      b'\0'
    };

  let mut increment = |count: usize| 
    {
      *index += count;
    };

  match currentChar 
  {
    b'+' => 
    {
           if nextChar == b'=' 
        { increment(2); Token::newEmpty(TokenType::PlusEquals) } 
      else if nextChar == b'+' 
        { increment(2); Token::newEmpty(TokenType::UnaryPlus) } 
      else 
        { increment(1); Token::newEmpty(TokenType::Plus) }
    }
    b'-' => 
    {
           if nextChar == b'=' 
        { increment(2); Token::newEmpty(TokenType::MinusEquals) } 
      else if nextChar == b'-' 
        { increment(2); Token::newEmpty(TokenType::UnaryMinus) } 
      else if nextChar == b'>' 
        { increment(2); Token::newEmpty(TokenType::Pointer) } 
      else 
        { increment(1); Token::newEmpty(TokenType::Minus) }
    }
    b'*' => 
    {
           if nextChar == b'=' 
        { increment(2); Token::newEmpty(TokenType::MultiplyEquals) } 
      else if nextChar == b'*' 
        { increment(2); Token::newEmpty(TokenType::UnaryMultiply) } 
      else 
        { increment(1); Token::newEmpty(TokenType::Multiply) }
    }
    b'/' => 
    {
           if nextChar == b'=' 
        { increment(2); Token::newEmpty(TokenType::DivideEquals) } 
      else if nextChar == b'/' 
        { increment(2); Token::newEmpty(TokenType::UnaryDivide) } 
      else 
        { increment(1); Token::newEmpty(TokenType::Divide) }
    }
    b'%' => 
    {
           if nextChar == b'=' 
        { increment(2); Token::newEmpty(TokenType::Modulo) } // todo: add new type in Token
      else if nextChar == b'%' 
        { increment(2); Token::newEmpty(TokenType::Modulo) } // todo: add new type in Token
      else 
        { increment(1); Token::newEmpty(TokenType::Modulo) }
    }
    b'^' => 
    {
           if nextChar == b'=' 
        { increment(2); Token::newEmpty(TokenType::Exponent) } // todo: add new type in Token
      else if nextChar == b'^' 
        { increment(2); Token::newEmpty(TokenType::Exponent) } // todo: add new type in Token
      else 
        { increment(1); Token::newEmpty(TokenType::Disjoint) }
    }
    b'>' => 
    {
      if nextChar == b'=' 
        { increment(2); Token::newEmpty(TokenType::GreaterThanOrEquals) } 
      else 
        { increment(1); Token::newEmpty(TokenType::GreaterThan) }
    }
    b'<' => 
    {
      if nextChar == b'=' 
        { increment(2); Token::newEmpty(TokenType::LessThanOrEquals) } 
      else 
        { increment(1); Token::newEmpty(TokenType::LessThan) }
    }
    b'!' => 
    {
      if nextChar == b'=' 
        { increment(2); Token::newEmpty(TokenType::NotEquals) } 
      else 
        { increment(1); Token::newEmpty(TokenType::Exclusion) }
    }
    b'&' => { increment(1); Token::newEmpty(TokenType::Joint) }
    b'|' => { increment(1); Token::newEmpty(TokenType::Inclusion) }
    b'=' => { increment(1); Token::newEmpty(TokenType::Equals) }
    // brackets
    b'(' => { increment(1); Token::newEmpty(TokenType::CircleBracketBegin) }
    b')' => { increment(1); Token::newEmpty(TokenType::CircleBracketEnd) }
    b'{' => { increment(1); Token::newEmpty(TokenType::FigureBracketBegin) }
    b'}' => { increment(1); Token::newEmpty(TokenType::FigureBracketEnd) }
    b'[' => { increment(1); Token::newEmpty(TokenType::SquareBracketBegin) }
    b']' => { increment(1); Token::newEmpty(TokenType::SquareBracketEnd) }
    // other
    b';' => { increment(1); Token::newEmpty(TokenType::Endline) }
    b':' => { increment(1); Token::newEmpty(TokenType::Colon) }
    b',' => { increment(1); Token::newEmpty(TokenType::Comma) }
    b'.' => { increment(1); Token::newEmpty(TokenType::Dot) }
    b'?' => { increment(1); Token::newEmpty(TokenType::Question) }
    b'~' => { increment(1); Token::newEmpty(TokenType::Tilde) }
    _ => Token::new(TokenType::None, String::new()),
  }
}

// bracket nasting [begin bracket -> end bracket]
//                [+ recall in token child tokens]
// 1 () no tokens childrens -> 
// 2 [] tokens childrens 1  ->
// 3 {} tokens childres 1+2
unsafe fn bracketNesting(tokens: &mut Vec<Token>, index: &mut usize, beginType: &TokenType, endType: &TokenType) -> ()
{
  for token in tokens.iter_mut() 
  {
    if token.tokens.len() > 0 
    {
      bracketNesting(&mut token.tokens, index, beginType, endType);
    }
  }
  blockNesting(tokens, index, beginType, endType);
}
// block nasting [begin token -> end token]
unsafe fn blockNesting(tokens: &mut Vec<Token>, index: &mut usize, beginType: &TokenType, endType: &TokenType) -> ()
{
  let mut brackets: Vec::<usize> = Vec::new();
  let mut   length: usize        = tokens.len();

  let mut l = 0; // index buffer
  while l < length 
  {
    let tokenType: &TokenType = tokens[l].getDataType();
    if tokenType == beginType 
    {
      brackets.push(l);
    } else if tokenType == endType 
    {
      if let Some(penultBracket) = brackets.pop() 
      {
        if !brackets.is_empty() 
        {
          let savedToken: Token = tokens[penultBracket].clone();
          tokens[ brackets[brackets.len()-1] ]
            .tokens.push( savedToken.clone() );

          tokens.remove(penultBracket);
          length -= 1;

          if penultBracket < l 
          {
            l -= 1;
          }
        }
      }

      tokens.remove(l);
      length -= 1;
      continue;
    } else if !brackets.is_empty() 
    {
      let savedToken: Token = tokens.remove(l);
      length -= 1;

      tokens[ brackets[brackets.len()-1] ]
        .tokens.push( savedToken.clone() );
      continue;
    }
    l += 1;
  }
}
// line nesting [line -> line]
fn lineNesting(linesLinks: &mut Vec< Arc<RwLock<Line>> >) -> ()
{
  let mut index:     usize = 0;
  let mut nextIndex: usize = 1;
  let mut length:    usize = linesLinks.len();

  while index < length 
  {
    if nextIndex < length 
    {
      let isNesting: bool = 
        { // check current indent < next indent
            let currentLine: RwLockReadGuard<'_, Line> = linesLinks    [index].read().unwrap();
            let nextLine:    RwLockReadGuard<'_, Line> = linesLinks[nextIndex].read().unwrap();
            currentLine.indent < nextLine.indent
        };
      if isNesting 
      {
        // get next line and remove
        let nestingLineLink = linesLinks.remove(nextIndex);
        length -= 1;
        { // set parent line link
          let mut nestingLine: RwLockWriteGuard<'_, Line> = nestingLineLink.write().unwrap();
          nestingLine.parent = Some( linesLinks[index].clone() );
        }
        // push nesting
        let mut currentLine = linesLinks[index].write().unwrap();
        currentLine.lines.push(nestingLineLink); // nesting
        lineNesting(&mut currentLine.lines);     // cycle
      } else {
        index += 1; // next line < current line => skip
        nextIndex = index+1;
      }
    } else {
      break; // if no lines
    }
  }
}
// get new line nesting nums
fn setLineNestingNums(linesLinks: &mut Vec< Arc<RwLock<Line>> >) {
    for (i, lineLink) in linesLinks.iter().enumerate() {
        let mut line = lineLink.write().unwrap();
        line.index = i;
        if line.lines.len() > 0 {
            setLineNestingNums(&mut line.lines);
        }
    }
}

// delete DoubleComment
unsafe fn deleteDoubleComment(linesLinks: &mut Vec< Arc<RwLock<Line>> >, mut index: usize) -> ()
{
  let mut linesLinksLength: usize = linesLinks.len();
  let mut lastTokenIndex:   usize;

  while index < linesLinksLength 
  {
    let mut deleteLine: bool  = false;
    'exit: 
    { // interrupt
      // get line and check lines len
      let mut line: RwLockWriteGuard<'_, Line> = linesLinks[index].write().unwrap();
      if !line.lines.is_empty() 
      {
        deleteDoubleComment(&mut line.lines, index);
      }
      // skip separator
      if line.tokens.is_empty() {
        break 'exit;
      }
      // ? delete comment
      lastTokenIndex = line.tokens.len()-1;
      if *line.tokens[lastTokenIndex].getDataType() == TokenType::Comment {
        line.tokens.remove(lastTokenIndex);
        if line.tokens.is_empty() { // go to delete empty line
          deleteLine = true;        //
          break 'exit;              //
        }
      }
    }
    // after interrupt -> delete line
    if deleteLine 
    {
      linesLinks.remove(index);
      linesLinksLength -= 1;
      continue;
    }
    // next
    index += 1;
  }
}

// output token and its tokens
pub unsafe fn outputTokens(tokens: &Vec<Token>, lineIdent: usize, indent: usize) -> ()
{
  let lineIdentString: String = " ".repeat(lineIdent*2+1);
  let identString:     String = " ".repeat(indent*2+1);

  let tokenCount: usize = tokens.len();
  for (i, token) in tokens.iter().enumerate() 
  {
    let c: char = 
      if i == tokenCount-1 
      {
        'X'
      } else 
      {
        '┃'
      };

    if !token.getData().is_empty() 
    {
    // single quote
      if *token.getDataType() == TokenType::Char || *token.getDataType() == TokenType::FormattedChar {
        log("parserToken",&format!(
          "{}{}{}\\fg(#f0f8ff)\\b'\\c{}\\fg(#f0f8ff)\\b'\\c  |{}",
          lineIdentString,
          c,
          identString,
          token.getData(),
          token.getDataType().to_string()
        ));
    // double quote
      } else
      if *token.getDataType() == TokenType::String || *token.getDataType() == TokenType::FormattedString {
        log("parserToken",&format!(
          "{}{}{}\\fg(#f0f8ff)\\b\"\\c{}\\fg(#f0f8ff)\\b\"\\c  |{}",
          lineIdentString,
          c,
          identString,
          token.getData(),
          token.getDataType().to_string()
        ));
    // back quote
      } else
      if *token.getDataType() == TokenType::RawString || *token.getDataType() == TokenType::FormattedRawString {
        log("parserToken",&format!(
          "{}{}{}\\fg(#f0f8ff)\\b`\\c{}\\fg(#f0f8ff)\\b`\\c  |{}",
          lineIdentString,
          c,
          identString,
          token.getData(),
          token.getDataType().to_string()
        ));
    // basic
      } else {
        log("parserToken",&format!(
          "{}{}{}{}  |{}",
          lineIdentString,
          c,
          identString,
          token.getData(),
          token.getDataType().to_string()
        ));
      }
    // type only
    } else {
      println!(
        "{}{}{}{}",
        lineIdentString,
        c,
        identString,
        token.getDataType().to_string()
      );
    }
    if (&token.tokens).len() > 0 
    {
        outputTokens(&token.tokens, lineIdent, indent+1)
    }
    //
  }
}
// output line info
pub unsafe fn outputLines(linesLinks: &Vec< Arc<RwLock<Line>> >, indent: usize) -> ()
{
  let identStr1: String = " ".repeat(indent*2);
  let identStr2: String = " ".repeat(indent*2+1);

  for (i, line) in linesLinks.iter().enumerate() 
  {
    let line = line.read().unwrap();
    log("parserBegin", &format!("{} {}",identStr1,i));

    if (&line.tokens).len() == 0 
    {
      log("parserHeader", &format!("{}┗ Separator",identStr2));
    } else 
    {
      log("parserHeader", &format!("{}┣ Tokens",identStr2));
    }

    outputTokens(&line.tokens, indent, 1);
    if (&line.lines).len() > 0 
    {
      log("parserHeader", &format!("{}┗ Lines",identStr2));
      outputLines(&line.lines, indent+1);
    }
  }
  //
}

// tokens reader cycle
pub unsafe fn readTokens(buffer: Vec<u8>, debugMode: bool) -> Vec< Arc<RwLock<Line>> > 
{
  if debugMode 
  {
    logSeparator("AST");
    log("ok","+Generation");
    println!("     ┃");
  }

  let mut      index: usize = 0;
  let   bufferLength: usize = buffer.len();
  let mut  lineIdent: usize = 0;
  let mut lineTokens: Vec<Token> = Vec::new();

  let startTime: Instant = Instant::now();

  let mut linesLinks:    Vec< Arc<RwLock<Line>> > = Vec::new();
  let mut readLineIdent: bool                     = true;

  while index < bufferLength 
  {
    let byte: u8 = buffer[index]; // current char

    // indent
    if byte == b' ' && index+1 < bufferLength && buffer[index+1] == b' ' && readLineIdent 
    {
      index += 2;
      lineIdent += 1;
    } else 
    {
      readLineIdent = false;
      // get endline
      if byte == b'\n' || byte == b';' 
      {
        // bracket nesting
        bracketNesting(
          &mut lineTokens,
          &mut index,
          &TokenType::CircleBracketBegin, 
          &TokenType::CircleBracketEnd
        );
        bracketNesting(
          &mut lineTokens,
          &mut index,
          &TokenType::SquareBracketBegin, 
          &TokenType::SquareBracketEnd
        );
        bracketNesting(
          &mut lineTokens,
          &mut index,
          &TokenType::FigureBracketBegin, 
          &TokenType::FigureBracketEnd
        );

        // add new line
        linesLinks.push( 
          Arc::new(RwLock::new( 
            Line {
              tokens: lineTokens.clone(),
              indent: lineIdent,
              index:  0,
              lines:  Vec::new(),
              parent: None
            }
          ))
        );
        lineIdent = 0;

        readLineIdent = true;
        lineTokens.clear();
        index += 1;
      } else
      // delete comment
      if byte == b'#' 
      {
        deleteComment(&buffer, &mut index, bufferLength);
        lineTokens.push( Token::newEmpty(TokenType::Comment) );
      } else
      // get int-float
      if isDigit(byte) || (byte == b'-' && index+1 < bufferLength && isDigit(buffer[index+1])) 
      {
        lineTokens.push( getNumber(&buffer, &mut index, bufferLength) );
      } else
      // get word
      if isLetter(byte) 
      {
        lineTokens.push( getWord(&buffer, &mut index, bufferLength) );
      } else
      // get quotes ' " `
      if byte == b'\'' || byte == b'"' || byte == b'`' 
      {
        let mut token: Token = getQuotes(&buffer, &mut index);
        if *token.getDataType() != TokenType::None 
        {
          let backTokenIndex = lineTokens.len()-1;
          // if formatted quotes
          if *lineTokens[backTokenIndex].getDataType() == TokenType::Word && lineTokens[backTokenIndex].getData() == "f" 
          {
            if *token.getDataType() == TokenType::RawString { token.setDataType( TokenType::FormattedRawString ); } else
            if *token.getDataType() == TokenType::String    { token.setDataType( TokenType::FormattedString );    } else
            if *token.getDataType() == TokenType::Char      { token.setDataType( TokenType::FormattedChar );      }
            lineTokens[backTokenIndex] = token.clone(); // todo: remove copy please
          // basic quotes
          } else 
          {
            lineTokens.push(token.clone()); // todo: remove copy please
          }
        } else 
        {
          index += 1;
        }
      } else
      // get single and double chars
      if isSingleChar(byte) 
      {
        let token: Token = getOperator(&buffer, &mut index, bufferLength);
        if *token.getDataType() != TokenType::None 
        {
            lineTokens.push(token.clone()); // todo: remove copy
        } else 
        {
          index += 1;
        }
          // skip
      } else 
      {
        index += 1;
      }
    }
  }

  // line nesting
  lineNesting(&mut linesLinks);

  // delete DoubleComment
  deleteDoubleComment(&mut linesLinks, 0);

  // debug output and return
  if debugMode 
  {
    // duration
    let endTime  = Instant::now();
    let duration = endTime-startTime;
    // lines
    outputLines(&linesLinks,2);
    //
    println!("     ┃");
    log("ok",&format!("xDuration: {:?}",duration));
  }
  setLineNestingNums(&mut linesLinks); // set correct line nums
  linesLinks
}