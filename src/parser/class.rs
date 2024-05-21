/*
    Class
*/

use crate::tokenizer::line::*;

#[derive(Clone)]
pub struct Class {
    pub name:  String,    // unique name
    pub line:  usize,     // defined on line
    pub lines: Vec<Line>, // nesting lines
}
impl Class {
    pub fn new(
        name: String,
        line: usize,
        lines: Vec<Line>
    ) -> Self {
        Class {
            name,
            line,
            lines,
        }
    }
}