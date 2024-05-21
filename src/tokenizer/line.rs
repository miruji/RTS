/*
    line
*/

use crate::logger::*;
use crate::tokenizer::token::*;

#[derive(Clone)]
pub struct Line {
    pub tokens:       Vec<Token>, // list
    pub ident:        usize,      // identation
    pub lines:        Vec<Line>,  // child lines
    pub linesDeleted: usize,      // deleted lines
}
impl Line {
    pub fn outputTokens(line: &Line) {
        let mut tokensString: String = String::new();
        for token in &line.tokens {
            tokensString.push_str( &Token::getTokenData(&token) );
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