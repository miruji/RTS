/*
    class
*/

use crate::tokenizer::line::*;

#[derive(Clone)]
pub struct Method {
    pub name: String,     // name
    pub line: usize,      // def line
    pub lines: Vec<Line>, // lines
}
impl Method {
    pub fn new(name: String, line: usize, lines: Vec<Line>) -> Self {
        Method {
            name,
            line,
            lines,
        }
    }
}