/*
    Enum
*/

use crate::tokenizer::line::*;

#[derive(Clone)]
pub struct Enum {
    pub name:  String,    // unique name
    pub line:  usize,     // defined on line
    pub lines: Vec<Line>, // nesting lines to:do: no lines -> items
}
impl Enum {
    pub fn new(
        name: String,
        line: usize,
        lines: Vec<Line>
    ) -> Self {
        Enum {
            name,
            line,
            lines,
        }
    }
}