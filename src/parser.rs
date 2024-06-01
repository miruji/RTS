/*
    parser
*/

use crate::logger::*;
use crate::filePath;
use crate::_argc;
use crate::_argv;

pub mod memoryCell;     use crate::parser::memoryCell::*;
pub mod memoryCellList; use crate::parser::memoryCellList::*;
use std::sync::MutexGuard;

pub mod value;
pub mod uf64;
pub mod class;  use crate::parser::class::*;
pub mod r#enum; use crate::parser::r#enum::*;
pub mod method; use crate::parser::method::*;
pub mod list;   use crate::parser::list::*;

use crate::tokenizer::*;
use crate::tokenizer::token::*;
use crate::tokenizer::line::*;

use std::process::Command;

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
                            unsafe{&*filePath},
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
/* search condition
   e:
     ? condition
       block
     ? condition
       block
*/
unsafe fn searchCondition(lines: &mut Vec<Line>, lineIndex: usize, linesLength: &mut usize) -> bool {
    let tokens:    &mut Vec<Token> = &mut lines[lineIndex].tokens;
    let mut token: &Token = &tokens[0];

    // search first condition
    if token.dataType == TokenType::Question {
        let mut conditions = Vec::new();
        {
            let mut i: usize = lineIndex+1;
            {
                let mut lineBuffer = (&lines[lineIndex]).clone();
                lineBuffer.tokens.remove(0);
                conditions.push(lineBuffer);
            }

            // search bottom lines
            let mut bottomLineBuffer: &Line;
            while i < *linesLength {
                bottomLineBuffer = &lines[i];
                if bottomLineBuffer.tokens.len() == 0 {
                    break;
                }
                if bottomLineBuffer.tokens[0].dataType == TokenType::Question {
                    let mut lineBuffer = bottomLineBuffer.clone();
                    lineBuffer.tokens.remove(0);
                    conditions.push(lineBuffer);

                    lines.remove(i);
                    *linesLength -= 1;
                    continue;
                } else {
                    break;
                }
                i += 1;
            }
        }

        if conditions.len() == 0 {
            return false;
        }

        // read conditions
        let mut conditionTruth: bool = false;
        for condition in &mut conditions {
            // if elif
            if condition.tokens.len() != 0 {
                //println!("  !  if elif");
                {   // todo: move mcl up ?
                    let mcl: MutexGuard<'static, MemoryCellList> = getMemoryCellList();
                    conditionTruth = mcl.expression(&mut condition.tokens,0).data == "true";
                }
                if conditionTruth {
                    let mut conditionLinesLength = condition.lines.len();
                    let mut conditionLineIndex = 0;
                    //println!("  !! condition true");
                    readLines(&mut condition.lines, &mut conditionLineIndex, &mut conditionLinesLength);
                    break;
                } else {
                    //println!("  !! condition false");
                }
            // else
            } else
            if !conditionTruth {
                //println!("  !  else");
                let mut conditionLinesLength = condition.lines.len();
                let mut conditionLineIndex = 0;
                //println!("  !! condition true");
                readLines(&mut condition.lines, &mut conditionLineIndex, &mut conditionLinesLength);
                break;
            }
        }
        return true;
    }
    return false;
}
/* search methods call
   e:
     methodCall(parameters)
*/
unsafe fn searchMethodsCall(line: &mut Line) -> bool {
    let tokens:       &mut Vec<Token> = &mut line.tokens;
    let tokensLength: usize           = tokens.len();
    let mut j: usize = 0;

    let mut expressionValue: Vec<Token>;
    let mcl: MutexGuard<'static, MemoryCellList>;

    let mut token: &Token;
    while j < tokensLength {
        token = &tokens[j];

        if token.dataType == TokenType::Word {
            // add method call
            if j+1 < tokensLength && tokens[j+1].dataType == TokenType::CircleBracketBegin {
                // check lower first char
                if token.data.starts_with(|c: char| c.is_lowercase()) {
                    expressionValue = tokens[j+1].tokens.clone();
                    // todo: multi-param

                    mcl = getMemoryCellList();
                    // println
                    if token.data == "println" {
                        println!("{}",
                            mcl.expression(
                                &mut expressionValue,
                                0
                            ).data
                        );
                    // print
                    } else 
                    if token.data == "print" {
                        print!("{}",
                            mcl.expression(
                                &mut expressionValue,
                                0
                            ).data
                        );
                    // exec
                    } else 
                    if token.data == "exec" {
                        let expression: String = mcl.expression(&mut expressionValue,0).data;
                        let mut parts = expression.split_whitespace();
                        let commandStr = parts.next().expect("No command found in expression");
                        let args: Vec<&str> = parts.collect();
                        let commandOutput = 
                            Command::new(commandStr)
                                .args(&args)
                                .output()
                                .expect("failed to execute process");
                        let outputStr = String::from_utf8_lossy(&commandOutput.stdout);
                        if !outputStr.is_empty() {
                            print!("{}", outputStr);
                        }
                    // exit
                    } else 
                    if token.data == "exit" {
                        std::process::exit(
                            mcl.expression(
                                &mut expressionValue,
                                0
                            ).data.parse::<i32>().unwrap()
                        );
                    } 
                    //
                    return true;
                } else {
                // read error
                    log("syntax","");
                    log("path",&format!(
                        "{} -> Word \"{}\"",
                        unsafe{&*filePath},
                        token.data
                    ));
                    Line::outputTokens( &getSavedLine() );
                    log("note","Method calls and variable names must begin with a lower char");
                    logExit();
                }
            }
        }

        j += 1;
    }
    return false;
}
// check memory cell type
fn checkMemoryCellType(dataType: TokenType) -> bool {
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
fn checkMemoryCellMathOperator(dataType: TokenType) -> bool {
    return 
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
            // todo: complex number
            // and other types
        } else {
            false
        }
}
/* search MemoryCell
   e:
     memoryCellName   -> final    locked
     memoryCellName~  -> variable locked
     memoryCellName~~ -> variable unlocked
*/
unsafe fn searchMemoryCell(line: &mut Line) -> bool {
    let tokens:           &mut Vec<Token> = &mut line.tokens;
    let mut tokensLength: usize           = tokens.len();
    let mut j:            usize           = 0;
    let mut goNext:       bool            = true;

    let mut nameBuffer:     String         = String::new();
    let mut modeBuffer:     MemoryCellMode = MemoryCellMode::LockedFinal;
    let mut modeReceived:   bool           = false;

    let mut typeBuffer:     TokenType = TokenType::None;
    let mut typeReceived:   bool      = false;

    let mut operatorBuffer: TokenType  = TokenType::None;
    let mut valueBuffer:    Vec<Token> = Vec::new();

    let mut token: &Token;
    while j < tokensLength {
        token = &tokens[j];

        if token.dataType == TokenType::Word || modeReceived == true {
            // check mode
            if !modeReceived {
                nameBuffer = token.data.clone();
                // e: variableName~~
                if j+2 < tokensLength && tokens[j+2].dataType == TokenType::Tilde {
                    modeBuffer = MemoryCellMode::UnlockedVariable;
                    tokens.remove(j); // remove name
                    tokens.remove(j); // remove ~
                    tokens.remove(j); // remove ~
                    tokensLength -= 3;
                // e: variableName~
                } else
                if j+1 < tokensLength && tokens[j+1].dataType == TokenType::Tilde {
                    modeBuffer = MemoryCellMode::LockedVariable;
                    tokens.remove(j); // remove name
                    tokens.remove(j); // remove ~
                    tokensLength -= 2;
                // e: variableName
                } else {
                    modeBuffer = MemoryCellMode::LockedFinal;
                    tokens.remove(j);
                    tokensLength -= 1; // remove name
                }
                //
                goNext = false;
                modeReceived = true;
            }
            // check type
            else
            if !typeReceived {
                if (j < tokensLength && token.dataType == TokenType::Colon) && j+1 < tokensLength {
                    let nextTokenType = tokens[j+1].dataType.clone();
                    if checkMemoryCellType(nextTokenType.clone()) {
                        typeBuffer = nextTokenType;
                        tokens.remove(j); // remove :
                        tokens.remove(j); // remove type
                        tokensLength -= 2;
                    }
                }
                //
                goNext = false;
                typeReceived = true;
            }
            // check value
            else {
                if (j < tokensLength && checkMemoryCellMathOperator(token.dataType.clone())) && j+1 < tokensLength {
                    // operator
                    operatorBuffer = token.dataType.clone();
                    // value
                    valueBuffer = tokens[j+1..(tokensLength)].to_vec();
                    tokens.clear();
                }
                // todo:
                //   if + or - or * or / and more ...
                //   -> skip this line
                //   e: a + 10
                break;
            }
        }

        if goNext {
            j += 1;
        } else {
            goNext = true;
        }
    }

    if !nameBuffer.is_empty() {
        //println!("    Name:\"{}\"",nameBuffer);
        //println!("      Operator: \"{}\"",operatorBuffer.to_string());
        //if !valueBuffer.is_empty() {
        //    println!("      Value");
        //    outputTokens(&valueBuffer, 0, 4);
        //}
        // memoryCellName - op - value
        let mut mcl = getMemoryCellList();
        if operatorBuffer == TokenType::Equals {
            // todo: check ~~ mode-type

            // new value to MemoryCell
            if /*let Some(mc) =*/ !mcl.getCell( &nameBuffer ).is_none() {
                //let mcName      = mc.name.clone();
                //let mcMode      = mc.mode.to_string();
                //let mcValueType = mc.valueType.to_string();

                mcl.op(
                    nameBuffer,
                    operatorBuffer,
                    Token::newNesting(valueBuffer)
                );
                return true;

                //println!("      Mode: \"{}\"",mcMode);
                //println!("      Type: \"{}\"",mcValueType);
            // create MemoryCell
            } else {
                // array
                if valueBuffer[0].dataType == TokenType::SquareBracketBegin {
                    valueBuffer = valueBuffer[0].tokens.clone();
                    valueBuffer.retain(|token| token.dataType != TokenType::Comma);
                    mcl.push(
                        MemoryCell::new(
                            nameBuffer,
                            modeBuffer,
                            TokenType::Array,
                            Token::newNesting( valueBuffer )
                        )
                    );
                    return true;
                // basic cell
                } else {
                    mcl.push(
                        MemoryCell::new(
                            nameBuffer,
                            modeBuffer,
                            typeBuffer,
                            Token::newNesting( valueBuffer )
                        )
                    );
                    return true;
                }
                //let mc: &MemoryCell = mcl.last();
                //println!("      Mode: \"{}\"",mc.mode.to_string());
                //println!("      Type: \"{}\"",mc.valueType.to_string());
            }
        // op
        } else
        if operatorBuffer != TokenType::None {
            mcl.op(
                nameBuffer,
                operatorBuffer,
                Token::newNesting( valueBuffer.clone() )
            );
            return true;
        }
    }
    return false;
}

// parse lines
static mut _lines:       Vec<Line> = Vec::new();
static mut _lineIndex:   usize     = 0;
static mut _linesLength: usize     = 0;

pub unsafe fn parseLines(tokenizerLines: Vec<Line>) {
// preparation
    _lines = tokenizerLines;

    // define upper struct [Class / Enum]
    let mut classes: Vec<Class> = Vec::new();
    let mut enums:   Vec<Enum>  = Vec::new();
    defineUpperStruct(&mut classes, &mut enums);
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
    let mut methods: Vec<Method> = Vec::new();
    let mut lists:   Vec<List>   = Vec::new();
    defineLowerStruct(&mut methods, &mut lists);
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

// read lines
    //log("parserInfo", "Lines");
    //let ident_str1: String = " ".repeat(2);
    //let ident_str2: String = " ".repeat(4);

    _linesLength = _lines.len();
    readLines(&mut _lines, &mut _lineIndex, &mut _linesLength);
}
pub unsafe fn readLines(lines: &mut Vec<Line>, lineIndex: &mut usize, linesLength: &mut usize) {
    let mut line: &mut Line;
    while *lineIndex < *linesLength {
        // no tokens in line ?
        if lines[*lineIndex].tokens.len() == 0 {
            *lineIndex += 1;
            continue;
        }

        //log("parserBegin", &format!("{}+{}", ident_str1, i));
        //println!("index: {}, length: {}",*lineIndex,*linesLength);
        replaceSavedLine( lines[*lineIndex].clone() ); // save line now for logger
        
        // output
        /*
        if !line.tokens.is_empty() {
            log("parserHeader", &format!("{}Tokens", ident_str2));
            outputTokens(&line.tokens, 0, 3);
        }

        if !line.lines.is_empty() {
            log("parserHeader", &format!("{}Lines", ident_str2));
            outputLines(&line.lines, 3);
        }
        */

        // search methods calls
        if !searchCondition(lines, *lineIndex, linesLength) {
            line = &mut lines[*lineIndex]; // set editable line
            if !searchMethodsCall(line) {
                searchMemoryCell(line);
            }
        }
        if lines.len() < *linesLength {
            *linesLength = lines.len();
        } else {
            *lineIndex += 1;
        }

        //
        //log("parserEnd", &format!("{}-{}", ident_str1, i));
    }
}