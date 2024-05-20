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
    pub fn outputTokens(token: &Line) {
        let mut tokensString: String = String::new();
        for token in &token.tokens {
            if token.data.is_empty() {
                tokensString.push_str(&token.dataType.to_string());
            } else {
                tokensString.push_str(&token.data);
            }
        }
        log("line",
            &format!(
                "{}|{}",
                token.linesDeleted.to_string().as_str(),
                &tokensString
            )
        );
        //
    }
}