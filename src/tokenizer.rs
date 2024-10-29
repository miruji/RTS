/*
    tokenizer
*/

pub mod token;
pub mod line;

use crate::{
  logger::*,
  tokenizer::token::*,
  tokenizer::line::*
};

use std::{
  time::{Instant,Duration},
  sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard}
};

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
    if byte1 == b'.' && !dot && isDigit(byte2) &&
       savedIndex > 1 && buffer[*index-1] != b'.' // fixed for a.0.1
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
    (true, _, _)     => Token::new( Some(TokenType::Rational), Some(result.clone()) ),
    (_, true, true)  => Token::new( Some(TokenType::Float),    Some(result.clone()) ),
    (_, true, false) => Token::new( Some(TokenType::UFloat),   Some(result.clone()) ),
    (_, false, true) => Token::new( Some(TokenType::Int),      Some(result.clone()) ),
    _                => Token::new( Some(TokenType::UInt),     Some(result.clone()) ),
  }
}

// is letter ?
fn isLetter(c: u8) -> bool 
{
  (c|32)>=b'a'&&(c|32)<=b'z'
}
// get word token by buffer-index
unsafe fn getWord(buffer: &[u8], index: &mut usize, bufferLength: usize) -> Token 
{
  let mut savedIndex: usize = *index; // index buffer
  let mut result: String = String::new();
  let mut isLink: bool = false;

  while savedIndex < bufferLength 
  {
    let byte1: u8 = buffer[savedIndex]; // current char

    if (isDigit(byte1) || byte1 == b'.') && !result.is_empty()
    {
      result.push(byte1 as char);
      savedIndex += 1;
      isLink = true;
    } else 
    if isLetter(byte1)
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
  if isLink 
  {
    Token::new( Some(TokenType::Link), Some(result.clone()) )
  } else 
  {
    match &result[..] 
    {
      "true"     => Token::new( Some(TokenType::Bool), Some("1".to_string()) ),
      "false"    => Token::new( Some(TokenType::Bool), Some("0".to_string()) ),
      _          => Token::new( Some(TokenType::Word), Some(result) ),
    }
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
        return Token::newEmpty(None);
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
      Token::newEmpty(None)
    } else 
    {
      Token::new( Some(TokenType::Char), Some(result.clone()) )
    }
  } else if byte1 == b'"' 
  {
    Token::new( Some(TokenType::String), Some(result.clone()) )
  } else if byte1 == b'`' 
  {
    Token::new( Some(TokenType::RawString), Some(result.clone()) )
  } else 
  {
    Token::newEmpty(None)
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
        { increment(2); Token::newEmpty( Some(TokenType::PlusEquals) ) }
      else if nextChar == b'+' 
        { increment(2); Token::newEmpty( Some(TokenType::UnaryPlus) ) }
      else 
        { increment(1); Token::newEmpty( Some(TokenType::Plus) ) }
    }
    b'-' => 
    {
           if nextChar == b'=' 
        { increment(2); Token::newEmpty( Some(TokenType::MinusEquals) ) }
      else if nextChar == b'-' 
        { increment(2); Token::newEmpty( Some(TokenType::UnaryMinus) ) }
      else if nextChar == b'>' 
        { increment(2); Token::newEmpty( Some(TokenType::Pointer) ) }
      else 
        { increment(1); Token::newEmpty( Some(TokenType::Minus) ) }
    }
    b'*' => 
    {
           if nextChar == b'=' 
        { increment(2); Token::newEmpty( Some(TokenType::MultiplyEquals) ) }
      else if nextChar == b'*' 
        { increment(2); Token::newEmpty( Some(TokenType::UnaryMultiply) ) }
      else 
        { increment(1); Token::newEmpty( Some(TokenType::Multiply) ) }
    }
    b'/' => 
    {
           if nextChar == b'=' 
        { increment(2); Token::newEmpty( Some(TokenType::DivideEquals) ) }
      else if nextChar == b'/' 
        { increment(2); Token::newEmpty( Some(TokenType::UnaryDivide) ) }
      else 
        { increment(1); Token::newEmpty( Some(TokenType::Divide) ) }
    }
    b'%' => 
    {
           if nextChar == b'=' 
        { increment(2); Token::newEmpty( Some(TokenType::Modulo) ) } // todo: add new type in Token
      else if nextChar == b'%' 
        { increment(2); Token::newEmpty( Some(TokenType::Modulo) ) } // todo: add new type in Token
      else 
        { increment(1); Token::newEmpty( Some(TokenType::Modulo) ) }
    }
    b'^' => 
    {
           if nextChar == b'=' 
        { increment(2); Token::newEmpty( Some(TokenType::Exponent) ) } // todo: add new type in Token
      else if nextChar == b'^' 
        { increment(2); Token::newEmpty( Some(TokenType::Exponent) ) } // todo: add new type in Token
      else 
        { increment(1); Token::newEmpty( Some(TokenType::Disjoint) ) }
    }
    b'>' => 
    {
      if nextChar == b'=' 
        { increment(2); Token::newEmpty( Some(TokenType::GreaterThanOrEquals) ) }
      else 
        { increment(1); Token::newEmpty( Some(TokenType::GreaterThan) ) }
    }
    b'<' => 
    {
      if nextChar == b'=' 
        { increment(2); Token::newEmpty( Some(TokenType::LessThanOrEquals) ) }
      else 
        { increment(1); Token::newEmpty( Some(TokenType::LessThan) ) }
    }
    b'!' => 
    {
      if nextChar == b'=' 
        { increment(2); Token::newEmpty( Some(TokenType::NotEquals) ) }
      else 
        { increment(1); Token::newEmpty( Some(TokenType::Exclusion) ) }
    }
    b'&' => { increment(1); Token::newEmpty( Some(TokenType::Joint) ) }
    b'|' => { increment(1); Token::newEmpty( Some(TokenType::Inclusion) ) }
    b'=' => { increment(1); Token::newEmpty( Some(TokenType::Equals) ) }
    // brackets
    b'(' => { increment(1); Token::newEmpty( Some(TokenType::CircleBracketBegin) ) }
    b')' => { increment(1); Token::newEmpty( Some(TokenType::CircleBracketEnd) ) }
    b'{' => { increment(1); Token::newEmpty( Some(TokenType::FigureBracketBegin) ) }
    b'}' => { increment(1); Token::newEmpty( Some(TokenType::FigureBracketEnd) ) }
    b'[' => { increment(1); Token::newEmpty( Some(TokenType::SquareBracketBegin) ) }
    b']' => { increment(1); Token::newEmpty( Some(TokenType::SquareBracketEnd) ) }
    // other
    b';' => { increment(1); Token::newEmpty( Some(TokenType::Endline) ) }
    b':' => { increment(1); Token::newEmpty( Some(TokenType::Colon) ) }
    b',' => { increment(1); Token::newEmpty( Some(TokenType::Comma) ) }
    b'.' => { increment(1); Token::newEmpty( Some(TokenType::Dot) ) }
    b'?' => { increment(1); Token::newEmpty( Some(TokenType::Question) ) }
    b'~' => { increment(1); Token::newEmpty( Some(TokenType::Tilde) ) }
    _ => Token::newEmpty( None ),
  }
}

// bracket nasting [begin bracket -> end bracket]
//                [+ recall in token child tokens]
// 1 () no tokens childrens -> 
// 2 [] tokens childrens 1  ->
// 3 {} tokens childres 1+2
unsafe fn bracketNesting(tokens: &mut Vec<Token>, beginType: &TokenType, endType: &TokenType) -> ()
{
  for token in tokens.iter_mut() 
  {
    if let Some(ref mut tokens) = token.tokens 
    {
      bracketNesting(tokens, beginType, endType);
    }
  }
  blockNesting(tokens, beginType, endType);
}
// block nasting [begin token -> end token]
unsafe fn blockNesting(tokens: &mut Vec<Token>, beginType: &TokenType, endType: &TokenType) -> ()
{
  let mut brackets: Vec::<usize> = Vec::new();
  let mut   length: usize        = tokens.len();

  let mut l = 0; // index buffer
  while l < length 
  {
    let tokenType: &TokenType = &tokens[l].getDataType().unwrap_or_default();
    if tokenType == beginType 
    {
      brackets.push(l);
    } else if tokenType == endType 
    {
      if let Some(penultBracket) = brackets.pop() 
      {
        if !brackets.is_empty() 
        {
          // add nesting
          let savedToken: Token = tokens[penultBracket].clone();
          if let Some(token) = tokens.get_mut(brackets[brackets.len()-1]) 
          {
            if let Some(tokenTokens) = &mut token.tokens 
            { // contains tokens 
              tokenTokens.push(savedToken.clone());
            } else 
            { // no tokens
              token.tokens = Some( vec![savedToken.clone()] );
            }
          }

          //
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
      // add nesting
      let savedToken: Token = tokens.remove(l);
      if let Some(token) = tokens.get_mut(brackets[brackets.len()-1]) 
      {
        if let Some(tokenTokens) = &mut token.tokens 
        { // contains tokens 
          tokenTokens.push(savedToken.clone());
        } else 
        { // no tokens
          token.tokens = Some( vec![savedToken.clone()] );
        }
      }

      //
      length -= 1;
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
        match &mut currentLine.lines 
        {
          Some(lineLines) => 
          { 
            lineLines.push(nestingLineLink); // nesting
            lineNesting(lineLines);          // cycle
          },
          None => 
          { 
            currentLine.lines = Some(vec![nestingLineLink]);  // nesting
            lineNesting(currentLine.lines.as_mut().unwrap()); // cycle
          }
        }
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
fn setLineNestingNums(linesLinks: &mut Vec< Arc<RwLock<Line>> >) 
{
  for (i, lineLink) in linesLinks.iter().enumerate() 
  {
    let mut line = lineLink.write().unwrap();
    line.index = i;
    if let Some(ref mut lineLines) = line.lines
    {
      setLineNestingNums(lineLines);
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
      if let Some(ref mut lineLines) = line.lines
      {
        deleteDoubleComment(lineLines, index);
      }
      // skip separator
      if line.tokens.is_empty() {
        break 'exit;
      }
      // ? delete comment
      lastTokenIndex = line.tokens.len()-1;
      if line.tokens[lastTokenIndex].getDataType().unwrap_or_default() == TokenType::Comment {
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
pub unsafe fn outputTokens(tokens: &Vec<Token>, lineIndent: usize, indent: usize) -> ()
{
  let lineIndentString: String = " ".repeat(lineIndent*2+1);
  let identString:      String = " ".repeat(indent*2+1);

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

    if let Some(tokenData) = token.getData()
    {
    // single quote
      if token.getDataType().unwrap_or_default() == TokenType::Char || 
         token.getDataType().unwrap_or_default() == TokenType::FormattedChar {
        log("parserToken",&format!(
          "{}{}{}\\fg(#f0f8ff)\\b'\\c{}\\fg(#f0f8ff)\\b'\\c  |{}",
          lineIndentString,
          c,
          identString,
          tokenData,
          token.getDataType().unwrap_or_default().to_string()
        ));
    // double quote
      } else
      if token.getDataType().unwrap_or_default() == TokenType::String || 
         token.getDataType().unwrap_or_default() == TokenType::FormattedString {
        log("parserToken",&format!(
          "{}{}{}\\fg(#f0f8ff)\\b\"\\c{}\\fg(#f0f8ff)\\b\"\\c  |{}",
          lineIndentString,
          c,
          identString,
          tokenData,
          token.getDataType().unwrap_or_default().to_string()
        ));
    // back quote
      } else
      if token.getDataType().unwrap_or_default() == TokenType::RawString || 
         token.getDataType().unwrap_or_default() == TokenType::FormattedRawString {
        log("parserToken",&format!(
          "{}{}{}\\fg(#f0f8ff)\\b`\\c{}\\fg(#f0f8ff)\\b`\\c  |{}",
          lineIndentString,
          c,
          identString,
          tokenData,
          token.getDataType().unwrap_or_default().to_string()
        ));
    // basic
      } else {
        log("parserToken",&format!(
          "{}{}{}{}  |{}",
          lineIndentString,
          c,
          identString,
          tokenData,
          token.getDataType().unwrap_or_default().to_string()
        ));
      }
    // type only
    } else {
      formatPrint(&format!(
        "{}{}{}{}\n",
        lineIndentString,
        c,
        identString,
        token.getDataType().unwrap_or_default().to_string()
      ));
    }
    if let Some(tokens) = &token.tokens
    {
      outputTokens(tokens, lineIndent, indent+1)
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
    let line: RwLockReadGuard<'_, Line> = line.read().unwrap();
    log("parserBegin", &format!("{} {}",identStr1,i));

    if (&line.tokens).len() == 0 
    {
      formatPrint(&format!("{}\\b┗ \\fg(#90df91)Separator\\c\n",identStr2));
    } else 
    {
      formatPrint(&format!("{}\\b┣ \\fg(#90df91)Tokens\\c\n",identStr2));
    }

    outputTokens(&line.tokens, indent, 1);
    if let Some(lineLines) = &line.lines
    {
      formatPrint(&format!("{}\\b┗ \\fg(#90df91)Lines\\c\n",identStr2));
      outputLines(lineLines, indent+1);
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
  let mut lineIndent: usize = 0;
  let mut lineTokens: Vec<Token> = Vec::new();

  let startTime: Instant = Instant::now();

  let mut linesLinks:     Vec< Arc<RwLock<Line>> > = Vec::new();
  let mut readLineIndent: bool                     = true;

  while index < bufferLength 
  {
    let byte: u8 = buffer[index]; // current char

    // indent
    if byte == b' ' && readLineIndent 
    {
      index += 1;
      lineIndent += 1;
    } else 
    {
      readLineIndent = false;
      // get endline
      if byte == b'\n' || byte == b';' 
      {
        // bracket nesting
        bracketNesting(
          &mut lineTokens,
          &TokenType::CircleBracketBegin, 
          &TokenType::CircleBracketEnd
        );
        bracketNesting(
          &mut lineTokens,
          &TokenType::SquareBracketBegin, 
          &TokenType::SquareBracketEnd
        );
        /*
        bracketNesting(
          &mut lineTokens,
          &TokenType::FigureBracketBegin, 
          &TokenType::FigureBracketEnd
        );
        */

        // add new line
        linesLinks.push( 
          Arc::new(RwLock::new( 
            Line {
              tokens: lineTokens.clone(),
              indent: lineIndent,
              index:  0,
              lines:  None,
              parent: None
            }
          ))
        );
        lineIndent = 0;

        readLineIndent = true;
        lineTokens.clear();
        index += 1;
      } else
      // delete comment
      if byte == b'#' 
      {
        deleteComment(&buffer, &mut index, bufferLength);
        lineTokens.push( Token::newEmpty( Some(TokenType::Comment) ) );
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
        if token.getDataType() != None 
        { // if formatted quotes
          let lineTokensLength: usize = lineTokens.len();
          if lineTokensLength > 0 
          {
              let backToken = &lineTokens[lineTokensLength-1];
              if backToken.getDataType().unwrap_or_default() == TokenType::Word && 
                 backToken.getData().unwrap_or_default() == "f" 
              {
                let newDataType: TokenType = token.getDataType().unwrap_or_default();
                if newDataType == TokenType::RawString 
                {
                 token.setDataType( Some(TokenType::FormattedRawString) ); 
                } else
                if newDataType == TokenType::String 
                { 
                  token.setDataType( Some(TokenType::FormattedString) ); 
                } else
                if newDataType == TokenType::Char 
                { 
                  token.setDataType( Some(TokenType::FormattedChar) ); 
                }
                lineTokens[lineTokensLength-1] = token; // replace the last token in place
              } else 
              { // basic quote
                lineTokens.push(token);
              }
          } else 
          { // basic quote
            lineTokens.push(token);
          }
        } else 
        { // skip
          index += 1;
        }
      } else
      // get single and double chars
      if isSingleChar(byte) 
      {
        let token: Token = getOperator(&buffer, &mut index, bufferLength);
        if token.getDataType() != None
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
    let endTime:  Instant  = Instant::now();
    let duration: Duration = endTime-startTime;
    // lines
    outputLines(&linesLinks,2);
    //
    println!("     ┃");
    log("ok",&format!("xDuration: {:?}",duration));
  }
  setLineNestingNums(&mut linesLinks); // set correct line nums
  linesLinks
}