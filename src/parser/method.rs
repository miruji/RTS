/*
    Method
*/

use crate::logger::*;
use crate::_filePath;

use crate::tokenizer::line::*;
use crate::tokenizer::token::*;

use crate::parser::memoryCellList::*;
use crate::parser::memoryCell::*;

use std::{io, io::Write};

use std::sync::{Arc, RwLock};

pub struct Method {
    pub name:           String,     // unique name
    pub lines:          Vec<Line>,  // nesting lines
    pub parameters:     Vec<Token>, // parameters
    pub resultType:     String,     // result type
        // if result type = None, => procedure
        // else => function
    pub memoryCellList: Arc<RwLock<MemoryCellList>>, // todo: option< Arc<RwLock<MemoryCellList>> > ?
    pub methods:        Vec<    Arc<RwLock<Method>> >,
    pub parent:         Option< Arc<RwLock<Method>> >,
}
impl Method {
    pub fn new(
        name:   String,
        lines:  Vec<Line>,
        parent: Option< Arc<RwLock<Method>> >,
    ) -> Self {
        Method {
            name,
            lines,
            parameters:     Vec::new(),
            resultType:     String::from("None"),
            memoryCellList: Arc::new(RwLock::new(MemoryCellList::new())),
            methods:        Vec::new(),
            parent
        }
    }

    // push memoryCell to self memoryCellList
    pub fn pushMemoryCell(&self, mut memoryCell: MemoryCell) {
        // basic
        if memoryCell.valueType != TokenType::Array {
            memoryCell.value = self.memoryCellExpression(&mut memoryCell.value.tokens.clone(), 0);
        }
        // array
        let mut memoryCellList = self.memoryCellList.write().unwrap();
        memoryCellList.value.push( Arc::new(RwLock::new(memoryCell)) );
    }

    // get memory cell by name
    pub fn getMemoryCellByName(&self, memoryCellName: &str) -> Option<Arc<RwLock<MemoryCell>>> {
        // search in self
        if let Some(memoryCell) = getMemoryCellByName(self.memoryCellList.clone(), memoryCellName) {
            return Some(memoryCell);
        }
        // search in parent
        if let Some(parentLink) = &self.parent {
            let parent = parentLink.read().unwrap();
            return parent.getMemoryCellByName(memoryCellName);
        }
        //
        None
    }

    // memory cell op
    pub fn memoryCellOp(&self, memoryCellLink: Arc<RwLock<MemoryCell>>, op: TokenType, opValue: Token) {
        if op != TokenType::Equals         &&
           op != TokenType::PlusEquals     && op != TokenType::MinusEquals &&
           op != TokenType::MultiplyEquals && op != TokenType::DivideEquals {
            return;
        }

        // calculate new values
        let rightValue: Token = self.memoryCellExpression(&mut opValue.tokens.clone(), 0);
        let mut memoryCell = memoryCellLink.write().unwrap();
        // =
        if op == TokenType::Equals {
            memoryCell.value = rightValue;
        // += -= *= /=
        } else {
            let leftValue: Token = memoryCell.value.clone();
            if op == TokenType::PlusEquals     { memoryCell.value = calculate(&TokenType::Plus,     &leftValue, &rightValue); } else 
            if op == TokenType::MinusEquals    { memoryCell.value = calculate(&TokenType::Minus,    &leftValue, &rightValue); } else 
            if op == TokenType::MultiplyEquals { memoryCell.value = calculate(&TokenType::Multiply, &leftValue, &rightValue); } else 
            if op == TokenType::DivideEquals   { memoryCell.value = calculate(&TokenType::Divide,   &leftValue, &rightValue); }
        }
    }

    // update value
    fn replaceMemoryCellByName(&self, value: &mut Vec<Token>, length: &mut usize, index: usize) {
        if let Some(memoryCellLink) = self.getMemoryCellByName(&value[index].data) {
            let memoryCell = memoryCellLink.read().unwrap();
            if index+1 < *length && value[index+1].dataType == TokenType::SquareBracketBegin {
                let arrayIndex: usize = value[index+1].tokens[0].data.parse::<usize>().unwrap();
                value.remove(index+1);
                *length -= 1;
                value[index].data     = memoryCell.value.tokens[arrayIndex].data.clone();
                value[index].dataType = memoryCell.value.tokens[arrayIndex].dataType.clone();
            } else {
                value[index].data     = memoryCell.value.data.clone();
                value[index].dataType = memoryCell.value.dataType.clone();
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

    // expression
    pub fn memoryCellExpression(&self, value: &mut Vec<Token>, indent: usize) -> Token {
        let identStr: String = " ".repeat(indent*2);
        let mut valueLength: usize = value.len();

        // 1 number
        if valueLength == 1 {
            if value[0].dataType != TokenType::CircleBracketBegin {
                if value[0].dataType == TokenType::Word {
                    self.replaceMemoryCellByName(value, &mut valueLength, 0);
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
                        if value[i+1].tokens.len() > 0 {
                            value[i] = self.memoryCellExpression(&mut value[i+1].tokens.clone(),indent+1);
                            value[i].dataType = TokenType::Int;
                        } else {
                            value[i].data     = String::new();
                            value[i].dataType = TokenType::None;
                        }

                        value.remove(i+1);
                        valueLength -= 1;
                        continue;
                    } else 
                    if functionName == "str" {
                        if value[i+1].tokens.len() > 0 {
                            value[i] = self.memoryCellExpression(&mut value[i+1].tokens.clone(),indent+1);
                            value[i].dataType = TokenType::String;
                        } else {
                            value[i].data     = String::new();
                            value[i].dataType = TokenType::None;
                        }

                        value.remove(i+1);
                        valueLength -= 1;
                        continue;
                    } else 
                    if functionName == "type" {
                        if value[i+1].tokens.len() > 0 {
                            value[i].data = self.memoryCellExpression(&mut value[i+1].tokens.clone(),indent+1).dataType.to_string();
                            value[i].dataType = TokenType::String;
                        } else {
                            value[i].data     = String::new();
                            value[i].dataType = TokenType::None;
                        }

                        value.remove(i+1);
                        valueLength -= 1;
                        continue;
                    } else
                    if functionName == "input" {
                        if value[i+1].tokens.len() > 0 {
                            let inputTextToken: Token = self.memoryCellExpression(&mut value[i+1].tokens.clone(),indent+1);
                            if inputTextToken.dataType != TokenType::None {
                                print!("{}",inputTextToken.data);
                                io::stdout().flush().unwrap();
                            }
                        } else {
                            value[i].data     = String::new();
                            value[i].dataType = TokenType::None;
                        }

                        value[i].data = String::new();
                        io::stdin().read_line(&mut value[i].data).expect("Input error"); // todo: delete error
                        value[i].data = value[i].data.trim_end().to_string();
                        value[i].dataType = TokenType::String;

                        value.remove(i+1);
                        valueLength -= 1;
                        continue;
                    }
                // array & basic cell
                } else {
                    self.replaceMemoryCellByName(value, &mut valueLength, i);
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
                value[i] = self.memoryCellExpression(&mut token.tokens.clone(),indent+1);
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
        // * and /
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
        // + and -
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
            // + and -
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
/*
    pub fn newWithResult(
        name:       String,
        lines:      Vec<Line>,
        resultType: String,
    ) -> Self {
        Method {
            name,
            lines,
            parameters: Vec::new(),
            resultType,
            mcl:        MemoryCellList::new(),
            methods:    Vec::new(),
            //parent:     None,
        }
    }
    pub fn newWithParameters(
        name:       String,
        lines:      Vec<Line>,
        parameters: Vec<Token>,
    ) -> Self {
        Method {
            name,
            lines,
            parameters,
            resultType: String::from("None"),
            mcl:        MemoryCellList::new(),
            methods:    Vec::new(),
            //parent:     None,
        }
    }
    pub fn newFull(
        name:       String,
        lines:      Vec<Line>,
        parameters: Vec<Token>,
        resultType: String,
    ) -> Self {
        Method {
            name,
            lines,
            parameters,
            resultType,
            mcl:        MemoryCellList::new(),
            methods:    Vec::new(),
            //parent:     None,
        }
    }
}
*/