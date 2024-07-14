/*
    line
*/

use crate::logger::*;
use crate::tokenizer::token::*;

// saved line for logger
// repeated use in the same line of sight should be avoided
use std::sync::{Mutex, MutexGuard};
lazy_static! {
    static ref _savedLine: Mutex<Line> = Mutex::new(Line::newEmpty());
}
pub fn getSavedLine() -> MutexGuard<'static, Line> {
    _savedLine.lock().unwrap()
}
pub fn replaceSavedLine(newLine: Line) {
    let mut guard = _savedLine.lock().unwrap();
    *guard = newLine;
}

// Line
#[derive(Clone)]
pub struct Line {
    pub tokens:       Vec<Token>, // list
    pub indent:        usize,     // indentation
    pub lines:        Vec<Line>,  // child lines
    pub linesDeleted: usize,      // deleted lines
}
impl Line {
    pub fn newEmpty() -> Self {
        Line {
            tokens:       Vec::new(),
            indent:        0,
            lines:        Vec::new(),
            linesDeleted: 0
        }
    }
    pub fn outputTokens(line: &Line) {
        let mut tokensString: String = String::new();
        for token in &line.tokens {
            tokensString.push_str( &Token::getData(&token) );
        }
        log("line",
            &format!(
                "{}|{}",
                line.linesDeleted.to_string(),
                &tokensString
            )
        );
        //
    }
}