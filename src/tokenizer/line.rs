/*
    line
*/

use crate::logger::*;
use crate::tokenizer::token::*;

use std::sync::{Arc, RwLock};

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
    pub tokens:       Vec<Token>,                  // list
                                                   // todo: Option<Vec< Arc<RwLock<Token>> >>
    pub indent:       usize,                       // indentation
    pub lines:        Vec< Arc<RwLock<Line>> >,    // child lines
                                                   // todo: Option
    pub linesDeleted: usize,                       // deleted lines
    pub parent:       Option< Arc<RwLock<Line>> >  // parent link
}
impl Line {
    pub fn newEmpty() -> Self {
        Line {
            tokens:       Vec::new(),
            indent:       0,
            lines:        Vec::new(),
            linesDeleted: 0,
            parent:       None
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