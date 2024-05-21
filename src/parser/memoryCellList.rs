/*
    Memory Cell List
*/

use crate::parser::memoryCell::*;
use crate::tokenizer::token::*;

#[derive(PartialEq)]
#[derive(Clone)]
pub enum OpType {
    MinusEquals,
}

#[derive(Clone)]
pub struct MemoryCellList {
    pub value: Vec<MemoryCell>,
}
impl MemoryCellList {
    pub fn new() -> Self {
        MemoryCellList { value: Vec::new() }
    }

    pub fn push(&mut self, memoryCell: MemoryCell) {
        self.value.push(memoryCell);
    }

    pub fn last(&self) -> &MemoryCell {
        self.value.last().unwrap()
    }

    pub fn op(&self, name: String, op: OpType, opValue: Token) {
    	print!("{}: ",name);
    	for mc in &self.value {
    		if mc.name == name {
    			println!("{}",mc.name);
    			if op == OpType::MinusEquals {
    				println!("  -= {}",expression(&mut opValue.tokens.clone(), 0).data);
    			}
    		}
    		//
    	}
    }
}

fn expression(value: &mut Vec<Token>, ident: usize) -> Token {
    let identStr: String = " ".repeat(ident*2);
    let mut valueLength: usize = value.len();
    println!("{}-> len {}",identStr,valueLength);

    // 1 number
    if valueLength == 1 {
        if value[0].dataType != TokenType::CircleBracketBegin {
            return value[0].clone();
        }
    }

    //
    let mut i:      usize = 0;
    let mut goNext: bool = true;
    // bracket
    while i < valueLength {
        let token = value[i].clone();
        if token.dataType == TokenType::CircleBracketBegin {
            println!("{}  (<-",identStr);
            value[i] = expression(&mut token.tokens.clone(),ident+1);
        }
        i += 1;
    }
    // * /
    goNext = true;
    i = 0;
    while i < valueLength {
        if valueLength == 1 {
            break;
        }
        if i == 0 {
            i += 1;
            continue;
        }
        let token = value[i].clone();
        if i+1 < valueLength {
            let tokenBack = value[i-1].clone();
            let tokenNext = value[i+1].clone();

            if token.dataType == TokenType::Multiply || token.dataType == TokenType::Divide {
                println!("{}  {}",identStr,Token::getData(&token));

                let left: usize = value[i-1].data.parse().unwrap_or(0);
                let right: usize = value[i+1].data.parse().unwrap_or(0);
                if token.dataType == TokenType::Multiply {
                    value[i-1].data = (left*right).to_string();
                } else {
                    value[i-1].data = (left/right).to_string();
                }
                println!("    1> {} {}:{}",value[i-1].data,left,right);

                value.remove(i); // remove op
                value.remove(i); // remove right value
                goNext = false;
                valueLength -= 2;
            }
        }
        if goNext {
            i += 1;
        } else {
            goNext = true;
        }
    }
    // + -
    goNext = true;
    i = 0;
    while i < valueLength {
        if valueLength == 1 {
            break;
        }
        if i == 0 {
            i += 1;
            continue;
        }
        let token = value[i].clone();
        if i+1 < valueLength {
            let tokenBack = value[i-1].clone();
            let tokenNext = value[i+1].clone();

            if token.dataType == TokenType::Plus || token.dataType == TokenType::Minus {
                println!("{}  {}",identStr,Token::getData(&token));

                let left: usize = tokenBack.data.parse().unwrap_or(0);
                let right: usize = tokenNext.data.parse().unwrap_or(0);
                if token.dataType == TokenType::Plus {
                    value[i-1].data = (left+right).to_string();
                } else {
                    value[i-1].data = (left-right).to_string();
                }
                println!("    2> {} {}:{}",value[i-1].data,left,right);

                value.remove(i); // remove op
                value.remove(i); // remove right value
                goNext = false;
                valueLength -= 2;
            }
        }
        if goNext {
            i += 1;
        } else {
            goNext = true;
        }
    }
    //
    i = 0;
    while i < valueLength {
        print!("{}end: ",identStr);
        print!("{}",Token::getData(&value[i]));
        println!();
        i += 1;
    }

    value[0].clone()
}
