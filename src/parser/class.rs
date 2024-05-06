/*
    class
*/

use crate::tokenizer::line::*;

#[derive(Clone)]
pub struct Class {
    pub name: String,     // name
    pub line: usize,      // def line
    pub lines: Vec<Line>, // lines
}
impl Class {
    pub fn new(name: String, line: usize, lines: Vec<Line>) -> Self {
        Class {
            name,
            line,
            lines,
        }
    }
}