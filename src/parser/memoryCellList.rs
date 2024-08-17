/*
    Memory Cell List
*/

use crate::logger::*;
use crate::_filePath;

use crate::parser::memoryCell::*;
use crate::parser::value::*;
use crate::parser::uf64::*;
use crate::tokenizer::token::*;
use crate::tokenizer::line::*;

use std::sync::{Arc, RwLock};

// MemoryCellList
#[derive(Clone)]
pub struct MemoryCellList {
    pub value: Vec< Arc<RwLock<MemoryCell>> >,
}
impl MemoryCellList {
    pub fn new() -> Self {
        MemoryCellList { value: Vec::new() }
    }
}

// get memory cell by name
pub fn getMemoryCellByName(memoryCellListLink: Arc<RwLock<MemoryCellList>>, name: &str) -> Option<Arc<RwLock<MemoryCell>>> {
    let memoryCellList = memoryCellListLink.read().unwrap();
    for memoryCellLink in &memoryCellList.value {
        let memoryCell = memoryCellLink.read().unwrap();
        if name == memoryCell.name {
            return Some(memoryCellLink.clone());
        }
    }
    None
}

// calculate value
pub fn calculate(op: &TokenType, left: &Token, right: &Token) -> Token {
    // set types
    //println!("calc1: {} {} {}",left.dataType.to_string(),op.to_string(),right.dataType.to_string());
    let leftValue = match left.dataType {
        TokenType::Int => {
            left.data.parse::<i64>().map(Value::Int).unwrap_or(Value::Int(0))
        },
        TokenType::UInt => {
            left.data.parse::<u64>().map(Value::UInt).unwrap_or(Value::UInt(0))
        },
        TokenType::Float => {
            left.data.parse::<f64>().map(Value::Float).unwrap_or(Value::Float(0.0))
        },
        TokenType::UFloat => {
            left.data.parse::<f64>().map(uf64::from).map(Value::UFloat).unwrap_or(Value::UFloat(uf64::from(0.0)))
        },
        TokenType::Char => {
            left.data.parse::<char>().map(|x| Value::Char(x)).unwrap_or(Value::Char('\0'))
        },
        TokenType::String => {
            left.data.parse::<String>().map(|x| Value::String(x)).unwrap_or(Value::String("".to_string()))
        },
        TokenType::Bool => {
            if left.data == "1" {
                Value::UInt(1)
            } else {
                Value::UInt(0)
            }
        },
        // todo: char
        _ => Value::UInt(0),
    };
    let rightValue = match right.dataType {
        TokenType::Int => {
            right.data.parse::<i64>().map(Value::Int).unwrap_or(Value::Int(0))
        },
        TokenType::UInt => {
            right.data.parse::<u64>().map(Value::UInt).unwrap_or(Value::UInt(0))
        },
        TokenType::Float => {
            right.data.parse::<f64>().map(Value::Float).unwrap_or(Value::Float(0.0))
        },
        TokenType::UFloat => {
            right.data.parse::<f64>().map(uf64::from).map(Value::UFloat).unwrap_or(Value::UFloat(uf64::from(0.0)))
        },
        TokenType::Char => {
            right.data.parse::<char>().map(|x| Value::Char(x)).unwrap_or(Value::Char('\0'))
        },
        TokenType::String => {
            right.data.parse::<String>().map(|x| Value::String(x)).unwrap_or(Value::String("".to_string()))
        },
        TokenType::Bool => {
            if right.data == "1" {
                Value::UInt(1)
            } else {
                Value::UInt(0)
            }
        },
        // todo: char
        _ => Value::UInt(0),
    };
    //println!("calc2: {} {} {}",leftValue,op.to_string(),rightValue);
    // next: set type, calculate value, check result type, return
    let mut resultType: TokenType = TokenType::UInt;
    // calculate
    let resultValue: String = 
        if *op == TokenType::Plus {
            (leftValue + rightValue).to_string()
        } else
        if *op == TokenType::Minus {
            (leftValue - rightValue).to_string()
        } else
        if *op == TokenType::Multiply {
            (leftValue * rightValue).to_string()
        } else
        if *op == TokenType::Divide {
            (leftValue / rightValue).to_string()
        } else
        if *op == TokenType::Inclusion {
            resultType = TokenType::Bool;
            (leftValue.toBool() || rightValue.toBool()).to_string()
        } else
        if *op == TokenType::Joint {
            resultType = TokenType::Bool;
            (leftValue.toBool() && rightValue.toBool()).to_string()
        } else
        if *op == TokenType::Equals {
            resultType = TokenType::Bool;
            (leftValue == rightValue).to_string()
        } else
        if *op == TokenType::NotEquals {
            resultType = TokenType::Bool;
            (leftValue != rightValue).to_string()
        } else
        if *op == TokenType::GreaterThan {
            resultType = TokenType::Bool;
            (leftValue > rightValue).to_string()
        } else
        if *op == TokenType::LessThan {
            resultType = TokenType::Bool;
            (leftValue < rightValue).to_string()
        } else
        if *op == TokenType::GreaterThanOrEquals {
            resultType = TokenType::Bool;
            (leftValue >= rightValue).to_string()
        } else
        if *op == TokenType::LessThanOrEquals {
            resultType = TokenType::Bool;
            (leftValue <= rightValue).to_string()
            // todo: % ^
        } else {
            "0".to_string()
        };
    // set result type
    if resultType != TokenType::Bool {
        // todo: bool
        if left.dataType == TokenType::String || right.dataType == TokenType::String {
            resultType = TokenType::String;
        } else
        if (left.dataType == TokenType::Int || left.dataType == TokenType::Int) && right.dataType == TokenType::Char {
            resultType = left.dataType.clone();
        }
        if left.dataType == TokenType::Char {
            resultType = TokenType::Char;
        } else
        if left.dataType == TokenType::Float  || right.dataType == TokenType::Float {
            resultType = TokenType::Float;
        } else
        if left.dataType == TokenType::UFloat || right.dataType == TokenType::UFloat {
            resultType = TokenType::UFloat;
        } else
        if left.dataType == TokenType::Int    || right.dataType == TokenType::Int {
            resultType = TokenType::Int;
        }
    }
    return Token::new(resultType, resultValue);
}
