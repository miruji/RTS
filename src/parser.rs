/*
    parser
*/

pub mod class;
pub mod r#enum;
pub mod method;
pub mod list;

use crate::parser::class::*;
use crate::parser::r#enum::*;
use crate::parser::method::*;
use crate::parser::list::*;

use crate::tokenizer::*;
use crate::tokenizer::line::*;

// define upper struct [Class / Enum]
fn define_upper_struct(lines: &mut Vec<Line>, classes: &mut Vec<Class>, enums: &mut Vec<Enum>) {
    let mut i:   usize = 0;
    let mut add: bool = true;
    while i < lines.len() {
        let line = lines[i].clone();
        for token in line.tokens {
            if let Some(first_char) = token.data.chars().next() {
                // fist char upper
                if first_char.is_ascii_uppercase() {
                    // Class or Enum ?
                    // => read this line.lines
                    let mut constr: bool = false;
                    'outer: {
                        if !constr && line.lines.len() > 0 {
                        for child_line in &line.lines {
                            for search_constr in &child_line.tokens {
                                if search_constr.data == "constr" {
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
// define lower struct [function / procedure / list]
fn define_lower_struct(lines: &mut Vec<Line>, methods: &mut Vec<Method>, lists: &mut Vec<List>) {
    let mut i:   usize = 0;
    let mut add: bool = true;
    while i < lines.len() {
        let line = lines[i].clone();
        let line_tokens_length = line.tokens.len();
        for token in line.tokens {
            if let Some(first_char) = token.data.chars().next() {
                // fist char upper
                if first_char.is_ascii_lowercase() {
                    // method or list ?
                    // => read this line.lines
                    let mut list: bool = false;
                    'outer: {
                        if !list && line.lines.len() > 0 {
                        for child_line in &line.lines {
                            for search_key in &child_line.tokens {
                                if let Some(fc) = search_key.data.chars().next() {
                                    if fc == '"' {
                                        lists.push( List::new(token.data.clone(), i, line.lines.clone()) );
                                        lines.remove(i);
                                        add = false;

                                        list = true;
                                        break 'outer;
                                    }
                                }
                                //
                            }
                        } }
                    }
                    // else type = method
                    // to:do: method(parameters) -> result
                    if !list && !line.lines.is_empty() && line_tokens_length == 1 {
                        methods.push( Method::new(token.data.clone(), i, line.lines.clone()) );
                        lines.remove(i);
                        add = false;
                    }
                    break;
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
}

// parse lines
pub fn parse_lines(lines: &mut Vec<Line>) {
// preparation
    // define upper struct [Class / Enum]
    let mut classes: Vec<Class> = Vec::new();
    let mut enums:   Vec<Enum>  = Vec::new();
    define_upper_struct(lines, &mut classes, &mut enums);
    // define lower struct [function / procedure / list]
    let mut methods: Vec<Method> = Vec::new();
    let mut lists:   Vec<List>   = Vec::new();
    define_lower_struct(lines, &mut methods, &mut lists);

// read lines
    println!("[LOG][INFO] Classes:");
    for c in classes {
        println!("  & {}", c.name);
        println!("    Line: {}", c.line);
        println!("    Lines:");
        output_lines(&c.lines, 2);
    }

    println!("[LOG][INFO] Enums:");
    for e in enums {
        println!("  & {}", e.name);
        println!("    Line: {}", e.line);
        println!("    Lines:");
        output_lines(&e.lines, 2);
    }

    println!("[LOG][INFO] Lists:");
    for l in lists {
        println!("  & {}", l.name);
        println!("    Line: {}", l.line);
        println!("    Lines:");
        output_lines(&l.lines, 2);
    }

    println!("[LOG][INFO] Methods:");
    for m in methods {
        println!("  @ {}", m.name);
        println!("    Line: {}", m.line);
        println!("    Lines:");
        output_lines(&m.lines, 2);
    }

    // lines
    println!("[LOG][INFO] Lines:");
    let ident_str1: String = " ".repeat(2);
    let ident_str2: String = " ".repeat(4);
    for (i, line) in lines.iter().enumerate() {
        println!("{}* {}", ident_str1, i);
        println!("{}Tokens:", ident_str2);
        output_tokens(&line.tokens, 0, 0);
        if (&line.lines).len() > 0 {
            println!("{}Lines:", ident_str2);
            output_lines(&line.lines, 2);
        }
        println!("{}.", ident_str1);
    }
}