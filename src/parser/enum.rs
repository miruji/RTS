/*
    enum
*/

use crate::tokenizer::line::*;

#[derive(Clone)]
pub struct Enum {
    pub name: String,     // name
    pub line: usize,      // def line
    pub lines: Vec<Line>, // lines to:do: no lines -> items
}
impl Enum {
    pub fn new(name: String, line: usize, lines: Vec<Line>) -> Self {
        Enum {
            name,
            line,
            lines,
        }
    }
}