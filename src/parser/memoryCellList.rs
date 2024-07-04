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
        // set expression value
        if mc.valueType != TokenType::Array {
            mc.value = mcl.expression(&mut mc.value.tokens.clone(), 0);
        }
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

        let mut mcl:     MemoryCellList;
        let mut mcConst: MemoryCell;
        let mut mc:      &mut MemoryCell;

        let mut leftValue:  Token;
        let mut rightValue: Token;

        while i < *mcl_length {
            mcl     = self.clone();
            mcConst = self.value[i].clone();

            if mcConst.name == name {
                //println!("{}", mcConst.name);
                rightValue = mcl.expression(&mut opValue.tokens.clone(), 0);
                mc = &mut self.value[i];
                // =
                if op == TokenType::Equals {
                    mc.value = rightValue;
                // += -= *= /=
                } else {
                    leftValue = mc.value.clone();
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
                    updateValue(self, value, &mut valueLength, 0);
                }
                return value[0].clone();
            }
        }

        //
        let mut i: usize = 0;
        let mut token: Token;
        // MemoryCell & function
        while i < valueLength {
            if value[i].dataType == TokenType::Word {
                // function
                if i+1 < valueLength && value[i+1].dataType == TokenType::CircleBracketBegin {
                    let functionName: String = value[i].data.clone();
                    // todo: uint float ufloat
                    if functionName == "int" {
                        token = value[i].clone();
                        value[i] = self.expression(&mut value[i+1].tokens.clone(),ident+1);
                        value[i].dataType = TokenType::Int;

                        value.remove(i+1);
                        valueLength -= 1;
                        continue;
                    } else 
                    if functionName == "str" {
                        token = value[i].clone();
                        value[i] = self.expression(&mut value[i+1].tokens.clone(),ident+1);
                        value[i].dataType = TokenType::String;

                        value.remove(i+1);
                        valueLength -= 1;
                        continue;
                    } else 
                    if functionName == "type" {
                        token = value[i].clone();
                        value[i].data = self.expression(&mut value[i+1].tokens.clone(),ident+1).dataType.to_string();
                        value[i].dataType = TokenType::String;

                        value.remove(i+1);
                        valueLength -= 1;
                        continue;
                    }
                // array & basic cell
                } else {
                    updateValue(self, value, &mut valueLength, i);
                }
            }

            if valueLength == 1 {
                break;
            }
            i += 1;
        }
        // bracket
        i = 0;
        while i < valueLength {
            token = value[i].clone();
            if token.dataType == TokenType::CircleBracketBegin {
                value[i] = self.expression(&mut token.tokens.clone(),ident+1);
            }
            i += 1;
        }
        // =
        i = 0;
        while i < valueLength {
            if valueLength == 1 {
                break;
            }
            if i == 0 {
                i += 1;
                continue;
            }

            token = value[i].clone();
            if i+1 < valueLength && 
                (token.dataType == TokenType::Equals              || 
                 token.dataType == TokenType::NotEquals           ||
                 token.dataType == TokenType::GreaterThan         || 
                 token.dataType == TokenType::LessThan            ||
                 token.dataType == TokenType::GreaterThanOrEquals || 
                 token.dataType == TokenType::LessThanOrEquals) {
                value[i-1] = calculate(&token.dataType, &value[i-1], &value[i+1]);
                
                value.remove(i); // remove op
                value.remove(i); // remove right value
                valueLength -= 2;
                continue;
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

            token = value[i].clone();
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

            token = value[i].clone();
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
        TokenType::Char => {
            left.data.parse::<char>().map(|x| Value::Char(x)).unwrap_or(Value::Char('\0'))
        },
        TokenType::String => {
            left.data.parse::<String>().map(|x| Value::String(x)).unwrap_or(Value::String("".to_string()))
        },
        TokenType::True => {
            Value::UInt(1)
        },
        TokenType::False => {
            Value::UInt(0)
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
        TokenType::True => {
            Value::UInt(1)
        },
        TokenType::False => {
            Value::UInt(0)
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
// update value
fn updateValue(mcl: &MemoryCellList, value: &mut Vec<Token>, length: &mut usize, index: usize) {
    if let Some(mc) = mcl.getCell(&value[index].data) {
        if index+1 < *length && value[index+1].dataType == TokenType::SquareBracketBegin {
            let arrayIndex: usize = value[index+1].tokens[0].data.parse::<usize>().unwrap();
            value.remove(index+1);
            *length -= 1;
            value[index].data     = mc.value.tokens[arrayIndex].data.clone();
            value[index].dataType = mc.value.tokens[arrayIndex].dataType.clone();
        } else {
            value[index].data     = mc.value.data.clone();
            value[index].dataType = mc.value.dataType.clone();
        }
    } else {
        log("syntax","");
        log("path",&format!(
            "{} -> MemoryCell",
            unsafe{&*_filePath},
        ));
        Line::outputTokens( &getSavedLine() );
        log("note",&format!(
            "An undeclared variable \"{}\" is used",
            value[index].data
        ));
        logExit();
    }
}
