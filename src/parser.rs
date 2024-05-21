/*
    parser
*/

use crate::logger::*;
use crate::filePath;

pub mod class;
pub mod r#enum;
pub mod method;
pub mod list;

use crate::parser::class::*;
use crate::parser::r#enum::*;
use crate::parser::method::*;
use crate::parser::list::*;

use crate::tokenizer::*;
use crate::tokenizer::token::*;
use crate::tokenizer::line::*;

// define upper struct [Class / Enum]
unsafe fn defineUpperStruct(classes: &mut Vec<Class>, enums: &mut Vec<Enum>) {
    let mut i:   usize = 0;
    let mut add: bool = true;
    while i < lines.len() {
        let line = lines[i].clone();
        for token in &line.tokens {
        if let Some(firstChar) = token.data.chars().next() {
            // fist char upper
            if firstChar.is_ascii_uppercase() {
                // Class or Enum ?
                // => read this line.lines
                let mut constr: bool = false;
                'outer: {
                    if !constr && line.lines.len() > 0 {
                    for childLine in &line.lines {
                        for searchConstr in &childLine.tokens {
                            if searchConstr.data == "constr" {
                                classes.push( Class::new(token.data.clone(), i, line.lines.clone()) );
                                lines.remove(i);
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
                    lines.remove(i);
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
    while i < lines.len() {
        let line = lines[i].clone();
        let lineTokensLength = line.tokens.len();
        for token in &line.tokens {
        if let Some(firstChar) = token.data.chars().next() {
            // fist char upper
            if firstChar.is_ascii_lowercase() {
                // method or list ?
                // => read this line.lines
                let mut list: bool = false;
                'outer: {
                    if !list && line.lines.len() > 0 {
                    for childLine in &line.lines {
                        for searchKey in &childLine.tokens {
                            if searchKey.dataType == TokenType::DoubleQuote {
                                lists.push( List::new(token.data.clone(), i, line.lines.clone()) );
                                lines.remove(i);
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
                    
                    lines.remove(i);
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

/* search methods calls
   e:
     methodCall(parameters)
*/
unsafe fn searchMethodsCalls(line: &mut Line) {
    let lineSave: Line = line.clone(); // save line now for logger

    let tokens:       &mut Vec<Token> = &mut line.tokens;
    let tokensLength: usize           = tokens.len();
    let mut j: usize = 0;
    while j < tokensLength {
        let token = &tokens[j];
        if token.dataType == TokenType::Word {

            // add method call
            if j+1 < tokensLength && tokens[j+1].dataType == TokenType::CircleBracketBegin {
                // check lower first char
                if token.data.starts_with(|c: char| c.is_lowercase()) {
                    tokens[j].dataType = TokenType::MethodCall;
                    tokens[j].tokens = tokens[j+1].tokens.clone();
                    tokens.remove(j+1);
                    break;
                } else {
                // read error
                    log("syntax","");
                    log("path",&format!(
                        "{} -> Word \"{}\"",
                        unsafe{&*filePath},
                        token.data
                    ));
                    Line::outputTokens(&lineSave);
                    log("note","Method calls and variable names must begin with a lower char");
                    logExit();
                }
            }
        }

        j += 1;
    }
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
           dataType == TokenType::Increment      || // ++
           dataType == TokenType::PlusEquals     || // +=
           dataType == TokenType::Decrement      || // --
           dataType == TokenType::MinusEquals    || // -=
           dataType == TokenType::MultiplyEquals || // *=
           dataType == TokenType::DivideEquals      // /=
        {
            true
            // todo: complex number
            // and other types
        } else {
            false
        }
}
/* search conditional memory cell
   e:
     varName   -> final    locked
     varName~  -> variable locked
     varName~~ -> variable unlocked
*/
unsafe fn searchConditionalMemoryCell(line: &mut Line) {
    let tokens:           &mut Vec<Token> = &mut line.tokens;
    let mut tokensLength: usize           = tokens.len();
    let mut j:            usize           = 0;
    let mut goNext:       bool            = true;

    let mut nameBuffer:     String    = String::new();
    let mut modeBuffer:     TokenType = TokenType::None;
    let mut modeReceived:   bool      = false;

    let mut typeBuffer:     TokenType = TokenType::None;
    let mut typeReceived:   bool      = false;

    let mut operatorBuffer: TokenType  = TokenType::None;
    let mut valueBuffer:    Vec<Token> = Vec::new();

    while j < tokensLength {

        let token = &tokens[j];
        if token.dataType == TokenType::Word || modeReceived == true {
            // check mode
            if !modeReceived {
                nameBuffer = token.data.clone();
                // e: variableName~~
                if j+2 < tokensLength && tokens[j+2].dataType == TokenType::Tilde {
                    modeBuffer = TokenType::UnlockedVariable;
                    tokens.remove(j); // remove name
                    tokens.remove(j); // remove ~
                    tokens.remove(j); // remove ~
                    tokensLength -= 3;
                // e: variableName~
                } else
                if j+1 < tokensLength && tokens[j+1].dataType == TokenType::Tilde {
                    modeBuffer = TokenType::LockedVariable;
                    tokens.remove(j); // remove name
                    tokens.remove(j); // remove ~
                    tokensLength -= 2;
                // e: variableName
                } else {
                    modeBuffer = TokenType::LockedFinal;
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
        println!("    ! Memory Cell \"{}\":", nameBuffer);
        println!("      Mode     : \"{}\"",   modeBuffer.to_string());
        println!("      Type     : \"{}\"",   typeBuffer.to_string());
        println!("      Operator : \"{}\"",   operatorBuffer.to_string());
        if !valueBuffer.is_empty() {
            println!("      Value");
            outputTokens(&valueBuffer, 0, 4);
        }
    }
}

// parse lines
static mut lines: Vec<Line> = Vec::new();
pub unsafe fn parseLines(tokenizerLines: Vec<Line>) {
// preparation
    lines = tokenizerLines;

    // define upper struct [Class / Enum]
    let mut classes: Vec<Class> = Vec::new();
    let mut enums:   Vec<Enum>  = Vec::new();
    defineUpperStruct(&mut classes, &mut enums);

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

    // define lower struct [function / procedure / list]
    let mut methods: Vec<Method> = Vec::new();
    let mut lists:   Vec<List>   = Vec::new();
    defineLowerStruct(&mut methods, &mut lists);
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

// read lines
    log("parserInfo", "Lines");
    let ident_str1: String = " ".repeat(2);
    let ident_str2: String = " ".repeat(4);

    let mut i:       usize = 0;
    let linesLength: usize = lines.len();
    while i < linesLength {
        log("parserBegin", &format!("{}+{}", ident_str1, i));
        let line = &mut lines[i];

        // search methods calls
        searchMethodsCalls(line);
        searchConditionalMemoryCell(line);

        // output
        if !line.tokens.is_empty() {
            log("parserHeader", &format!("{}Tokens", ident_str2));
            outputTokens(&line.tokens, 0, 3);
        }

        if !line.lines.is_empty() {
            log("parserHeader", &format!("{}Lines", ident_str2));
            outputLines(&line.lines, 3);
        }

        //
        log("parserEnd", &format!("{}-{}", ident_str1, i));
        i += 1;
    }
}