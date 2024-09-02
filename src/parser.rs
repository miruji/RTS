/*
  parser
*/

use crate::logger::*;
use crate::_argc;
use crate::_argv;
use crate::_debugMode;
use crate::_exitCode;

pub mod value;
pub mod uf64;
pub mod structure; use crate::parser::structure::*;

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
/* search condition
 e:
   ? condition
     block
   ? condition
     block
*/
/*
unsafe fn searchCondition(lineLink: Arc<RwLock<Line>>, structureLink: Arc<RwLock<Structure>>) -> bool 
{
  // todo: delete lineIndex and use structureLink.lines ?
  let mut conditions: Vec< Arc<RwLock<Line>> > = Vec::new();
  { // get conditions
    let structure:      RwLockReadGuard<'_, Structure> = structureLink.read().unwrap();
    let lines:       &Vec< Arc<RwLock<Line>> >   = &structure.lines;
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
          if bottomLine.tokens[0].getDataType().unwrap_or_default() != TokenType::Question { break; }
        }
        conditions.push(lineBottomLink);
        i += 1;
      }
    }
    // if no conditions
    if conditions.len() == 0 { return false; }
    else { _lineIndex += conditions.len()-1; }
  }
  // read conditions
  let mut conditionTruth: bool = false;
  for conditionLink in &mut conditions 
  {
    let condition: RwLockReadGuard<'_, Line> = conditionLink.read().unwrap();
    // if elif
    if condition.tokens.len() != 0 
    {
      { // check condition truth and unlock mcl
        let mut structure: RwLockWriteGuard<'_, Structure> = structureLink.write().unwrap();
        let mut conditionTokens: Vec<Token> = condition.tokens.clone(); // todo: no clone ? fix its please
        conditionTokens.remove(0);
        conditionTruth = 
          {
            let expressionResult: Option<String> = structure.expression(&mut conditionTokens).getData();
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
      { // new temporary structure
        let mut conditionLinesLength: usize = condition.lines.clone().unwrap_or(vec![]).len();
        let mut conditionLineIndex:   usize = 0;
        let structure: Arc<RwLock<Structure>> =
          Arc::new(
          RwLock::new(
            Structure::new(
              String::from("if-el"),
              condition.lines.clone().unwrap_or(vec![]),
              Some(structureLink.clone())
            )
          ));
        readLines(structure, &mut conditionLineIndex, &mut conditionLinesLength);
        break; // end
      }
    // else
    } else
    if !conditionTruth 
    { // new temporary structure
      let mut conditionLinesLength: usize = condition.lines.clone().unwrap_or(vec![]).len();
      let mut conditionLineIndex:   usize = 0;
      let structure: Arc<RwLock<Structure>> =
        Arc::new(
        RwLock::new(
          Structure::new(
            String::from("else"),
            condition.lines.clone().unwrap_or(vec![]),
            Some(structureLink.clone())
          )
        ));
      readLines(structure, &mut conditionLineIndex, &mut conditionLinesLength);
      break; // end
    }
  }
  return true;
}
*/
// return [function only]
// e:  = value
unsafe fn searchReturn(lineLink: Arc<RwLock<Line>>, structureLink: Arc<RwLock<Structure>>) -> bool 
{
  let     line:       RwLockReadGuard<'_, Line>       = lineLink.read().unwrap();
  let mut lineTokens: Vec<Token>                      = line.tokens.clone();
  let mut structure:  RwLockWriteGuard<'_, Structure> = structureLink.write().unwrap();

  if lineTokens[0].getDataType().unwrap_or_default() == TokenType::Equals 
  {
    lineTokens.remove(0);
    let structureResultType: Option<TokenType> = 
      if let Some(structureResult) = &structure.result 
      {
        structureResult.getDataType().clone()
      } else 
      {
        Some(TokenType::None)
      };
    structure.result = Some(structure.expression(&mut lineTokens));
    if let Some(structureResult) = &mut structure.result 
    {
      structureResult.setDataType( structureResultType );
    }
    return true;
  }
  return false;
}
//
unsafe fn searchStructure(lineLink: Arc<RwLock<Line>>, structureLink: Arc<RwLock<Structure>>) -> bool 
{
  let line:             RwLockReadGuard<'_, Line> = lineLink.read().unwrap();
  let lineTokens:       &Vec<Token>               = &line.tokens;
  let lineTokensLength: usize                     = lineTokens.len();

  let firstTokenType:  TokenType                = lineTokens[0].getDataType().unwrap_or_default();
  let lineLines:       Vec< Arc<RwLock<Line>> > = line.lines.clone().unwrap_or(vec![]);
  let lineLinesLength: usize                    = lineLines.len();

  if firstTokenType == TokenType::Word && lineLinesLength > 0
  { // if structure
//    println!("structure {:?}",lineTokens);
    if let Some(newStructureName) = lineTokens[0].getData() 
    { // if there are parameters
      let mut newStructureResultType: Option<TokenType> = None;
      let mut parameters: Option< Vec<Token> > = None;
      if lineTokensLength > 1 && lineTokens[1].getDataType().unwrap_or_default() == TokenType::Equals
      {
        return false;
      } else 
      if lineTokensLength > 1 && lineTokens[1].getDataType().unwrap_or_default() == TokenType::CircleBracketBegin 
      {
        let structure: RwLockReadGuard<'_, Structure> = structureLink.read().unwrap();
        if let Some(mut lineTokens) = lineTokens[1].tokens.clone() {
          parameters = Some( structure.getStructureParameters(&mut lineTokens) ); // todo: no Word type, fix pls
        }
        // if result
        if lineTokensLength > 3 && lineTokens[2].getDataType().unwrap_or_default() == TokenType::Pointer && 
           lineTokens[3].getDataType().unwrap_or_default() == TokenType::Word 
        {
          if let Some(lineTokenData) = lineTokens[3].getData() 
          {
            newStructureResultType = Some( getStructureResultType(lineTokenData) );
          }
        } // else -> skip
      // if there are no parameters
      } else 
      { // if result
        if lineTokensLength > 2 && lineTokens[1].getDataType().unwrap_or_default() == TokenType::Pointer && 
           lineTokens[2].getDataType().unwrap_or_default() == TokenType::Word 
        {
          if let Some(lineTokenData) = lineTokens[2].getData() 
          {
            newStructureResultType = Some( getStructureResultType(lineTokenData) );
          }
        } // else -> skip
      }   // else -> skip
      // new structure
      let mut newStructure: Structure = 
        Structure::new(
          newStructureName,
          lineLines,
          Some(structureLink.clone())
        );
      // new structure modificators
      newStructure.result = Some( Token::newEmpty(newStructureResultType.clone()) );
      if let Some(parameters) = &parameters 
      { // add parameters
        for parameter in parameters 
        {
          newStructure.pushMemoryCell(
            Structure::new(
              parameter.getData().unwrap_or_default(),
              vec![], // todo: add option, pls 
              None,
            )
          );
        }
      }
      // get parent
      let mut structure: RwLockWriteGuard<'_, Structure> = structureLink.write().unwrap();
      if let Some(ref mut structureStructures) = structure.structures 
      { // if exists
        structureStructures.push(
          Arc::new(RwLock::new(
            newStructure
          ))
        );
      } else 
      { // if no exists
        structure.structures = 
          Some(vec![
            Arc::new(RwLock::new(
              newStructure
            ))
          ]);
      }
      return true;
    }
  } else 
  if firstTokenType == TokenType::Question && lineLinesLength > 0
  { // if condition
//    println!("condition");
    return true;
  }
  return false;
}
/*
// define structures [function / procedure]
//   min = 1, max = 4 tokens:
//   name
//   name(param)
//   name -> Type
//   name(param) -> Type
unsafe fn searchStructure(lineLink: Arc<RwLock<Line>>, structureLink: Arc<RwLock<Structure>>) -> bool 
{
  let line:             RwLockReadGuard<'_, Line> = lineLink.read().unwrap();
  let lineTokens:       &Vec<Token>               = &line.tokens;
  let lineTokensLength: usize                     = lineTokens.len();

  //
  let mut parameters: Option< Vec<Token> > = None;
  if lineTokens[0].getDataType().unwrap_or_default() == TokenType::Word && 
     line.lines.clone().unwrap_or(vec![]).len() > 0
  { // if there are parameters
    let mut newStructureResultType: Option<TokenType> = None;
    if lineTokensLength > 1 && lineTokens[1].getDataType().unwrap_or_default() == TokenType::Equals
    {
      return false;
    } else 
    if lineTokensLength > 1 && lineTokens[1].getDataType().unwrap_or_default() == TokenType::CircleBracketBegin 
    {
      let structure: RwLockReadGuard<'_, Structure> = structureLink.read().unwrap();
      if let Some(mut lineTokens) = lineTokens[1].tokens.clone() {
        parameters = Some( structure.getStructureParameters(&mut lineTokens) ); // todo: no Word type, fix pls
      }
      // if result
      if lineTokensLength > 3 && lineTokens[2].getDataType().unwrap_or_default() == TokenType::Pointer && 
         lineTokens[3].getDataType().unwrap_or_default() == TokenType::Word 
      {
        if let Some(lineTokenData) = lineTokens[3].getData() 
        {
          newStructureResultType = Some( getStructureResultType(lineTokenData) );
        }
      } // else -> skip
    // if there are no parameters
    } else 
    { // if result
      if lineTokensLength > 2 && lineTokens[1].getDataType().unwrap_or_default() == TokenType::Pointer && 
         lineTokens[2].getDataType().unwrap_or_default() == TokenType::Word 
      {
        if let Some(lineTokenData) = lineTokens[2].getData() 
        {
          newStructureResultType = Some( getStructureResultType(lineTokenData) );
        }
      } // else -> skip
    }   // else -> skip

    // create new structure
    if let Some(newStructureName) = lineTokens[0].getData() 
    {
      let mut newStructureBuffer: Structure = 
        Structure::new(
          newStructureName,
          line.lines.clone().unwrap_or(vec![]),
          Some(structureLink.clone())
        );
      newStructureBuffer.result = Some( Token::newEmpty(newStructureResultType.clone()) );
      // add parameters
      if let Some(parameters) = &parameters 
      {
        for parameter in parameters 
        {
//          println!("1: parameter [{:?}]:[{}]",parameter,parameter.getDataType().unwrap_or_default().to_string());
          newStructureBuffer.pushMemoryCell(
            Structure::new(
              parameter.getData().unwrap_or_default(),
              vec![], // todo: add option, pls 
              None,
            )
          );
        }
      }
      // add
      let mut parentStructure: RwLockWriteGuard<'_, Structure> = structureLink.write().unwrap(); // todo: check correct work
      if let Some(ref mut parentStructureStructure) = parentStructure.structures 
      {
        parentStructureStructure.push(
          Arc::new(
          RwLock::new(
            newStructureBuffer
          ))
        );
      } else 
      {
        parentStructure.structures = 
          Some(vec![
            Arc::new(
            RwLock::new(
              newStructureBuffer
            ))
          ]);
      }
      return true;
    }
  }
  return false;
}
*/
/* search MemoryCell
 e:
   memoryCellName   -> final    locked
   memoryCellName~  -> variable locked
   memoryCellName~~ -> variable unlocked
*/
/*
unsafe fn searchMemoryCell(lineLink: Arc<RwLock<Line>>, structureLink: Arc<RwLock<Structure>>) -> bool 
{
  let line: RwLockWriteGuard<'_, Line> = lineLink.write().unwrap();

  let           tokens: &Vec<Token>     = &line.tokens;
  let     tokensLength: usize           = tokens.len();
  let mut            j: usize           = 0;
  let mut       goNext: bool            = true;

  let mut   nameBuffer: Option< String > = None;
//  let mut   modeBuffer: MemoryCellMode   = MemoryCellMode::LockedFinal;
  let mut modeReceived: bool             = false;

  let mut   typeBuffer: TokenType = TokenType::None;
  let mut typeReceived: bool      = false;

  let mut operatorBuffer: TokenType  = TokenType::None;

  let mut token: &Token;
  while j < tokensLength 
  {
    token = &tokens[j];
    if token.getDataType().unwrap_or_default() == TokenType::Word || modeReceived == true 
    { // check mode
      if !modeReceived 
      {
        nameBuffer = token.getData();
        // variableName~~
        if j+2 < tokensLength && tokens[j+2].getDataType().unwrap_or_default() == TokenType::Tilde 
        {
//          modeBuffer = MemoryCellMode::UnlockedVariable;
          // skip name
          // skip ~
          // skip ~
          j += 3;
        } else
        // variableName~
        if j+1 < tokensLength && tokens[j+1].getDataType().unwrap_or_default() == TokenType::Tilde 
        {
//          modeBuffer = MemoryCellMode::LockedVariable;
          // skip name
          // skip ~
          j += 2;
        } else 
        { // variableName
//          modeBuffer = MemoryCellMode::LockedFinal;
          // skip name
          j += 1;
        }
        //
        goNext = false;
        modeReceived = true;
      } else 
      if !typeReceived 
      { // check type
        if (j < tokensLength && token.getDataType().unwrap_or_default() == TokenType::Colon) && j+1 < tokensLength 
        {
          let nextTokenType = tokens[j+1].getDataType().clone();
          if checkMemoryCellType( nextTokenType.clone().unwrap_or_default() ) 
          {
            typeBuffer = nextTokenType.unwrap_or_default();
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
        if j < tokensLength && checkMemoryCellMathOperator( token.getDataType().unwrap_or_default() )
        {
          // operator
          operatorBuffer = token.getDataType().unwrap_or_default();
        }
        break;
      }
    }

    if goNext { j += 1; } 
    else { goNext = true; }
  }
  //
  if let Some(nameBuffer) = nameBuffer 
  {
    let mut structure: RwLockWriteGuard<'_, Structure> = structureLink.write().unwrap();
    if let Some(structureLink) = structure.getStructureByName(&nameBuffer) 
    { // if searched in structures
//      println!("111");
      /*
      structure.memoryCellOp(
        structureLink, 
        operatorBuffer, 
        Token::newNesting( valueBuffer )
      );
      */
    } else 
    if let Some(lineLines) = &line.lines 
    { // if no searched, then create new Structure and equal right value
//      println!("new memoryCell!");
      // new memoryCell
      structure.pushMemoryCell(
        Structure::new(
          nameBuffer,
          //modeBuffer,
          //TokenType::Array,
          lineLines.clone(),
          None
        )
      );
    }
  }
  return false;
}
*/
//
lazy_static! 
{
  static ref _main: Arc<RwLock<Structure>> = Arc::new(
    RwLock::new(
      Structure::new(
        String::from("main"),
        Vec::new(),
        None
      )
    )
  );
}

// parse lines
static mut _lineIndex:   usize = 0;
static mut _linesLength: usize = 0;

pub unsafe fn parseLines(tokenizerLinesLinks: Vec< Arc<RwLock<Line>> >) -> ()
{
// preparation
  if unsafe{_debugMode} 
  {
      logSeparator("Preparation");
  }

  // argc & argv
  {
    let mut main = _main.write().unwrap();
    main.lines = tokenizerLinesLinks.clone();
    // argc
    /*
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
    */
    // argv
    let mut argv: Vec<Token> = Vec::new();
    for a in &_argv 
    {
      argv.push(
        Token::new( Some(TokenType::String), Some(String::from(a)) )
      );
    }
    /*
    main.pushMemoryCell(
      Structure::new(
        String::from("argv"),
        //MemoryCellMode::LockedFinal,
        //TokenType::Array,
        Some(argv),
        None
      )
    );
    */
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
pub unsafe fn readLines(structureLink: Arc<RwLock<Structure>>, lineIndex: *mut usize, linesLength: *mut usize) -> ()
{
  while _exitCode == false && *lineIndex < *linesLength 
  {
    let lineLink: Arc<RwLock<Line>>;
    {
      let   structure: RwLockReadGuard<'_, Structure> = structureLink.read().unwrap();
      lineLink = structure.lines[*lineIndex].clone();
      let     line: RwLockReadGuard<'_, Line>         = lineLink.read().unwrap();
      // check tokens in line
      if line.tokens.len() == 0 
      {
        *lineIndex += 1;
        continue;
      }
    }
    // structure
    if !searchStructure(lineLink.clone(), structureLink.clone()) 
    { // search return
      if !searchReturn(lineLink.clone(), structureLink.clone()) 
      { // expression
        let mut structure: RwLockWriteGuard<'_, Structure> = structureLink.write().unwrap(); // todo: remove it
        structure.expression(&mut lineLink.write().unwrap().tokens);
      }
    }
    /*
    // search conditions
    if !searchCondition(lineLink.clone(), structureLink.clone()) 
    { // search structures
      if !searchStructure(lineLink.clone(), structureLink.clone()) 
      { // search return
        if !searchReturn(lineLink.clone(), structureLink.clone()) 
        { // search structures calls
          let procedureCall: bool = 
            {
              let mut structure: RwLockWriteGuard<'_, Structure> = structureLink.write().unwrap(); // todo: remove it
              structure.procedureCall(lineLink.clone())
            };
          // search memory cells
          if !procedureCall 
          {
            if !searchMemoryCell(lineLink.clone(), structureLink.clone()) 
            { // expression
              let mut structure: RwLockWriteGuard<'_, Structure> = structureLink.write().unwrap(); // todo: remove it
              structure.expression(&mut lineLink.write().unwrap().tokens);
            }
          }
        }
      }
    }
    */
    { // todo: rewrite this part
      let structure = structureLink.read().unwrap();
      if structure.lines.len() < *linesLength 
      {
        *linesLength = structure.lines.len();
      } else 
      {
        *lineIndex += 1;
      }
    }
    //
  }
}