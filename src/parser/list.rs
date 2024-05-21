/*
    List
*/

use crate::tokenizer::line::*;

#[derive(Clone)]
pub struct List {
    pub name:  String,    // unique name
    pub line:  usize,     // defined on line
    pub lines: Vec<Line>, // nesting lines
}
impl List {
    pub fn new(
        name: String,
        line: usize,
        lines: Vec<Line>
    ) -> Self {
        List {
            name,
            line,
            lines,
        }
    }
}