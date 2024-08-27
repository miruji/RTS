/*
  parser
*/

use crate::logger::*;
use crate::_argc;
use crate::_argv;
use crate::_debugMode;
use crate::_exitCode;

pub mod memoryCell;     use crate::parser::memoryCell::*;
pub mod memoryCellList; use crate::parser::memoryCellList::*;

pub mod value;
pub mod uf64;
pub mod class;  use crate::parser::class::*;
pub mod r#enum; use crate::parser::r#enum::*;
pub mod method; use crate::parser::method::*;
pub mod list;   use crate::parser::list::*;

use crate::tokenizer::*;
use crate::tokenizer::token::*;
use crate::tokenizer::line::*;

use std::time::Instant;

use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::ptr::addr_of_mut;

// check memory cell type
fn checkMemoryCellType(dataType: TokenType) -> bool 
{
  return 
    if dataType == TokenType::Int    || 
       dataType == TokenType::UInt   || 
       dataType == TokenType::Float  || 
       dataType == TokenType::UFloat || 
       dataType == TokenType::Rational
    {
      true
      // todo: complex number
      // and other types
    } else {
      false
    }
}
// check operator
fn checkMemoryCellMathOperator(dataType: TokenType) -> bool 
{
  if dataType == TokenType::Equals         || // =

     dataType == TokenType::UnaryPlus      || // ++
     dataType == TokenType::PlusEquals     || // +=

     dataType == TokenType::UnaryMinus     || // --
     dataType == TokenType::MinusEquals    || // -=

     dataType == TokenType::UnaryMultiply  || // **
     dataType == TokenType::MultiplyEquals || // *=

     dataType == TokenType::UnaryDivide    || // //
     dataType == TokenType::DivideEquals   || // /=

     dataType == TokenType::UnaryModulo    || // %%
     dataType == TokenType::ModuloEquals   || // %=

     dataType == TokenType::UnaryExponent  || // ^^
     dataType == TokenType::ExponentEquals    // ^=
  {
    true
  } else 
  {
    false
  }
}
/*
// define upper struct [Class / Enum]
unsafe fn defineUpperStruct(classes: &mut Vec<Class>, enums: &mut Vec<Enum>) {
  let mut i:   usize = 0;
  let mut add: bool = true;

  let mut line: Line;

  let mut constr: bool;

  while i < _lines.len() { // todo: add linesLength and -= 1
      line = _lines[i].clone();

      for token in &line.tokens {
      if let Some(firstChar) = token.data.chars().next() {
          // fist char upper
          if firstChar.is_ascii_uppercase() {
              // Class or Enum ?
              // => read this line.lines
              constr = false;
              'outer: {
                  if !constr && line.lines.len() > 0 {
                  for childLine in &line.lines {
                      for searchConstr in &childLine.tokens {
                          if searchConstr.data == "constr" {
                              classes.push( Class::new(token.data.clone(), i, line.lines.clone()) );
                              _lines.remove(i);
                              add = false;

                              constr = true;
                              break 'outer;
                          }
                      }
                  } }
              }
              // else type = enum
              if !constr && !line.lines.is_empty() {
                  enums.push( Enum::new(token.data.clone(), i, line.lines.clone()) );
                  _lines.remove(i);
                  add = false;
              }
              break;
              //
          }
      } }
      if !add {
          add = true;
      } else {
          i += 1;
      }
  }
  //
}
// define lower struct [function / procedure / list]
unsafe fn defineLowerStruct(methods: &mut Vec<Method>, lists: &mut Vec<List>) {
  let mut i:   usize = 0;
  let mut add: bool = true;

  let mut line: Line;
  let mut lineTokensLength: usize;

  let mut list: bool;

  let mut token: &Token;

  while i < _lines.len() { // todo: add linesLength and -= 1
      line = _lines[i].clone();
      lineTokensLength = line.tokens.len();
      if lineTokensLength == 0 {
          i += 1;
          continue;
      }

      token = &line.tokens[0];
      if let Some(firstChar) = token.data.chars().next() {
          // fist char upper
          if firstChar.is_ascii_lowercase() {
              // method or list ?
              // => read this line.lines
              list = false;
              'outer: {
                  if !list && line.lines.len() > 0 {
                  for childLine in &line.lines {
                      for searchKey in &childLine.tokens {
                          if searchKey.dataType == TokenType::String {
                              lists.push( List::new(token.data.clone(), i, line.lines.clone()) );
                              _lines.remove(i);
                              add = false;

                              list = true;
                              break 'outer;
                          }
                          //
                      }
                  } }
              }
              // else type = method
              // to:do: method(parameters) -> result
              if !list && !line.lines.is_empty() {
                  // min = 1, max = 4 tokens:
                  // name
                  if lineTokensLength == 1 {
                      methods.push(
                          Method::new(
                              token.data.clone(),
                              i,
                              line.lines.clone()
                          )
                      );
                  } else
                  if lineTokensLength < 4 {
                      // name(param)
                      if line.tokens[1].dataType == TokenType::CircleBracketBegin {
                          methods.push(
                              Method::newWithParameters(
                                  token.data.clone(),
                                  i,
                                  line.lines.clone(),
                                  line.tokens[1].tokens.clone()
                              )
                          );
                      } else
                      // name -> Type
                      if line.tokens[1].dataType == TokenType::Pointer {
                          methods.push( 
                              Method::newWithResult(
                                  token.data.clone(),
                                  i,
                                  line.lines.clone(),
                                  line.tokens[2].dataType.to_string()
                              )
                          );
                      }
                  } else
                  // name(param) -> Type
                  if lineTokensLength == 4 {
                      methods.push(
                          Method::newFull(
                              token.data.clone(),
                              i,
                              line.lines.clone(),
                              line.tokens[1].tokens.clone(),
                              line.tokens[3].dataType.to_string()
                          )
                      );
                  } else {
                  // read error
                      // to:do no working here
                      log("syntax","");
                      log("path",&format!(
                          "{} -> Method \"{}\"",
                          unsafe{&*_filePath},
                          line.tokens[0].data
                      ));
                      log("note","Maximum number of instructions when declaring a procedure is 3;");
                      log("note","Maximum number of instructions when declaring a function is 4.");
                      logExit();
                  }
                  
                  _lines.remove(i);
                  add = false;
              }
              //break;
              //
          }
      }
      if !add {
          add = true;
      } else {
          i += 1;
      }
  }
  //
}
*/
/* search condition
 e:
   ? condition
     block
   ? condition
     block
*/
unsafe fn searchCondition(lineLink: Arc<RwLock<Line>>, methodLink: Arc<RwLock<Method>>) -> bool 
{
  // todo: delete lineIndex and use methodLink.lines ?
  let mut conditions: Vec< Arc<RwLock<Line>> > = Vec::new();
  { // get conditions
    let      method: RwLockReadGuard<'_, Method> = methodLink.read().unwrap();
    let       lines: &Vec< Arc<RwLock<Line>> >   = &method.lines;
    let linesLength: usize                       = lines.len();
    { // search bottom lines
      let mut i: usize =
        {
          let line: RwLockReadGuard<'_, Line> = lineLink.read().unwrap();
          line.index
        };
      // if line index < lines length
      while i < linesLength 
      {
        // check question
        let lineBottomLink: Arc<RwLock<Line>> = lines[i].clone();
        { // check question token
          let bottomLine: RwLockReadGuard<'_, Line> = lineBottomLink.read().unwrap();
          // skip empty line
          if bottomLine.tokens.len() == 0 { break; } else
          // check question
          if bottomLine.tokens[0].getDataType().unwrap_or(TokenType::None) != TokenType::Question { break; }
        }
        conditions.push(lineBottomLink);
        i += 1;
      }
    }
    // if no conditions
    if conditions.len() == 0 { return false; }
  }
  // read conditions
  let mut conditionTruth: bool = false;
  for conditionLink in &mut conditions 
  {
    let condition = conditionLink.read().unwrap();
    // if elif
    if condition.tokens.len() != 0 
    {
      { // check condition truth and unlock mcl
        let method = methodLink.read().unwrap();
        let mut conditionTokens = condition.tokens.clone(); // todo: no clone ? fix its please
        conditionTokens.remove(0);
        conditionTruth = 
          {
            let expressionResult = method.memoryCellExpression(&mut conditionTokens).getData(); // todo: type
            if let Some(expressionResult) = expressionResult 
            {
              expressionResult == "true"
            } else 
            {
              false
            }
          }
      }
      if conditionTruth 
      {
        // new temporary method
        let mut conditionLinesLength: usize = condition.lines.len();
        let mut conditionLineIndex:   usize = 0;
        let method = 
          Arc::new(
          RwLock::new(
            Method::new(
              String::from("if-el"),
              condition.lines.clone(),
              Some(methodLink.clone())
            )
          ));
        readLines(method, &mut conditionLineIndex, &mut conditionLinesLength);
        break; // end
      }
    // else
    } else
    if !conditionTruth 
    {
      // new temporary method
      let mut conditionLinesLength: usize = condition.lines.len();
      let mut conditionLineIndex:   usize = 0;
      let method = 
        Arc::new(
        RwLock::new(
          Method::new(
            String::from("else"),
            condition.lines.clone(),
            Some(methodLink.clone())
          )
        ));
      readLines(method, &mut conditionLineIndex, &mut conditionLinesLength);
      break; // end
    }
  }
  return true;
}
// return [function only]
  // e:  = value
unsafe fn searchReturn(lineLink: Arc<RwLock<Line>>, methodLink: Arc<RwLock<Method>>) -> bool 
{
  let line = lineLink.read().unwrap();
  let mut lineTokens: Vec<Token> = line.tokens.clone();
  let mut method = methodLink.write().unwrap();

  if lineTokens[0].getDataType().unwrap_or(TokenType::None) == TokenType::Equals 
  {
    lineTokens.remove(0);

    let methodResultType = 
      if let Some(methodResult) = &method.result 
      {
        methodResult.getDataType().clone()
      } else {
        Some(TokenType::None)
      };
    method.result = Some(method.memoryCellExpression(&mut lineTokens));
    if let Some(methodResult) = &mut method.result 
    {
      methodResult.setDataType( methodResultType );
    }

    return true;
  }
  return false;
}
// define methods [function / procedure]
//   min = 1, max = 4 tokens:
//   name
//   name(param)
//   name -> Type
//   name(param) -> Type
unsafe fn searchMethod(lineLink: Arc<RwLock<Line>>, methodLink: Arc<RwLock<Method>>) -> bool 
{
  let line = lineLink.read().unwrap(); // todo: add type

  let lineTokens: &Vec<Token> = &line.tokens;
  let lineTokensLength: usize = lineTokens.len();

  //
  let mut parameters: Option< Vec<Token> > = None;
  if lineTokens[0].getDataType().unwrap_or(TokenType::None) == TokenType::Word && line.lines.len() > 0 
  { // if there are parameters
    let mut newMethodResultType: Option<TokenType> = None;
    if lineTokensLength > 1 && lineTokens[1].getDataType().unwrap_or(TokenType::None) == TokenType::CircleBracketBegin 
    {
      let method = methodLink.read().unwrap();
      if let Some(mut lineTokens) = lineTokens[1].tokens.clone() {
        parameters = Some( method.getMethodParameters(&mut lineTokens) );
      }
      // if result
      if lineTokensLength > 3 && lineTokens[2].getDataType().unwrap_or(TokenType::None) == TokenType::Pointer && 
         lineTokens[3].getDataType().unwrap_or(TokenType::None) == TokenType::Word 
      {
        if let Some(lineTokenData) = lineTokens[3].getData() 
        {
          newMethodResultType = Some( getMethodResultType(lineTokenData) );
        }
      } // else -> skip
    // if there are no parameters
    } else 
    { // if result
      if lineTokensLength > 2 && lineTokens[1].getDataType().unwrap_or(TokenType::None) == TokenType::Pointer && 
         lineTokens[2].getDataType().unwrap_or(TokenType::None) == TokenType::Word 
      {
        if let Some(lineTokenData) = lineTokens[2].getData() 
        {
          newMethodResultType = Some( getMethodResultType(lineTokenData) );
        }
      } // else -> skip
    }   // else -> skip

    // create new method
    if let Some(newMethodName) = lineTokens[0].getData() 
    {
      let mut newMethodBuffer = // todo: add type
        Method::new(
          newMethodName,
          line.lines.clone(),
          Some(methodLink.clone())
        );
      newMethodBuffer.result = Some( Token::newEmpty(newMethodResultType.clone()) );
      // add parameters
      if let Some(parameters) = parameters 
      {
        for parameter in parameters 
        {
          if let Some(parameterData) = parameter.getData() 
          {
            newMethodBuffer.pushMemoryCell(
              MemoryCell::new(
                parameterData,
                MemoryCellMode::LockedFinal,
                TokenType::Array,
                Token::newEmpty( parameter.getDataType() )
              )
            );
          }
        }
      }
      // add
      let mut parentMethod = methodLink.write().unwrap(); // todo: check correct work | add type
      parentMethod.methods.push(
        Arc::new(
        RwLock::new(
          newMethodBuffer
        ))
      );
      return true;
    }
  }

  return false;
  /*
  let mut i:   usize = 0;
  let mut add: bool = true;

  let mut line: Line;
  let mut lineTokensLength: usize;

  let mut list: bool;

  let mut token: &Token;

  while i < _lines.len() { // todo: add linesLength and -= 1
      line = _lines[i].clone();
      lineTokensLength = line.tokens.len();
      if lineTokensLength == 0 {
          i += 1;
          continue;
      }

      token = &line.tokens[0];
      if let Some(firstChar) = token.data.chars().next() {
          // fist char upper
          if firstChar.is_ascii_lowercase() {
              // to:do: method(parameters) -> result
              if !line.lines.is_empty() {
                  // min = 1, max = 4 tokens:
                  // name
                  if lineTokensLength == 1 {
                      methods.push(
                          Method::new(
                              token.data.clone(),
                              i,
                              line.lines.clone()
                          )
                      );
                  } else
                  if lineTokensLength < 4 {
                      // name(param)
                      if line.tokens[1].dataType == TokenType::CircleBracketBegin {
                          methods.push(
                              Method::newWithParameters(
                                  token.data.clone(),
                                  i,
                                  line.lines.clone(),
                                  line.tokens[1].tokens.clone()
                              )
                          );
                      } else
                      // name -> Type
                      if line.tokens[1].dataType == TokenType::Pointer {
                          methods.push( 
                              Method::newWithResult(
                                  token.data.clone(),
                                  i,
                                  line.lines.clone(),
                                  line.tokens[2].dataType.to_string()
                              )
                          );
                      }
                  } else
                  // name(param) -> Type
                  if lineTokensLength == 4 {
                      methods.push(
                          Method::newFull(
                              token.data.clone(),
                              i,
                              line.lines.clone(),
                              line.tokens[1].tokens.clone(),
                              line.tokens[3].dataType.to_string()
                          )
                      );
                  } else {
                  // read error
                      // to:do no working here
                      log("syntax","");
                      log("path",&format!(
                          "{} -> Method \"{}\"",
                          unsafe{&*_filePath},
                          line.tokens[0].data
                      ));
                      log("note","Maximum number of instructions when declaring a procedure is 3;");
                      log("note","Maximum number of instructions when declaring a function is 4.");
                      logExit();
                  }
                  
                  _lines.remove(i);
                  add = false;
              //
              }
          }
      }
      if !add {
          add = true;
      } else {
          i += 1;
      }
  }
  //
  */
}
/* search MemoryCell
 e:
   memoryCellName   -> final    locked
   memoryCellName~  -> variable locked
   memoryCellName~~ -> variable unlocked
*/
unsafe fn searchMemoryCell(lineLink: Arc<RwLock<Line>>, methodLink: Arc<RwLock<Method>>) -> bool 
{
  let method: RwLockReadGuard<'_, Method> = methodLink.read().unwrap();
  let   line:  RwLockWriteGuard<'_, Line> = lineLink.write().unwrap();

  let           tokens: &Vec<Token>     = &line.tokens;
  let     tokensLength: usize           = tokens.len();
  let mut            j: usize           = 0;
  let mut       goNext: bool            = true;

  let mut   nameBuffer: Option< String > = None;
  let mut   modeBuffer: MemoryCellMode   = MemoryCellMode::LockedFinal;
  let mut modeReceived: bool             = false;

  let mut   typeBuffer: TokenType = TokenType::None;
  let mut typeReceived: bool      = false;

  let mut operatorBuffer: TokenType  = TokenType::None;
  let mut    valueBuffer: Option< Vec<Token> > = None;

  let mut token: &Token;
  while j < tokensLength 
  {
    token = &tokens[j];

    if token.getDataType().unwrap_or(TokenType::None) == TokenType::Word || modeReceived == true 
    {
      // check mode
      if !modeReceived 
      {
        nameBuffer = token.getData();
        // variableName~~
        if j+2 < tokensLength && tokens[j+2].getDataType().unwrap_or(TokenType::None) == TokenType::Tilde 
        {
          modeBuffer = MemoryCellMode::UnlockedVariable;
          // skip name
          // skip ~
          // skip ~
          j += 3;
        } else
        // variableName~
        if j+1 < tokensLength && tokens[j+1].getDataType().unwrap_or(TokenType::None) == TokenType::Tilde 
        {
          modeBuffer = MemoryCellMode::LockedVariable;
          // skip name
          // skip ~
          j += 2;
        } else 
        { // variableName
          modeBuffer = MemoryCellMode::LockedFinal;
          // skip name
          j += 1;
        }
        //
        goNext = false;
        modeReceived = true;
      } else 
      if !typeReceived 
      { // check type
        if (j < tokensLength && token.getDataType().unwrap_or(TokenType::None) == TokenType::Colon) && j+1 < tokensLength 
        {
          let nextTokenType = tokens[j+1].getDataType().clone();
          if checkMemoryCellType( nextTokenType.clone().unwrap_or(TokenType::None) ) 
          {
            typeBuffer = nextTokenType.unwrap_or(TokenType::None);
            // skip :
            // skip type
            j += 2;
          }
        }
        //
        goNext = false;
        typeReceived = true;
      } else 
      { // check value
        if (j < tokensLength && checkMemoryCellMathOperator( token.getDataType().unwrap_or(TokenType::None) ) ) && j+1 < tokensLength 
        {
          // operator
          operatorBuffer = token.getDataType().unwrap_or(TokenType::None);
          // value
          valueBuffer = Some( tokens[j+1..(tokensLength)].to_vec() );
        }
        // todo: made?
        //   if + or - or * or / and more ...
        //   -> skip this line
        //   e: a + 10
        break;
      }
    }

    if goNext { j += 1; } 
    else { goNext = true; }
  }
  //
  if let Some(nameBuffer) = nameBuffer 
  {
    // if searched in methods
    if let Some(memoryCellLink) = method.getMemoryCellByName(&nameBuffer) 
    {
      method.memoryCellOp(
        memoryCellLink, 
        operatorBuffer, 
        Token::newNesting( valueBuffer )
      );
    // if no searched, then create new MemoryCell and equal right value
    } else 
    if let Some(ref value) = valueBuffer { //  let mut    valueBuffer: Option< Vec<Token> > = None;
      // memoryCellName - op - value
      // array
      if value.len() > 0 && value[0].getDataType().unwrap_or(TokenType::None) == TokenType::SquareBracketBegin 
      {
        if let Some(mut value) = value[0].tokens.clone() 
        {
          value.retain(|token| token.getDataType().unwrap_or(TokenType::None) != TokenType::Comma);
          method.pushMemoryCell(
            MemoryCell::new(
              nameBuffer,
              modeBuffer,
              TokenType::Array,
              Token::newNesting( Some(value) )
            )
          );
        }
        return true;
      // basic cell
      } else 
      {
        method.pushMemoryCell(
          MemoryCell::new(
            nameBuffer,
            modeBuffer,
            typeBuffer,
            Token::newNesting( valueBuffer )
          )
        );
        return true;
      }
      //
    }
  }
  return false;
}
//
lazy_static! 
{
  static ref _main: Arc<RwLock<Method>> = Arc::new(
    RwLock::new(
      Method::new(
        String::from("main"),
        Vec::new(),
        None
      )
    )
  );
}

// parse lines
static mut _lineIndex:   usize                    = 0;
static mut _linesLength: usize                    = 0;

pub unsafe fn parseLines(tokenizerLinesLinks: Vec< Arc<RwLock<Line>> >) -> ()
{
// preparation
  if unsafe{_debugMode} 
  {
      logSeparator("Preparation");
  }

  // define upper struct [Class / Enum]
//    let mut classes: Vec<Class> = Vec::new();
//    let mut enums:   Vec<Enum>  = Vec::new();
//    defineUpperStruct(&mut classes, &mut enums);
  /*
  // output classes
  if !classes.is_empty() {
      log("parserInfo", "Classes");
      for c in classes {
          log("parserBegin",  &format!("  {}",           c.name));
          log("parserHeader", &format!("    Defined on line {}", c.line));
          log("parserHeader",          "    Lines");
          outputLines(&c.lines, 3);
      }
      println!();
  }
  // output enums
  if !enums.is_empty() {
      log("parserInfo", "Enums");
      for e in enums {
          log("parserBegin",  &format!("  {}",           e.name));
          log("parserHeader", &format!("    Defined on line {}", e.line));
          log("parserHeader",          "    Lines");
          outputLines(&e.lines, 3);
      }
      println!();
  }
  */

  // define lower struct [function / procedure / list]
//    let mut methods: Vec<Method> = Vec::new();
//    let mut lists:   Vec<List>   = Vec::new();
//    defineLowerStruct(&mut methods, &mut lists);
  /*
  // output methods
  if !methods.is_empty() {
      log("parserInfo", "Methods");
      for m in methods {
          log("parserBegin",&format!(
              "  {} -> {}",
              m.name,
              m.resultType
          ));
          log("parserHeader", &format!("    Defined on line {}", m.line));
          log("parserHeader", &format!("    Parameters"));
          outputTokens(&m.parameters, 0, 3);
          log("parserHeader",          "    Lines");
          outputLines(&m.lines, 3);
      }
      println!();
  }
  // output lists
  if !lists.is_empty() {
      log("parserInfo", "Lists");
      for l in lists {
          log("parserBegin",  &format!("  {}",           l.name));
          log("parserHeader", &format!("    Defined on line {}", l.line));
          log("parserHeader",          "    Lines");
          outputLines(&l.lines, 3);
      }
      println!();
  }
  */

  // set argv-argc
  // todo: move to main func
  /*
  {
      let mut mcl = getMemoryCellList();
      // argc
      mcl.push(
          MemoryCell::new(
              String::from("argc"),
              MemoryCellMode::LockedFinal,
              TokenType::UInt,
              Token::newNesting(
                  vec![Token::new(TokenType::UInt, _argc.to_string())]
              )
          )
      );
      // argv
      let mut argv: Vec<Token> = Vec::new();
      for a in &_argv {
          argv.push(
              Token::new(TokenType::String, String::from(a))
          );
      }
      mcl.push(
          MemoryCell::new(
              String::from("argv"),
              MemoryCellMode::LockedFinal,
              TokenType::Array,
              Token::newNesting(argv)
          )
      );
  }
  */

  // argc & argv
  {
    let mut main = _main.write().unwrap();
    main.lines = tokenizerLinesLinks.clone();
    // argc
    main.pushMemoryCell(
      MemoryCell::new(
        String::from("argc"),
        MemoryCellMode::LockedFinal,
        TokenType::UInt,
        Token::newNesting(
          Some( vec![
            Token::new( Some(TokenType::UInt), Some(_argc.to_string()) )
          ] )
        )
      )
    );
    // argv
    let mut argv: Vec<Token> = Vec::new();
    for a in &_argv 
    {
      argv.push(
        Token::new( Some(TokenType::String), Some(String::from(a)) )
      );
    }
    main.pushMemoryCell(
      MemoryCell::new(
        String::from("argv"),
        MemoryCellMode::LockedFinal,
        TokenType::Array,
        Token::newNesting( Some(argv) )
      )
    );
  }

  if unsafe{_debugMode} 
  {
    log("ok",&format!("argc [{}]",_argc));
    if _argc > 0 
    {
      log("ok",&format!("argv {:?}",_argv));
    }
  }

  _linesLength = tokenizerLinesLinks.len();

  // read lines
  let startTime: Instant = Instant::now();
  if unsafe{_debugMode} 
  {
    logSeparator("Interpretation");
  }
  readLines(_main.clone(), addr_of_mut!(_lineIndex), addr_of_mut!(_linesLength));
  // duration
  if unsafe{_debugMode} 
  {
    let endTime  = Instant::now();
    let duration = endTime-startTime;
    logSeparator("End");
    log("ok",&format!("Parser duration [{:?}]",duration));
  }
}
pub unsafe fn readLines(methodLink: Arc<RwLock<Method>>, lineIndex: *mut usize, linesLength: *mut usize) -> ()
{
  while _exitCode == false && *lineIndex < *linesLength 
  {
    let lineLink: Arc<RwLock<Line>>;
    {
      let   method: RwLockReadGuard<'_, Method> = methodLink.read().unwrap();
      lineLink = method.lines[*lineIndex].clone();
      let     line: RwLockReadGuard<'_, Line>   = lineLink.read().unwrap();
      // check tokens in line
      if line.tokens.len() == 0 
      {
        *lineIndex += 1;
        continue;
      }
    }
/*
      // save line now for logger
      // todo: delete its pls
      replaceSavedLine( method.lines[*lineIndex].clone() );
*/
    // search conditions
    if !searchCondition(lineLink.clone(), methodLink.clone()) 
    { // search methods
      if !searchMethod(lineLink.clone(), methodLink.clone()) 
      { // search return
        if !searchReturn(lineLink.clone(), methodLink.clone()) 
        { // search methods calls
          let method: RwLockReadGuard<'_, Method> = methodLink.read().unwrap();
          if !method.procedureCall(lineLink.clone()) 
          { // search memory cells
            searchMemoryCell(lineLink, methodLink.clone());
          }
        }
      }
    }

    //
    { // todo: rewrite this part
      let method = methodLink.read().unwrap();
      if method.lines.len() < *linesLength 
      {
        *linesLength = method.lines.len();
      } else 
      {
        *lineIndex += 1;
      }
    }
    //
  }
}