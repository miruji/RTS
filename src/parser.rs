/*
    parser
*/

use crate::logger::*;
use crate::_filePath;
use crate::_argc;
use crate::_argv;
use crate::_debugMode;
use crate::_exitCode;

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

use std::time::Instant;

use std::sync::{Arc, RwLock};

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
    } else {
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
unsafe fn searchCondition(lineIndex: usize, linesLength: &mut usize, methodLink: Arc<RwLock<Method>>) -> bool {
    let mut conditions: Vec<Line> = Vec::new();
    {
        let mut method = methodLink.write().unwrap();
        let lines: &mut Vec<Line> = &mut method.lines;

        let tokens:    &mut Vec<Token> = &mut lines[lineIndex].tokens;
        let mut token: &Token = &tokens[0];
        // check first condition begin
        if token.dataType != TokenType::Question {
            return false;
        }

        // get first condition
        {
            { // clone first condition line
                let mut lineBuffer: Line = (&lines[lineIndex]).clone();
                lineBuffer.tokens.remove(0); // remove ? token
                conditions.push(lineBuffer);
            }

            // search bottom lines
            let mut i: usize = lineIndex+1;
            let mut bottomLineBuffer: &Line;
            let mut lineBuffer:        Line;
            while i < *linesLength {
                bottomLineBuffer = &lines[i];
                if bottomLineBuffer.tokens.len() == 0 {
                    break;
                }
                if bottomLineBuffer.tokens[0].dataType == TokenType::Question {
                    lineBuffer = bottomLineBuffer.clone();
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
    }

    // read conditions
    let mut conditionTruth: bool = false;
    for condition in &mut conditions {
        // if elif
        if condition.tokens.len() != 0 {
            { // check condition truth and unlock mcl
                //let mut mcl = getMemoryCellList(mcl);
                let mcl = &methodLink.read().unwrap().mcl;
                let mcl2 = mcl.read().unwrap();
                conditionTruth = mcl2.expression(&mut condition.tokens,0).data == "true";
            }
            if conditionTruth {
                let mut conditionLinesLength: usize = condition.lines.len();
                let mut conditionLineIndex:   usize = 0;
//                println!("\n>>> Next read block [if-el]");
                _methods.push(
                    Arc::new(
                    RwLock::new(
                        Method::new(
                            String::from("if-el"),
                            condition.lines.clone(),
                            Some(methodLink.clone())
                        )
                    ))
                );
                let methodNum: usize = _methods.len()-1;
                readLines(_methods[_methods.len()-1].clone(), &mut conditionLineIndex, &mut conditionLinesLength);
                _methods.remove(methodNum);
                break;
            }
        // else
        } else
        if !conditionTruth {
            let mut conditionLinesLength: usize = condition.lines.len();
            let mut conditionLineIndex:   usize = 0;
//            println!("  Next read block [else]");
            _methods.push(
                Arc::new(
                RwLock::new(
                    Method::new(
                        String::from("else"),
                        condition.lines.clone(),
                        Some(methodLink.clone())
                    )
                ))
            );
            let methodNum: usize = _methods.len()-1;
            readLines(_methods[methodNum].clone(), &mut conditionLineIndex, &mut conditionLinesLength);
            _methods.remove(methodNum);
            break;
        }
    }
    return true;
    return false;
}
/* search methods call
   e:
     methodCall(parameters)
*/
unsafe fn searchMethodCall(line: &mut Line, methodLink: Arc<RwLock<Method>>) -> bool {
    let tokens:       &mut Vec<Token> = &mut line.tokens;
    let tokensLength: usize           = tokens.len();
    let mut j: usize = 0;

    let mut expressionValue: Vec<Token>;
    let mut result: bool;

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

                    // basic methods
                    result = true;
                    {
                        let mut method = methodLink.write().unwrap();
//                        println!("searchMethodCall() in [{}]",method.name);
                        //let mut mcl = &mut method.mcl.write().unwrap();
                        // println
                        if token.data == "println" {
                            println!("{}",
                                formatPrint(
                                    &method.memoryCellExpression(
                                        &mut expressionValue,
                                        0
                                    ).data
                                )
                            );
                        // print
                        } else 
                        if token.data == "print" {
                            print!("{}",
                                formatPrint(
                                    &method.memoryCellExpression(
                                        &mut expressionValue,
                                        0
                                    ).data
                                )
                            );
                        // exec
                        } else 
                        if token.data == "exec" {
                            let expression: String = method.memoryCellExpression(&mut expressionValue,0).data;
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
                            _exitCode = true;
                        // custom method
                        } else {
                            result = false;
                        }
                    };
                    // custom methods
                    if !result {
                        // search in _methods
                        for method in &_methods {
                            let methodGuard = method.read().unwrap();
                            if token.data == methodGuard.name {
                                let mut linesLengthBuffer: usize = methodGuard.lines.len();
                                let mut lineIndexBuffer:   usize = 0;
                                readLines(method.clone(), &mut lineIndexBuffer, &mut linesLengthBuffer);
                                return true;
                            }
                        }
                    }
                    return result;
                } else {
                // read error
                    log("syntax","");
                    log("path",&format!(
                        "{} -> Word \"{}\"",
                        &*_filePath,
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
/*
// define methods [function / procedure]
unsafe fn searchMethod(line: &mut Line) -> bool {
    let mut i: usize = 0;
    let lineTokens: &Vec<Token> = &line.tokens;
    let lineTokensLength: usize = lineTokens.len();
    // e:  methodName
    //       block
    if lineTokens[0].dataType == TokenType::Word && line.lines.len() > 0 {
        println!("WORD BEGIN");
        /*
        while i < lineTokensLength {
            let token: &Token = &lineTokens[i];
            println!("  i {} t {}",i,token.dataType.to_string());
            i += 1;
        }
        */

        // min = 1, max = 4 tokens:
        // name
        // name(param)
        // name -> Type
        // name(param) -> Type

        _methods.push(
            Method::new(
                lineTokens[0].data.clone(),
                line.lines.clone(),
                None, // todo
            )
        );

        outputTokens(lineTokens,0,0);
        return true;
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
*/
/* search MemoryCell
   e:
     memoryCellName   -> final    locked
     memoryCellName~  -> variable locked
     memoryCellName~~ -> variable unlocked
*/
unsafe fn searchMemoryCell(line: &mut Line, methodLink: Arc<RwLock<Method>>) -> bool {
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
                // todo: made?
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
    //
    if !nameBuffer.is_empty() {
//        println!("MemoryCell: {}",nameBuffer);
        //if let Some(mclLink) = getMemoryCellListByMemoryCellName(methodLink.clone(),&nameBuffer) {
        //    memoryCellOp(mclLink);

            //let mut mcl = mclLink.write().unwrap();
            //mcl.op( // todo: 2 repeats to search one name, rewrite this func
            //    nameBuffer,
            //    operatorBuffer,
            //    Token::newNesting(valueBuffer)
            //);
        //} else 
        let mut method = methodLink.write().unwrap();
//        println!("  Method: {:?}",method.name);
        if let Some(memoryCellLink) = method.getMemoryCellByName(&nameBuffer) {
            {
                let memoryCell = memoryCellLink.read().unwrap();
//                println!("  OK! searched memoryCell [{}]",memoryCell.name);
            }
            method.memoryCellOp(
                memoryCellLink, 
                operatorBuffer, 
                Token::newNesting(valueBuffer)
            );
        } else {
            let mut mcl = &mut method.mcl.write().unwrap();
//            println!("  NO searched! op [{}]",operatorBuffer.to_string());

            // memoryCellName - op - value
            // equals
            if operatorBuffer == TokenType::Equals {
                // todo: check ~~ mode-type
                // new value to MemoryCell
                /*
                if !mcl.getByName( &nameBuffer ).is_none() {
                    mcl.op(
                        nameBuffer,
                        operatorBuffer,
                        Token::newNesting(valueBuffer)
                    );
                    return true;
                // create MemoryCell
                } else*/ {
                    // array
                    if valueBuffer[0].dataType == TokenType::SquareBracketBegin {
//                        println!("    Array in method [{}]",method.name);
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
//                        println!("    Basic in method [{}]",method.name);
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
    }
    return false;
}
//
static mut _methods: Vec< Arc<RwLock<Method>> > = Vec::new();

// parse lines
static mut _lines:       Vec<Line> = Vec::new();
static mut _lineIndex:   usize     = 0;
static mut _linesLength: usize     = 0;

pub unsafe fn parseLines(tokenizerLines: Vec<Line>) {
// preparation
    if unsafe{_debugMode} {
        logSeparator(" > AST preparation");
    }

    _lines = tokenizerLines;

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

    // debug output and read lines
    let startTime: Instant = Instant::now();
    if unsafe{_debugMode} {
        logSeparator(" > AST interpretation");
    }
    // create main method, without name
    _methods.push(
        Arc::new(
        RwLock::new(
            Method::new(
                String::from("main"),
                _lines.clone(),
                None,
            )
        ))
    );
    _linesLength = _lines.len();
    readLines(_methods[0].clone(), &mut _lineIndex, &mut _linesLength);
    // duration
    if unsafe{_debugMode} {
        let endTime  = Instant::now();
        let duration = endTime-startTime;
        logSeparator( &format!("?> Parser duration: {:?}",duration) );
    }
}
pub unsafe fn readLines(methodLink: Arc<RwLock<Method>>, lineIndex: &mut usize, linesLength: &mut usize) {
    let mut line: Option<Line>;
    while _exitCode == false && *lineIndex < *linesLength {
        line = None;
        {
            let method = methodLink.read().unwrap();
            // no tokens in line ?
            if method.lines[*lineIndex].tokens.len() == 0 {
                *lineIndex += 1;
                continue;
            }
            // clone line
            line = Some(method.lines[*lineIndex].clone());
        }
        {
            // search conditions
            if !searchCondition(*lineIndex, linesLength, methodLink.clone()) {
            // search methods calls
                if let Some(mut l) = line {
                    if !searchMethodCall(&mut l, methodLink.clone()) {
            // search memory cells
                        searchMemoryCell(&mut l, methodLink.clone());
                    }
                }
            }
        }
/*
        // save line now for logger
        // todo: delete its pls
        replaceSavedLine( method.lines[*lineIndex].clone() );

        // search conditions
//        if !searchCondition(lines, *lineIndex, linesLength, methodLink, mcl) {
            line = &mut method.lines[*lineIndex];
//            searchMemoryCell(line, methodLink.clone());
//        }
*/
        /*
        // search conditions
        if !searchCondition(methodLink.clone(), lines, *lineIndex, linesLength, mcl) {
            line = &mut lines[*lineIndex]; // set editable line
        // search methods
            if !searchMethod(line) {
        // search methods calls
                if !searchMethodCall(line, mcl) {
        // search memory cells
                    searchMemoryCell(line, methodLink.clone(), mcl);
                }
            }
        }
        */
        {
            let mut method = methodLink.write().unwrap();
            if method.lines.len() < *linesLength {
                *linesLength = method.lines.len();
            } else {
                *lineIndex += 1;
            }
        }
    }
}