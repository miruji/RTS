/* /tokenizer
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

// проверяет buffer по index и так пропускае возможные комментарии
// потом они будут удалены по меткам
unsafe fn deleteComment(buffer: &[u8], index: &mut usize, bufferLength: usize) -> ()
{
  *index += 1;
  while *index < bufferLength && buffer[*index] != b'\n' 
  {
    *index += 1;
  }
}

// проверяет что байт является одиночным знаком;
// доступным для синтаксиса
fn isSingleChar(c: u8) -> bool {
  matches!(c, 
    b'+' | b'-' | b'*' | b'/' | b'=' | b'%' | b'^' |
    b'>' | b'<' | b'?' | b'!' | b'&' | b'|' | 
    b'(' | b')' | b'{' | b'}' | b'[' | b']' | 
    b':' | b',' | b'.' | b'~'
  )
}

// проверяет что байт является числом
fn isDigit(c: u8) -> bool 
{
  c >= b'0' && c <= b'9'
}
// проверяет buffer по index и так находит возможные 
// примитивные численные типы данных;
// e: UInt, Int, UFloat, Float, Rational, Complex
// todo: ввести Complex числа
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

    // todo: use match case
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
      result.push_str("//");
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
    (true, _, _)     => Token::new( Some(TokenType::Rational), Some(result) ),
    (_, true, true)  => Token::new( Some(TokenType::Float),    Some(result) ),
    (_, true, false) => Token::new( Some(TokenType::UFloat),   Some(result) ),
    (_, false, true) => Token::new( Some(TokenType::Int),      Some(result) ),
    _                => Token::new( Some(TokenType::UInt),     Some(result) ),
  }
}

// проверяет что байт является буквой a-z A-Z
fn isLetter(c: u8) -> bool 
{
  (c|32)>=b'a'&&(c|32)<=b'z'
}
// проверяет buffer по index и так находит возможные слова;
// из них также выделяет сразу определяемые зарезервированные
unsafe fn getWord(buffer: &[u8], index: &mut usize, bufferLength: usize) -> Token 
{
  let mut savedIndex: usize = *index; // index buffer
  let mut result: String = String::new();
  let mut isLink: bool = false;

  while savedIndex < bufferLength 
  {
    let byte1: u8 = buffer[savedIndex]; // current char

    // todo: use match case
    if (isDigit(byte1) || byte1 == b'.') && !result.is_empty()
    {
      result.push(byte1 as char);
      savedIndex += 1;
      if byte1 == b'.' { isLink = true; } // только если есть . то мы знаем что это ссылка
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
    match result.as_str()
    {
      "true"     => Token::new( Some(TokenType::Bool), Some(String::from("1")) ),
      "false"    => Token::new( Some(TokenType::Bool), Some(String::from("0")) ),
      _          => Token::new( Some(TokenType::Word), Some(result) ),
    }
  }
}

// проверяет buffer по index и так находит возможные 
// Char, String, RawString
unsafe fn getQuotes(buffer: &[u8], index: &mut usize,) -> Token 
{
  let byte1: u8 = buffer[*index]; // quote byte
  let mut result = String::new();

  if buffer[*index] == byte1 
  {
    let mut open:             bool = false;
    let mut noSlash:          bool;
    let mut backslashCounter: usize;

    let length: usize = buffer.len();
    while *index < length 
    {
      let byte2: u8 = buffer[*index]; // current byte

      // check endline error
      if byte2 == b'\n' 
      {
        // quotes were not closed
        // skipped it!
        return Token::newEmpty(None);
      }

      // read quote
      // todo: use match case ?
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
      Token::new( Some(TokenType::Char), Some(result) )
    }
  } else if byte1 == b'"' 
  {
    Token::new( Some(TokenType::String), Some(result) )
  } else if byte1 == b'`' 
  {
    Token::new( Some(TokenType::RawString), Some(result) )
  } else 
  {
    Token::newEmpty(None)
  }
}

// проверяет buffer по index и так находит возможные 
// двойные и одиночные операторы
unsafe fn getOperator(buffer: &[u8], index: &mut usize, bufferLength: usize) -> Token 
{
  let currentByte: u8 = buffer[*index]; // current byte
  let nextByte: u8 =                    // next byte or \0
    if *index+1 < bufferLength 
    { 
      buffer[*index+1]
    } else 
    { 
      b'\0'
    };

  let mut increment = |count: usize| 
  { // index increment for single & duble operators
    *index += count;
  };

  match currentByte 
  {
    b'+' => 
    {
           if nextByte == b'=' 
        { increment(2); Token::newEmpty( Some(TokenType::PlusEquals) ) }
      else if nextByte == b'+' 
        { increment(2); Token::newEmpty( Some(TokenType::UnaryPlus) ) }
      else 
        { increment(1); Token::newEmpty( Some(TokenType::Plus) ) }
    }
    b'-' => 
    {
           if nextByte == b'=' 
        { increment(2); Token::newEmpty( Some(TokenType::MinusEquals) ) }
      else if nextByte == b'-' 
        { increment(2); Token::newEmpty( Some(TokenType::UnaryMinus) ) }
      else if nextByte == b'>' 
        { increment(2); Token::newEmpty( Some(TokenType::Pointer) ) }
      else 
        { increment(1); Token::newEmpty( Some(TokenType::Minus) ) }
    }
    b'*' => 
    {
           if nextByte == b'=' 
        { increment(2); Token::newEmpty( Some(TokenType::MultiplyEquals) ) }
      else if nextByte == b'*' 
        { increment(2); Token::newEmpty( Some(TokenType::UnaryMultiply) ) }
      else 
        { increment(1); Token::newEmpty( Some(TokenType::Multiply) ) }
    }
    b'/' => 
    {
           if nextByte == b'=' 
        { increment(2); Token::newEmpty( Some(TokenType::DivideEquals) ) }
      else if nextByte == b'/' 
        { increment(2); Token::newEmpty( Some(TokenType::UnaryDivide) ) }
      else 
        { increment(1); Token::newEmpty( Some(TokenType::Divide) ) }
    }
    b'%' => 
    {
           if nextByte == b'=' 
        { increment(2); Token::newEmpty( Some(TokenType::Modulo) ) } // todo: add new type in Token
      else if nextByte == b'%' 
        { increment(2); Token::newEmpty( Some(TokenType::Modulo) ) } // todo: add new type in Token
      else 
        { increment(1); Token::newEmpty( Some(TokenType::Modulo) ) }
    }
    b'^' => 
    {
           if nextByte == b'=' 
        { increment(2); Token::newEmpty( Some(TokenType::Exponent) ) } // todo: add new type in Token
      else if nextByte == b'^' 
        { increment(2); Token::newEmpty( Some(TokenType::Exponent) ) } // todo: add new type in Token
      else 
        { increment(1); Token::newEmpty( Some(TokenType::Disjoint) ) }
    }
    b'>' => 
    {
      if nextByte == b'=' 
        { increment(2); Token::newEmpty( Some(TokenType::GreaterThanOrEquals) ) }
      else 
        { increment(1); Token::newEmpty( Some(TokenType::GreaterThan) ) }
    }
    b'<' => 
    {
      if nextByte == b'=' 
        { increment(2); Token::newEmpty( Some(TokenType::LessThanOrEquals) ) }
      else 
        { increment(1); Token::newEmpty( Some(TokenType::LessThan) ) }
    }
    b'!' => 
    {
      if nextByte == b'=' 
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

// основная функция, которая вкладывает токены в скобки
// e: () [] {}
// от начальной скобки до закрывающей
// её особенность в рекурсивном вызове себя для дочерних токенов
unsafe fn bracketNesting(tokens: &mut Vec<Token>, beginType: &TokenType, endType: &TokenType) -> ()
{
  for token in tokens.iter_mut() 
  { // чтение токенов
    if let Some(ref mut tokens) = token.tokens 
    { // рекурсия
      bracketNesting(tokens, beginType, endType);
    }
  }
  // вкладывание
  blockNesting(tokens, beginType, endType);
}
// эта функция является дочерней bracketNesting 
// и занимается только самим вложением токенов 
// от начальной скобки до закрывающей
unsafe fn blockNesting(tokens: &mut Vec<Token>, beginType: &TokenType, endType: &TokenType) -> ()
{
  let mut brackets: Vec::<usize> = Vec::new();   // nested brackets
  let mut tokensLength: usize    = tokens.len(); // tokens length

  let mut i: usize = 0; // index buffer
  while i < tokensLength 
  { // read tokens
    let tokenType: &TokenType = &tokens[i].getDataType().unwrap_or_default();
    // compare type
    if tokenType == beginType 
    { // if this is the first token
      brackets.push(i);
    } else 
    if tokenType == endType 
    { // if this is the last token
      if let Some(lastBracket) = brackets.pop() 
      { // then delete last bracket
        if !brackets.is_empty() 
        { // add new nesting
          let savedToken: Token = tokens[lastBracket].clone(); // last token (bracket)
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

          // remove last token (bracket)
          tokens.remove(lastBracket);
          tokensLength -= 1;

          if lastBracket < i { i -= 1; }
        }
      }

      // remove begin token (bracket)
      tokens.remove(i);
      tokensLength -= 1;
      continue;
    } else 
    if !brackets.is_empty() 
    { // nesting tokens to bracket begin
      let savedToken: Token = tokens.remove(i);
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

      // go to next token
      tokensLength -= 1;
      continue;
    }
    i += 1; // continue
  }
}
// вкладывает линии токенов друг в друга
fn lineNesting(linesLinks: &mut Vec< Arc<RwLock<Line>> >) -> ()
{
  let mut index:     usize = 0;                // current line index
  let mut nextIndex: usize = 1;                // next    line index
  let mut length:    usize = linesLinks.len(); // all lines links length

  while index < length && nextIndex < length 
  { // если мы не дошли до конца, то читаем линии
    if // compare current indent < next indent
      linesLinks[index].read().unwrap()
        .indent < 
      linesLinks[nextIndex].read().unwrap()
        .indent
    {
      // get next line and remove
      let nestingLineLink: Arc<RwLock<Line>> = linesLinks.remove(nextIndex);
      length -= 1;
      { // set parent line link
        nestingLineLink.write().unwrap()
          .parent = Some( linesLinks[index].clone() );
      }
      // push nesting
      let mut currentLine: RwLockWriteGuard<'_, Line> = linesLinks[index].write().unwrap();
      match &mut currentLine.lines 
      {
        Some(lineLines) => 
        { // если вложения уже были, то просто делаем push
          lineLines.push(nestingLineLink); // nesting
          lineNesting(lineLines);          // cycle
        },
        None => 
        { // если вложения не было до этого, то создаём
          currentLine.lines = Some(vec![nestingLineLink]);  // nesting
          lineNesting(currentLine.lines.as_mut().unwrap()); // cycle
        }
      }
    } else 
    {
      index += 1;
      nextIndex = index+1;
    }
  }
}

// удаляет возможные вложенные комментарии по меткам;
// это такие комментарии, которые имеют вложения
unsafe fn deleteNestedComment(linesLinks: &mut Vec< Arc<RwLock<Line>> >, mut index: usize) -> ()
{
  let mut linesLinksLength: usize = linesLinks.len(); // количество ссылок строк
  let mut lastTokenIndex:   usize;                    // это указатель на метку где TokenType::Comment

  while index < linesLinksLength 
  {
    let mut deleteLine: bool  = false; // состояние удаления текущей линии
    'exit: 
    { // прерывание чтобы не нарушать мутабельность
      let mut line: RwLockWriteGuard<'_, Line> = linesLinks[index].write().unwrap();
      if let Some(ref mut lineLines) = line.lines
      { // рекурсивно обрабатываем вложенные линии
        deleteNestedComment(lineLines, index);
      }
      // пропускаем разделители, они нужны для синтаксиса
      if line.tokens.is_empty() { // todo: разделители стоит объединять в один если они идут подряд
        break 'exit;
      }
      // комментарии удаляем
      lastTokenIndex = line.tokens.len()-1; // todo: после того как разделители объединены в 1;
                                            //       если разделитель зажат между комментариями,
                                            //       то это один большой комментарий
      if line.tokens[lastTokenIndex].getDataType().unwrap_or_default() == TokenType::Comment {
        line.tokens.remove(lastTokenIndex);
        if line.tokens.is_empty() { // переходим к удалению пустой линии
          deleteLine = true;        // линия была удалена
          break 'exit;              // выходим из прерывания
        }
      }
    }
    // когда линия удалена в прерывании, 
    // её можно спокойно удалить
    if deleteLine 
    {
      linesLinks.remove(index);
      linesLinksLength -= 1;
      continue;
    }
    // продолжаем чтение
    index += 1;
  }
}

// выводит токен, его тип данных
pub unsafe fn outputTokens(tokens: &Vec<Token>, lineIndent: usize, indent: usize) -> ()
{
  let lineIndentString: String = " ".repeat(lineIndent*2+1); // отступ для линии
  let identString:      String = " ".repeat(indent*2+1);     // отступ для вложения токенов

  let tokenCount: usize = tokens.len()-1;
  let mut c: char;

  let mut tokenType: TokenType;
  for (i, token) in tokens.iter().enumerate() 
  { // читаем все токены

    // слева помечаем что это за токен;
    // в случае с X это завершающий токен
    c = 
      if i == tokenCount 
      {
        'X'
      } else 
      {
        '┃'
      };

    tokenType = token.getDataType().unwrap_or_default(); // тип токена
    if let Some(tokenData) = token.getData()
    { // проверяем что за токен
      if matches!(tokenType, TokenType::Char | TokenType::FormattedChar) 
      { // если токен это Char | FormattedChar
        log("parserToken",&format!(
          "{}{}{}\\fg(#f0f8ff)\\b'\\c{}\\fg(#f0f8ff)\\b'\\c  |{}",
          lineIndentString,
          c,
          identString,
          tokenData,
          tokenType.to_string()
        ));
      } else
      if matches!(tokenType, TokenType::String | TokenType::FormattedString) 
      { // если токен это String | FormattedString
        log("parserToken",&format!(
          "{}{}{}\\fg(#f0f8ff)\\b\"\\c{}\\fg(#f0f8ff)\\b\"\\c  |{}",
          lineIndentString,
          c,
          identString,
          tokenData,
          tokenType.to_string()
        ));
      } else
      if matches!(tokenType, TokenType::RawString | TokenType::FormattedRawString) 
      { // если токен это RawString | FormattedRawString
        log("parserToken",&format!(
          "{}{}{}\\fg(#f0f8ff)\\b`\\c{}\\fg(#f0f8ff)\\b`\\c  |{}",
          lineIndentString,
          c,
          identString,
          tokenData,
          tokenType.to_string()
        ));
      } else 
      { // если это обычный токен
        log("parserToken",&format!(
          "{}{}{}{}  |{}",
          lineIndentString,
          c,
          identString,
          tokenData,
          tokenType.to_string()
        ));
      }
    } else 
    { // если это токен только с типом, то выводим тип как символ
      formatPrint(&format!(
        "{}{}{}{}\n",
        lineIndentString,
        c,
        identString,
        tokenType.to_string()
      ));
    }

    // если есть вложения у токена, то просто рекурсивно обрабатываем их
    if let Some(tokens) = &token.tokens
    {
      outputTokens(tokens, lineIndent, indent+1)
    }
  }
}
// выводит информацию о линии;
// также токены линии
pub unsafe fn outputLines(linesLinks: &Vec< Arc<RwLock<Line>> >, indent: usize) -> ()
{
  let identStr1: String = " ".repeat(indent*2);      // это отступ для главной строки
  let identStr2: String = format!("{} ", identStr1); // а это для дочерних токенов

  for (i, line) in linesLinks.iter().enumerate() 
  { // проходи по линиям через чтение
    let line: RwLockReadGuard<'_, Line> = line.read().unwrap();
    log("parserBegin", &format!("{} {}",identStr1,i));

    if (&line.tokens).len() == 0 
    { // заголовок для разделителей
      formatPrint(&format!("{}\\b┗ \\fg(#90df91)Separator\\c\n",identStr2));
    } else 
    { // заголовок для начала вложенных токенов
      formatPrint(&format!("{}\\b┣ \\fg(#90df91)Tokens\\c\n",identStr2));
      outputTokens(&line.tokens, indent, 1); // выводим вложенные токены
    }
    
    if let Some(lineLines) = &line.lines
    { // заголовок для начала вложенных линий
      formatPrint(&format!("{}\\b┗ \\fg(#90df91)Lines\\c\n",identStr2));
      outputLines(lineLines, indent+1); // выводим вложенные линии
    }
  }
  //
}

// основная функция для чтения токенов и получения чистых линий из них;
// токены в этот момент не только сгруппированы в линии, но и имеют 
// предварительные базовые типы данных.
pub unsafe fn readTokens(buffer: Vec<u8>, debugMode: bool) -> Vec< Arc<RwLock<Line>> > 
{
  if debugMode 
  {
    logSeparator("AST");
    log("ok","+Generation");
    println!("     ┃");
  }

  let mut      index: usize = 0;               // основной индекс чтения
  let   bufferLength: usize = buffer.len();    // размер буфера байтов
  let mut lineIndent: usize = 0;               // текущий отступ линии
  let mut lineTokens: Vec<Token> = Vec::new(); // прочитанные токены текущей линии

  let startTime: Instant = Instant::now(); // замеряем текущее время для debug

  let mut linesLinks:     Vec< Arc<RwLock<Line>> > = Vec::new(); // это ссылки на готовые линии 
  let mut readLineIndent: bool                     = true;       // флаг на проверку есть ли indent сейчас

  while index < bufferLength 
  { // читаем байты
    let byte: u8 = buffer[index]; // текущий байт

    // проверяем отступы, они могут быть указаны пробелами;
    // либо readLineIndent будет true после конца строки предыдущей линии
    if byte == b' ' && readLineIndent 
    {
      index += 1;
      lineIndent += 1;
    } else 
    {
      readLineIndent = false;
      // смотрим является ли это endline
      if byte == b'\n' || byte == b';' 
      { // если это действительно конец строки,
        // то вкладываем возможные скобки
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
        // FigureBracketBegin и FigureBracketEnd
        // это остаётся всё ещё здесь только потому,
        // что может быть нужным для реализации использования
        // подобных структур:
        /*
          for x(args: <Token>) -> None
            args[0]
            ? args[1]
              {}
              args[2]
              go(1)

          for i = 0, i < 10, i++
            println(10)
        */
        // здесь наглядно видно, что for функция будет запущена
        // только когда дойдёт до самого конца вложения,
        // после чего {} позволит запустить всё вложение.
        // а при необходимости мы бы могли обращаться к вложению,
        // например: {}.0 или {}[0] ...
        // поэтому эта тема требует отдельных тестов.
        /*
        bracketNesting(
          &mut lineTokens,
          &TokenType::FigureBracketBegin, 
          &TokenType::FigureBracketEnd
        );
        */

        // добавляем новую линию и пушим ссылку на неё
        linesLinks.push( 
          Arc::new(RwLock::new( 
            Line {
              tokens: std::mem::take(&mut lineTokens), // забираем все токены в линию, 
                                                       // оставляя пустой вектор для следующей
              indent: lineIndent,
              lines:  None, // в данный момент у неё нет вложенных линий, это будет чуть ниже
              parent: None  // также у неё нет родителя, это тоже будет ниже при вложении
            }
          ))
        );
        lineIndent = 0;

        readLineIndent = true; // это был конец строки
        index += 1;
      } else
      if byte == b'#' 
      { // ставим метку на комментарий в линии, по ним потом будут удалены линии
        deleteComment(&buffer, &mut index, bufferLength); // пропускает комментарий
        lineTokens.push( Token::newEmpty( Some(TokenType::Comment) ) );
      } else
      if isDigit(byte) || (byte == b'-' && index+1 < bufferLength && isDigit(buffer[index+1])) 
      { // получаем все возможные численные примитивные типы данных
        lineTokens.push( getNumber(&buffer, &mut index, bufferLength) );
      } else
      if isLetter(byte) 
      { // получаем все возможные и зарезервированные слова
        lineTokens.push( getWord(&buffer, &mut index, bufferLength) );
      } else
      if byte == b'\'' || byte == b'"' || byte == b'`' 
      { // получаем Char, String, RawString
        let mut token: Token = getQuotes(&buffer, &mut index);
        if token.getDataType() != None 
        { // if formatted quotes
          let lineTokensLength: usize = lineTokens.len();
          if lineTokensLength > 0 
          {
              let backToken: &Token = &lineTokens[lineTokensLength-1];
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
      // получаем возможные двойные и одиночные символы
      if isSingleChar(byte) 
      {
        let token: Token = getOperator(&buffer, &mut index, bufferLength);
        if token.getDataType() != None
        {
            lineTokens.push(token);
        } else 
        {
          index += 1;
        }
      } else 
      { // если мы ничего не нашли из возможного, значит этого нет в синтаксисе;
        // поэтому просто идём дальше
        index += 1;
      }
    }
  }

  // вкладываем линии
  lineNesting(&mut linesLinks);
  // удаляем возможные вложенные комментарии по меткам
  deleteNestedComment(&mut linesLinks, 0);

  // debug output and return
  if debugMode 
  {
    let endTime:  Instant  = Instant::now();    // получаем текущее время
    let duration: Duration = endTime-startTime; // получаем сколько всего прошло
    outputLines(&linesLinks,2); // выводим полученное AST дерево из линий
    //
    println!("     ┃");
    log("ok",&format!("xDuration: {:?}",duration));
  }
  // возвращаем готовые ссылки на линии
  linesLinks
}