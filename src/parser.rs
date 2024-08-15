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

use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

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
unsafe fn searchCondition(lineLink: Arc<RwLock<Line>>, methodLink: Arc<RwLock<Method>>) -> bool {
    // todo: delete lineIndex and use methodLink.lines ?
    let mut conditions: Vec< Arc<RwLock<Line>> > = Vec::new();
    { // get conditions
        let method: RwLockReadGuard<'_, Method>  = methodLink.read().unwrap();
        let lines:  &Vec< Arc<RwLock<Line>> >    = &method.lines;
        let mut linesLength: usize = lines.len();
        { // search bottom lines
            let mut i: usize = 0;
            {
                let line: RwLockReadGuard<'_, Line> = lineLink.read().unwrap();
                i = line.index;
            }
            // if line index < lines length
            while i < linesLength {
                // check question
                let mut lineBottomLink: Arc<RwLock<Line>> = lines[i].clone();
                { // check question token
                    let bottomLine: RwLockReadGuard<'_, Line> = lineBottomLink.read().unwrap();
                    //Line::outputTokens(&bottomLine);
                    // skip empty line
                    if bottomLine.tokens.len() == 0 { break; } else
                    // check question
                    if bottomLine.tokens[0].dataType != TokenType::Question { break; }
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
    for conditionLink in &mut conditions {
        let condition = conditionLink.read().unwrap();
        // if elif
        if condition.tokens.len() != 0 {
            { // check condition truth and unlock mcl
                let method = methodLink.read().unwrap();
                let mut conditionTokens = condition.tokens.clone(); // todo: no clone ? fix its please
                conditionTokens.remove(0);
                conditionTruth = method.memoryCellExpression(&mut conditionTokens,0).data == "true";
            }
            if conditionTruth {
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
        if !conditionTruth {
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
/* search methods call
   e:
     methodCall(parameters)
*/
unsafe fn searchMethodCall(lineIndex: usize, methodLink: Arc<RwLock<Method>>) -> bool {
    let method: RwLockReadGuard<'_, Method> = methodLink.read().unwrap();
    let lines:  &Vec< Arc<RwLock<Line>> >   = &method.lines;
    let line:   RwLockReadGuard<'_, Line>   = lines[lineIndex].read().unwrap();

    let     tokensLength: usize = line.tokens.len();
    let mut            j: usize = 0;

    let mut expressionValue: Vec<Token>;
    let mut result: bool;

    let mut token: &Token;
    while j < tokensLength {
        token = &line.tokens[j];

        if token.dataType == TokenType::Word {
            // add method call
            if j+1 < tokensLength && line.tokens[j+1].dataType == TokenType::CircleBracketBegin {
                // check lower first char
                if token.data.starts_with(|c: char| c.is_lowercase()) {
                    expressionValue = line.tokens[j+1].tokens.clone();
                    // todo: multi-param
                    // basic methods
                    result = true;
                    {
                        let method = methodLink.read().unwrap();
                        // go block up
                        if token.data == "go" {
                            if let Some(parentLink) = &line.parent {
                                let parent = parentLink.read().unwrap();
                                if let Some(methodParent) = &method.parent {
                                    searchCondition(parentLink.clone(), methodParent.clone());
                                }
                            }
                        } else
                        // exit block up
                        if token.data == "ex" {
                            println!("ex");
                        } else
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
                        } else 
                        // print
                        if token.data == "print" {
                            print!("{}",
                                formatPrint(
                                    &method.memoryCellExpression(
                                        &mut expressionValue,
                                        0
                                    ).data
                                )
                            );
                        } else 
                        // exec
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
                        } else 
                        // exit
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
unsafe fn searchMemoryCell(lineIndex: usize, methodLink: Arc<RwLock<Method>>) -> bool {
    let mut method: RwLockWriteGuard<'_, Method> = methodLink.write().unwrap();
    let      lines: &Vec< Arc<RwLock<Line>> >    = &method.lines;
    let mut   line: RwLockWriteGuard<'_, Line>   = lines[lineIndex].write().unwrap();

    let           tokens: &Vec<Token>     = &line.tokens;
    let mut tokensLength: usize           = tokens.len();
    let mut            j: usize           = 0;
    let mut       goNext: bool            = true;

    let mut   nameBuffer: String         = String::new();
    let mut   modeBuffer: MemoryCellMode = MemoryCellMode::LockedFinal;
    let mut modeReceived: bool           = false;

    let mut   typeBuffer: TokenType = TokenType::None;
    let mut typeReceived: bool      = false;

    let mut operatorBuffer: TokenType  = TokenType::None;
    let mut    valueBuffer: Vec<Token> = Vec::new();

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
                    // skip name
                    // skip ~
                    // skip ~
                    j += 3;
                // e: variableName~
                } else
                if j+1 < tokensLength && tokens[j+1].dataType == TokenType::Tilde {
                    modeBuffer = MemoryCellMode::LockedVariable;
                    // skip name
                    // skip ~
                    j += 2;
                // e: variableName
                } else {
                    modeBuffer = MemoryCellMode::LockedFinal;
                    // skip name
                    j += 1;
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
                        // skip :
                        // skip type
                        j += 2;
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
        // if searched in methods
        if let Some(memoryCellLink) = method.getMemoryCellByName(&nameBuffer) {
            method.memoryCellOp(
                memoryCellLink, 
                operatorBuffer, 
                Token::newNesting(valueBuffer)
            );
        // if no searched, then create new MemoryCell and equal right value
        } else {
            // memoryCellName - op - value
            // array
            if valueBuffer.len() > 0 && valueBuffer[0].dataType == TokenType::SquareBracketBegin {
                valueBuffer = valueBuffer[0].tokens.clone();
                valueBuffer.retain(|token| token.dataType != TokenType::Comma);
                method.pushMemoryCell(
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
static mut _methods: Vec< Arc<RwLock<Method>> > = Vec::new();

// parse lines
static mut _lineIndex:   usize                    = 0;
static mut _linesLength: usize                    = 0;

pub unsafe fn parseLines(tokenizerLinesLinks: Vec< Arc<RwLock<Line>> >) {
// preparation
    if unsafe{_debugMode} {
        logSeparator(" > AST preparation");
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

    // debug output and read lines
    let startTime: Instant = Instant::now();
    if unsafe{_debugMode} {
        logSeparator(" > AST interpretation");
    }
    // create main method, without name
    let mainMethodLink = 
        Arc::new(
        RwLock::new(
            Method::new(
                String::from("main"),
                tokenizerLinesLinks.clone(),
                None,
            )
        ));
    // argc & argv
    {
        let mut mainMethod = mainMethodLink.write().unwrap();
        // argc
        mainMethod.pushMemoryCell(
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
        mainMethod.pushMemoryCell(
            MemoryCell::new(
                String::from("argv"),
                MemoryCellMode::LockedFinal,
                TokenType::Array,
                Token::newNesting(argv)
            )
        );
    }
    _methods.push(mainMethodLink);

    _linesLength = tokenizerLinesLinks.len();
    readLines(_methods[0].clone(), &mut _lineIndex, &mut _linesLength);
    // duration
    if unsafe{_debugMode} {
        let endTime  = Instant::now();
        let duration = endTime-startTime;
        logSeparator( &format!("?> Parser duration: {:?}",duration) );
    }
}
pub unsafe fn readLines(methodLink: Arc<RwLock<Method>>, lineIndex: &mut usize, linesLength: &mut usize) {
    while _exitCode == false && *lineIndex < *linesLength {
        let lineLink: Arc<RwLock<Line>>;
        {
            let     method: RwLockReadGuard<'_, Method> = methodLink.read().unwrap();
            lineLink = method.lines[*lineIndex].clone();
            let line: RwLockReadGuard<'_, Line>   = lineLink.read().unwrap();
            // check tokens in line
            if line.tokens.len() == 0 {
                *lineIndex += 1;
                continue;
            }
        }
        {
            // search conditions
            if !searchCondition(lineLink, methodLink.clone()) {
                // search methods calls
                if !searchMethodCall(*lineIndex, methodLink.clone()) {
                // search memory cells
                    searchMemoryCell(*lineIndex, methodLink.clone());
                }
            }
            //
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