/*
    Memory Cell List
*/

use crate::logger::*;
use crate::filePath;

use crate::parser::memoryCell::*;
use crate::parser::value::*;
use crate::parser::uf64::*;
use crate::tokenizer::token::*;
use crate::tokenizer::line::*;

// mcl
// repeated use in the same line of sight should be avoided
use std::sync::{Mutex, MutexGuard};
lazy_static! {
    static ref _mcl: Mutex<MemoryCellList> = Mutex::new(MemoryCellList::new());
}
pub fn getMemoryCellList() -> MutexGuard<'static, MemoryCellList> {
    _mcl.lock().unwrap()
}

// MemoryCellList
#[derive(Clone)]
pub struct MemoryCellList {
    pub value: Vec<MemoryCell>,
}
impl MemoryCellList {
    pub fn new() -> Self {
        MemoryCellList { value: Vec::new() }
    }

    pub fn push(&mut self, mut mc: MemoryCell) {
        let mcl = self.clone();
        mc.value = mcl.expression(&mut mc.value.tokens.clone(), 0);
        self.value.push(mc);
    }

    pub fn last(&self) -> &MemoryCell {
        self.value.last().unwrap()
    }

    pub fn getCell(&self, name: &str) -> Option<&MemoryCell> {
        for mc in &self.value {
            if mc.name == name {
                return Some(mc);
            }
        }
        None
    }

    pub fn op(&mut self, name: String, op: TokenType, opValue: Token) {
        //print!("{}: ",name);
        if op != TokenType::Equals &&
           op != TokenType::PlusEquals     && op != TokenType::MinusEquals &&
           op != TokenType::MultiplyEquals && op != TokenType::DivideEquals {
            return;
        }

        let mcl_length: &usize = &self.value.len();
        let mut i = 0;
        while i < *mcl_length {
            let mcl = self.clone();
            let mcConst = self.value[i].clone();
            if mcConst.name == name {
                //println!("{}", mcConst.name);
                let rightValue: Token = mcl.expression(&mut opValue.tokens.clone(), 0);
                let mc = &mut self.value[i];
                // =
                if op == TokenType::Equals {
                    mc.value = rightValue;
                // += -= *= /=
                } else {
                    let leftValue: Token = mc.value.clone();
                    //println!("{}:{}", leftValue.data, rightValue.data);
                    //println!("{}:{}", leftValue.dataType.to_string(), rightValue.dataType.to_string());
                    if op == TokenType::PlusEquals {
                        mc.value = calculate(&TokenType::Plus, &leftValue, &rightValue);
                    } else if op == TokenType::MinusEquals {
                        mc.value = calculate(&TokenType::Minus, &leftValue, &rightValue);
                    } else if op == TokenType::MultiplyEquals {
                        mc.value = calculate(&TokenType::Multiply, &leftValue, &rightValue);
                    } else if op == TokenType::DivideEquals {
                        mc.value = calculate(&TokenType::Divide, &leftValue, &rightValue);
                    }
                }
                //println!("  = {}", mc.value.data);
            }
            i += 1;
        }
    }

    // expression
    pub fn expression(&self, value: &mut Vec<Token>, ident: usize) -> Token {
        let identStr: String = " ".repeat(ident*2);
        let mut valueLength: usize = value.len();

        // 1 number
        if valueLength == 1 {
            if value[0].dataType != TokenType::CircleBracketBegin {
                if value[0].dataType == TokenType::Word {
                    updateValue(self, value, 0);
                }
                return value[0].clone();
            }
        }

        //
        let mut i: usize = 0;
        // bracket
        while i < valueLength {
            let token = value[i].clone();
            if token.dataType == TokenType::CircleBracketBegin {
                value[i] = self.expression(&mut token.tokens.clone(),ident+1);
            }
            i += 1;
        }
        // MemoryCell
        i = 0;
        while i < valueLength {
            if value[i].dataType == TokenType::Word {
                updateValue(self, value, i);
            }

            if valueLength == 1 {
                break;
            }
            i += 1;
        }
        // * /
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
            if i+1 < valueLength && (token.dataType == TokenType::Multiply || token.dataType == TokenType::Divide) {
                value[i-1] = calculate(&token.dataType, &value[i-1], &value[i+1]);

                value.remove(i); // remove op
                value.remove(i); // remove right value
                valueLength -= 2;
                continue;
            }

            i += 1;
        }
        // + -
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
            // + -
            if i+1 < valueLength && (token.dataType == TokenType::Plus || token.dataType == TokenType::Minus) {
                value[i-1] = calculate(&token.dataType, &value[i-1], &value[i+1]);

                value.remove(i); // remove op
                value.remove(i); // remove right value
                valueLength -= 2;
                continue;
            } else
            // value -value2
            if token.dataType == TokenType::Int || token.dataType == TokenType::Float {
                value[i-1] = calculate(&TokenType::Plus, &value[i-1], &value[i]);

                value.remove(i); // remove UInt
                valueLength -= 1;
                continue;
            }

            i += 1;
        }
        //
        value[0].clone()
    }
}
// calculate value
fn calculate(op: &TokenType, left: &Token, right: &Token) -> Token {
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
        _ => Value::Int(0),
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
        _ => Value::Int(0),
    };
    //println!("calc2: {} {} {}",leftValue,op.to_string(),rightValue);
    // next: set type, calculate value, check result type, return
    // set result type
    let mut resultType: TokenType;
    if left.dataType == TokenType::Float || right.dataType == TokenType::Float {
        resultType = TokenType::Float;
    } else
    if left.dataType == TokenType::UFloat || right.dataType == TokenType::UFloat {
        resultType = TokenType::UFloat;
    } else
    if left.dataType == TokenType::Int || right.dataType == TokenType::Int {
        resultType = TokenType::Int;
    } else {
        resultType = TokenType::UInt;
    }
    // calculate
    let resultValue: String = 
        if *op == TokenType::Plus {
            (leftValue+rightValue).to_string()
        } else if *op == TokenType::Minus {
            (leftValue-rightValue).to_string()
        } else if *op == TokenType::Multiply {
            (leftValue*rightValue).to_string()
        } else if *op == TokenType::Divide {
            (leftValue/rightValue).to_string()
        } else {
            "0".to_string()
        };
    // type changed ?
    if resultValue.starts_with('-') {
        if resultType == TokenType::UInt {
            resultType = TokenType::Int;
        } else 
        if resultType == TokenType::UFloat {
            resultType = TokenType::Float;
        }
    }
    //
    return Token::new(resultType, resultValue);
}
// update value
fn updateValue(mcl: &MemoryCellList, value: &mut Vec<Token>, index: usize) {
    if let Some(mc) = mcl.getCell(&value[index].data) {
        value[index].data     = mc.value.data.clone();
        value[index].dataType = mc.value.dataType.clone();
    } else {
        log("syntax","");
        log("path",&format!(
            "{} -> MemoryCell",
            unsafe{&*filePath},
        ));
        Line::outputTokens( &getSavedLine() );
        log("note","An undeclared variable is used");
        logExit();
    }
}
