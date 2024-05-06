/*
    class
*/

use crate::tokenizer::line::*;

#[derive(Clone)]
pub struct List {
    pub name: String,     // name
    pub line: usize,      // def line
    pub lines: Vec<Line>, // lines
}
impl List {
    pub fn new(name: String, line: usize, lines: Vec<Line>) -> Self {
        List {
            name,
            line,
            lines,
        }
    }
}