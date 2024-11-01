/*
  parser
*/

pub mod value;
pub mod uf64;
pub mod structure;

use crate::{
  logger::*,
  _argc, _argv, _debugMode, _exitCode,
  parser::structure::*,
  tokenizer::{self, token::*, line::*}
};

use std::{
  time::{Instant, Duration},
  sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard},
  ptr::addr_of_mut
};

// check memory cell type
fn checkMemoryCellType(dataType: TokenType) -> bool 
{
  matches!(dataType, 
    TokenType::Int | 
    TokenType::UInt | 
    TokenType::Float | 
    TokenType::UFloat | 
    TokenType::Rational
  )
  // todo: complex number
  // and other types
}
// check operator
fn checkMemoryCellMathOperator(dataType: TokenType) -> bool 
{
  matches!(dataType, 
    TokenType::Equals         | // =
    TokenType::UnaryPlus      | // ++
    TokenType::PlusEquals     | // +=
    TokenType::UnaryMinus     | // --
    TokenType::MinusEquals    | // -=
    TokenType::UnaryMultiply  | // **
    TokenType::MultiplyEquals | // *=
    TokenType::UnaryDivide    | // //
    TokenType::DivideEquals   | // /=
    TokenType::UnaryModulo    | // %%
    TokenType::ModuloEquals   | // %=
    TokenType::UnaryExponent  | // ^^
    TokenType::ExponentEquals   // ^=
  )
}

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
unsafe fn searchStructure(lineLink: Arc<RwLock<Line>>, parentLink: Arc<RwLock<Structure>>) -> bool 
{
  let line:             RwLockReadGuard<'_, Line> = lineLink.read().unwrap();
  let lineTokens:       &Vec<Token>               = &line.tokens;
  let lineTokensLength: usize                     = lineTokens.len();

  let firstTokenType:  TokenType                = lineTokens[0].getDataType().unwrap_or_default();
  let lineLines:       Vec< Arc<RwLock<Line>> > = line.lines.clone().unwrap_or(vec![]);
  let lineLinesLength: usize                    = lineLines.len();

  if firstTokenType == TokenType::Word
  { // if structure
    if lineLinesLength > 0 
    { // array structure
//      println!("[array structure] {:?}",lineTokens);
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
          let structure: RwLockReadGuard<'_, Structure> = parentLink.read().unwrap();
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
            Some(parentLink.clone())
          );
        // new structure modificators
        newStructure.result = Some( Token::newEmpty(newStructureResultType.clone()) );
        if let Some(parameters) = &parameters 
        { // add parameters
          for parameter in parameters 
          {
            newStructure.pushStructure(
              Structure::new(
                parameter.getData().unwrap_or_default(),
                vec![], // todo: add option, pls 
                None,
              )
            );
          }
        }
        // create new structure link
        let newStructure: Arc<RwLock<Structure>> =
          Arc::new(RwLock::new(
            newStructure
          ));
        {
          let mut conditionLinesLength: usize = lineLinesLength;
          let mut conditionLineIndex:   usize = 0;
          readLines(newStructure.clone(), &mut conditionLineIndex, &mut conditionLinesLength, true);
        }
        // get parent
        let mut parent: RwLockWriteGuard<'_, Structure> = parentLink.write().unwrap();
        if let Some(ref mut parentStructures) = parent.structures 
        { // if exists
          parentStructures.push(
            newStructure
          );
        } else 
        { // if no exists
          parent.structures = 
            Some(vec![
              newStructure
            ]);
        }
        return true;
      }
    } else 
    { // check op
      let mut opType: TokenType = TokenType::None;
      let mut opPos:  usize     = 0;
      for (l, lineToken) in lineTokens.iter().enumerate()
      {
        opType = lineToken.getDataType().unwrap_or_default().clone();
        if checkMemoryCellMathOperator(opType.clone()) 
        {
          opPos = l+1;
          break;
        }
      }
      if lineTokensLength > 1 && opPos > 1
      { // line structure
//        println!("[line structure]");
        if let Some(structureName) = lineTokens[0].getData() 
        {
          let leftValue  = Some( lineTokens[1..opPos-1].to_vec() ); // todo: type
          let rightValue = Some( lineTokens[opPos..(lineTokensLength)].to_vec() ); // todo: type
//          println!("  [structureName] [{}] [{:?}]",structureName,rightValue);
          let mut structure: RwLockWriteGuard<'_, Structure> = parentLink.write().unwrap();
          if let Some(parentLink) = structure.getStructureByName(&structureName) 
          { // if searched in structures
//            println!("  searched!!!");
            structure.structureOp(
              parentLink, 
              opType, 
              leftValue.unwrap_or(vec![]).clone(),
              rightValue.unwrap_or(vec![]).clone()
            );
          } else 
          { // if no searched, then create new Structure and equal right value
//            println!("  new!");
            // new structure
            structure.pushStructure(
              Structure::new(
                structureName,
                vec![ Arc::new(RwLock::new( 
                  Line {
                    tokens: rightValue.unwrap_or(vec![]).clone(),
                    indent: 0,
                    index:  10,
                    lines:  None,
                    parent: None
                  }
                )) ],
                None
              )
            );
          }
          return true;
        }
      }
      //
    }
  } else 
  if firstTokenType == TokenType::Question && lineLinesLength > 0
  { // if condition
//    println!("[condition]");
    let mut conditions: Vec< Arc<RwLock<Line>> > = Vec::new();
    { // get conditions
      let structure:   RwLockReadGuard<'_, Structure> = parentLink.read().unwrap();
      let lines:       &Vec< Arc<RwLock<Line>> >      = &structure.lines;
      let linesLength: usize                          = lines.len();
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
//    println!("[conditions.len] [{}]",conditions.len());
    let mut conditionTruth: bool = false;
    for conditionLink in &mut conditions 
    {
      let condition: RwLockReadGuard<'_, Line> = conditionLink.read().unwrap();
      if condition.tokens.len() > 1 
      { // if elif
        { // check condition truth
          let mut structure: RwLockWriteGuard<'_, Structure> = parentLink.write().unwrap();
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
                Some(parentLink.clone())
              )
            ));
          readLines(structure, &mut conditionLineIndex, &mut conditionLinesLength, false);
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
              Some(parentLink.clone())
            )
          ));
        readLines(structure, &mut conditionLineIndex, &mut conditionLinesLength, false);
        break; // end
      }
    }
    return true;
  }
  return false;
}
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
    let mut main: RwLockWriteGuard<'_, Structure> = _main.write().unwrap();
    main.lines = tokenizerLinesLinks.clone();
    // argc
    /*
    main.pushStructure(
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
    main.pushStructure(
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
  readLines(_main.clone(), addr_of_mut!(_lineIndex), addr_of_mut!(_linesLength), false);
  // duration
  if unsafe{_debugMode} 
  {
    let endTime:  Instant  = Instant::now();
    let duration: Duration = endTime-startTime;
    logSeparator("End");
    log("ok",&format!("Parser duration [{:?}]",duration));
  }
}
pub unsafe fn readLines(structureLink: Arc<RwLock<Structure>>, lineIndex: *mut usize, linesLength: *mut usize, structuresRead: bool) -> ()
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
        if !structuresRead 
        {
          let mut structure: RwLockWriteGuard<'_, Structure> = structureLink.write().unwrap(); // todo: remove it
          structure.expression(&mut lineLink.write().unwrap().tokens);
        }
      }
    }
    { // todo: rewrite this part
      let structure: RwLockReadGuard<'_, Structure> = structureLink.read().unwrap();
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