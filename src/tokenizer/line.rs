/*
    line + methods for it
*/

pub mod line {
    use crate::tokenizer::token::token::*;

    #[derive(Clone)]
    pub struct Line {
        pub tokens: Vec<Token>, // list
        pub ident: u8,          // identation
        pub lines: Vec<Line>,   // child lines
    }
}