/*
  line
*/

use crate::logger::*;
use crate::tokenizer::token::*;

use std::sync::{Arc, RwLock};

// Line
#[derive(Clone)]
pub struct Line {
  pub       tokens: Vec<Token>,                  // list
                                                 // todo: Option<Vec< Arc<RwLock<Token>> >>
  pub       indent: usize,                       // indentation
                                                 // todo: Option
  pub        index: usize,                       // index
                                                 // todo: Option
  pub        lines: Vec< Arc<RwLock<Line>> >,    // child lines
                                                 // todo: Option
  pub       parent: Option< Arc<RwLock<Line>> >  // parent link
}
impl Line 
{
  pub fn newEmpty() -> Self 
  {
    Line 
    {
      tokens: Vec::new(),
      indent: 0,
       index: 0,
       lines: Vec::new(),
      parent: None
    }
  }
}