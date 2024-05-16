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
fn defineUpperStruct(lines: &mut Vec<Line>, classes: &mut Vec<Class>, enums: &mut Vec<Enum>) {
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
fn defineLowerStruct(lines: &mut Vec<Line>, methods: &mut Vec<Method>, lists: &mut Vec<List>) {
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

// parse lines
pub fn parseLines(lines: &mut Vec<Line>) {
// preparation
    // define upper struct [Class / Enum]
    let mut classes: Vec<Class> = Vec::new();
    let mut enums:   Vec<Enum>  = Vec::new();
    defineUpperStruct(lines, &mut classes, &mut enums);

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
    defineLowerStruct(lines, &mut methods, &mut lists);
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

    // output lines
    log("parserInfo", "Lines");
    let ident_str1: String = " ".repeat(2);
    let ident_str2: String = " ".repeat(4);
    for (i, line) in lines.iter().enumerate() {
        log("parserBegin", &format!("{}+{}", ident_str1, i));
        log("parserHeader", &format!("{}Tokens", ident_str2));
        outputTokens(&line.tokens, 0, 3);
        if (&line.lines).len() > 0 {
            log("parserHeader", &format!("{}Lines", ident_str2));
            outputLines(&line.lines, 3);
        }
        log("parserEnd", &format!("{}-{}", ident_str1, i));
    }
}