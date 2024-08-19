/*
    Method
*/

use crate::logger::*;
use crate::_filePath;
use crate::_exitCode;

use crate::tokenizer::line::*;
use crate::tokenizer::token::*;

use crate::parser::memoryCellList::*;
use crate::parser::memoryCell::*;

use crate::parser::readTokens;
use crate::parser::readLines;
use crate::parser::searchCondition;

use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

use std::{io, io::Write};
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;
use rand::Rng;

pub struct Method {
    pub name:           String,                        // unique name
                                                       // todo: Option
    pub lines:          Vec< Arc<RwLock<Line>> >,      // nesting lines
                                                       // todo: Option
    pub parameters:     Vec<Token>,                    // parameters
                                                       // todo: Option< Arc<RwLock<Token>> >
    pub result:         Option<Token>,                 // result type
        // if result type = None, => procedure
        // else => function
    pub memoryCellList: Arc<RwLock<MemoryCellList>>,   // todo: option< Arc<RwLock<MemoryCellList>> > ?
    pub methods:        Vec<    Arc<RwLock<Method>> >,
    pub parent:         Option< Arc<RwLock<Method>> >,
}
impl Method {
    pub fn new(
        name:   String,
        lines:  Vec< Arc<RwLock<Line>> >,
        parent: Option< Arc<RwLock<Method>> >,
    ) -> Self {
        Method {
            name,
            lines,
            parameters:     Vec::new(),
            result:         None,
            memoryCellList: Arc::new(RwLock::new(MemoryCellList::new())),
            methods:        Vec::new(),
            parent
        }
    }

    // get method by name
    pub fn getMethodByName(&self, name: &str) -> Option<Arc<RwLock<Method>>> {
        for childMethodLink in &self.methods {
            let childMethod = childMethodLink.read().unwrap();
            if name == childMethod.name {
                return Some(childMethodLink.clone());
            }
        }

        // Check the parent method if it exists
        if let Some(parentLink) = &self.parent {
            let parentMethod = parentLink.read().unwrap();
            parentMethod.getMethodByName(name)
        } else {
            None
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
                let arrayIndex = // todo: rewrite if no UInt type ...
                    self.memoryCellExpression(&mut value[index+1].tokens,0).data.parse::<usize>();
                value.remove(index+1);
                *length -= 1;
                match arrayIndex {
                    Ok(idx) => {

                        value[index].data     = memoryCell.value.tokens[idx].data.clone();
                        value[index].dataType = memoryCell.value.tokens[idx].dataType.clone();
                    }
                    Err(_) => {
                        // Обработка ошибки парсинга
                        value[index].data     = String::new();
                        value[index].dataType = TokenType::None;
                    }
                }
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

    // format quote
    fn formatQuote(&self, quote: String) -> String {
        let mut result:           String    = String::new();
        let mut expressionBuffer: String    = String::new();
        let mut expressionRead:   bool      = false;
        let     chars:            Vec<char> = quote.chars().collect();

        let mut i:      usize = 0;
        let     length: usize = chars.len();
        let mut c:      char;

        while i < length {
            c = chars[i];
            if c == '{' {
                expressionRead = true;
            } else
            if c == '}' {
                expressionRead = false;
                expressionBuffer += "\n";
                unsafe{ 
                    let expressionLineLink = &readTokens( expressionBuffer.as_bytes().to_vec(), false )[0];
                    let expressionLine     = expressionLineLink.read().unwrap();
                    let mut expressionBufferTokens: Vec<Token> = expressionLine.tokens.clone();
                    result += &self.memoryCellExpression(&mut expressionBufferTokens,0).data;
                }
                expressionBuffer = String::new();
            } else {
                if expressionRead {
                    expressionBuffer.push(c);
                } else {
                    result.push(c);
                }
            }
            i += 1;
        }
        result
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
                } else 
                if value[0].dataType == TokenType::FormattedRawString ||
                   value[0].dataType == TokenType::FormattedString    ||
                   value[0].dataType == TokenType::FormattedChar {
                    //
                    value[0].data = self.formatQuote(value[0].data.clone());
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
                    // todo: uint float ufloat ...
                    if functionName == "int" {
                        if value[i+1].tokens.len() > 0 {
                            value[i] = self.memoryCellExpression(&mut value[i+1].tokens.clone(),indent+1);
                            value[i].dataType = TokenType::Int;
                            value.remove(i+1);
                        } else {
                            value[i].data     = String::new();
                            value[i].dataType = TokenType::None;
                        }
                        valueLength -= 1;
                        continue;
                    } else 
                    if functionName == "char" {
                        if value[i+1].tokens.len() > 0 {
                            value[i] = self.memoryCellExpression(&mut value[i+1].tokens.clone(),indent+1);
                            value[i].data = (value[i].data.parse::<u8>().unwrap() as char).to_string();
                            value[i].dataType = TokenType::Char;
                            value.remove(i+1);
                        } else {
                            value[i].data     = String::new();
                            value[i].dataType = TokenType::None;
                        }
                        valueLength -= 1;
                        continue;
                    } else 
                    if functionName == "str" {
                        if value[i+1].tokens.len() > 0 {
                            value[i] = self.memoryCellExpression(&mut value[i+1].tokens.clone(),indent+1);
                            value[i].dataType = TokenType::String;
                            value.remove(i+1);
                        } else {
                            value[i].data     = String::new();
                            value[i].dataType = TokenType::None;
                        }
                        valueLength -= 1;
                        continue;
                    } else 
                    if functionName == "type" {
                        if value[i+1].tokens.len() > 0 {
                            value[i].data = self.memoryCellExpression(&mut value[i+1].tokens.clone(),indent+1).dataType.to_string();
                            value[i].dataType = TokenType::String;
                            value.remove(i+1);
                        } else {
                            value[i].data     = String::new();
                            value[i].dataType = TokenType::None;
                        }
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
                            value.remove(i+1);
                        } else {
                            value[i].data     = String::new();
                            value[i].dataType = TokenType::None;
                        }

                        value[i].data = String::new();
                        io::stdin().read_line(&mut value[i].data).expect("Input error"); // todo: delete error
                        value[i].data = value[i].data.trim_end().to_string();
                        value[i].dataType = TokenType::String;

                        valueLength -= 1;
                        continue;
                    } else 
                    if functionName == "randUInt" {
                        // get expressions
                        let mut expressions: Vec<Token> = Vec::new();
                        if value[i+1].tokens.len() > 0 {
                            let mut expressionsBuffer: Vec<Vec<Token>> = Vec::new();
                            {
                                let mut l = 0;
                                let tokens = &value[i+1].tokens;
                                let tokensLength = tokens.len();
                                let mut token;
                                let mut expressionBuffer = Vec::new();
                                while l < tokensLength {
                                    token = tokens[l].clone();
                                    if token.dataType == TokenType::Comma || l+1 == tokensLength {
                                        if l+1 == tokensLength {
                                            expressionBuffer.push( token );
                                        }
                                        expressionsBuffer.push( expressionBuffer.clone() );
                                        expressionBuffer.clear();
                                    } else {
                                        expressionBuffer.push( token );
                                    }
                                    l += 1;
                                }
                            }
                            for mut expression in expressionsBuffer {
                                expressions.push(
                                    self.memoryCellExpression(&mut expression,indent+1)
                                );
                            }
                            value.remove(i+1);
                        }
                        // todo: check errors
                        if expressions.len() == 2 {
                            let mut rng = rand::thread_rng();
                            let min: usize = expressions[0].data.parse::<usize>().unwrap_or(0);
                            let max: usize = expressions[1].data.parse::<usize>().unwrap_or(0);
                            let randomNumber: usize = rng.gen_range(min..=max);

                            value[i].data = randomNumber.to_string();
                            value[i].dataType = TokenType::UInt;
                        } else {
                            value[i].data     = String::new();
                            value[i].dataType = TokenType::None;
                        }

                        valueLength -= 1;
                        continue;
                    } else {
                        let mut lineBuffer = Line::newEmpty();
                        lineBuffer.tokens = value.clone();
                        unsafe{ self.methodCall( Arc::new(RwLock::new(lineBuffer)) ); }

                        // todo: rewrite
                        if let Some(methodLink) = self.getMethodByName(&value[0].data) {
                            let method = methodLink.read().unwrap();
                            if let Some(result) = &method.result {
                                value[i].data     = result.data.clone();
                                value[i].dataType = result.dataType.clone();
                            } else {
                                value[i].data     = String::new();
                                value[i].dataType = TokenType::None;
                            }
                        } else {
                            value[i].data     = String::new();
                            value[i].dataType = TokenType::None;
                        }

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
                (token.dataType == TokenType::Inclusion           || 
                 token.dataType == TokenType::Joint               || 
                 token.dataType == TokenType::Equals              || 
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
        if value.len() > 0 {
            value[0].clone()
        } else {
            Token::newEmpty(TokenType::None)
        }
    }

    /* search methods call
       e:
         methodCall(parameters)
    */
    pub unsafe fn methodCall(&self, lineLink: Arc<RwLock<Line>>) -> bool {
    //    println!("  searchMethodCall");
        let line: RwLockReadGuard<'_, Line> = lineLink.read().unwrap();

        let mut expressionValue: Vec<Token>;
        let mut result: bool;

        let mut token: &Token = &line.tokens[0];
        if line.tokens[0].dataType == TokenType::Word {
            // add method call
            if line.tokens.len() > 1 && line.tokens[1].dataType == TokenType::CircleBracketBegin {
                // check lower first char
                if token.data.starts_with(|c: char| c.is_lowercase()) {
                    expressionValue = line.tokens[1].tokens.clone();
                    // todo: multi-param
                    // basic methods
                    result = true;
                    {
                        // go block up
                        if token.data == "go" {
                            if let Some(parentLink) = &line.parent {
                                let parent = parentLink.read().unwrap();
                                if let Some(methodParent) = &self.parent {
                                    searchCondition(parentLink.clone(), methodParent.clone());
                                }
                            }
                        } else
                        // exit block up
                        if token.data == "ex" {
                            println!("ex");
                        } else
                        // println
                        if token.data == "println" {
                            println!("{}",
                                formatPrint(
                                    &self.memoryCellExpression(
                                        &mut expressionValue,
                                        0
                                    ).data
                                )
                            );
                        } else 
                        // print
                        if token.data == "print" {
                            print!("{}",
                                formatPrint(
                                    &self.memoryCellExpression(
                                        &mut expressionValue,
                                        0
                                    ).data
                                )
                            );
                        } else 
                        // print
                        if token.data == "sleep" {
                            io::stdout().flush().unwrap(); // forced withdrawal of old
                            let value = 
                                &self.memoryCellExpression(
                                    &mut expressionValue,
                                    0
                                ).data;
                            let valueNumber = value.parse::<u64>().unwrap_or(0);
                            sleep(Duration::from_millis(valueNumber));
                        } else 
                        // exec
                        if token.data == "exec" {
                            io::stdout().flush().unwrap(); // forced withdrawal of old
                            let expression: String = self.memoryCellExpression(&mut expressionValue,0).data;
                            let mut parts = expression.split_whitespace();
                            let commandStr = parts.next().expect("No command found in expression");
                            let args: Vec<&str> = parts.collect();
                            let commandOutput = 
                                Command::new(commandStr)
                                    .args(&args)
                                    .output()
                                    .expect("failed to execute process");
                            let outputStr = String::from_utf8_lossy(&commandOutput.stdout);
                            if !outputStr.is_empty() {
                                print!("{}", outputStr);
                            }
                        } else 
                        // exit
                        if token.data == "exit" {
                            _exitCode = true;
                        // custom method
                        } else {
                            result = false;
                        }
                    };
                    // custom methods
                    if !result {
                        if let Some(calledMethodLink) = self.getMethodByName(&token.data) {
                            let mut linesLengthBuffer: usize = 0;
                            let mut   lineIndexBuffer: usize = 0;
                            {
                                let calledMethod = calledMethodLink.read().unwrap();
                                linesLengthBuffer = calledMethod.lines.len();
                            }
                            readLines(calledMethodLink.clone(), &mut lineIndexBuffer, &mut linesLengthBuffer);
                            return true;
                        }
                    }
                    return result;
                } else {
                // read error
                    log("syntax","");
                    log("path",&format!(
                        "{} -> Word \"{}\"",
                        &*_filePath,
                        token.data
                    ));
                    Line::outputTokens( &getSavedLine() );
                    log("note","Method calls and variable names must begin with a lower char");
                    logExit();
                }
            }
        }
        return false;
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