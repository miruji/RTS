/*
    Method
*/

use crate::tokenizer::line::*;
use crate::tokenizer::token::*;

#[derive(Clone)]
pub struct Method {
    pub name:       String,     // unique name
    pub line:       usize,      // defined on line
    pub lines:      Vec<Line>,  // nesting lines
    pub parameters: Vec<Token>, // parameters
    pub resultType: String,     // result type
        // if result type = None, => procedure
        // else => function
}
impl Method {
    pub fn new(
        name:  String,
        line:  usize,
        lines: Vec<Line>
    ) -> Self {
        Method {
            name,
            line,
            lines,
            parameters: Vec::new(),
            resultType: String::from("None")
        }
    }
    pub fn newWithResult(
        name:       String,
        line:       usize,
        lines:      Vec<Line>,
        resultType: String
    ) -> Self {
        Method {
            name,
            line,
            lines,
            parameters: Vec::new(),
            resultType
        }
    }
    pub fn newWithParameters(
        name:       String,
        line:       usize,
        lines:      Vec<Line>,
        parameters: Vec<Token>
    ) -> Self {
        Method {
            name,
            line,
            lines,
            parameters,
            resultType: String::from("None")
        }
    }
    pub fn newFull(
        name:       String,
        line:       usize,
        lines:      Vec<Line>,
        parameters: Vec<Token>,
        resultType: String
    ) -> Self {
        Method {
            name,
            line,
            lines,
            parameters,
            resultType
        }
    }
}