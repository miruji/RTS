/*
    Method
*/

use crate::logger::*;
use crate::_exitCode;

use crate::tokenizer::line::*;
use crate::tokenizer::token::*;

use crate::parser::readTokens;
use crate::parser::readLines;
use crate::parser::value::*;
use crate::parser::uf64::*;

use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

use std::{io, io::Write};
use std::{str::SplitWhitespace};
use std::process::{Command, Output};
use std::thread::sleep;
use std::time::Duration;
use rand::Rng;

// calculate value
pub fn calculate(op: &TokenType, leftToken: &Token, rightToken: &Token) -> Token 
{
  // get values of types
  let leftTokenData:     String    = leftToken.getData().unwrap_or_default();
  let leftTokenDataType: TokenType = leftToken.getDataType().unwrap_or_default();
  let leftValue = match leftTokenDataType
  {
    TokenType::Int => 
    {
      leftTokenData.parse::<i64>()
        .map(Value::Int)
        .unwrap_or(Value::Int(0))
    },
    TokenType::UInt => 
    {
      leftTokenData.parse::<u64>()
        .map(Value::UInt)
        .unwrap_or(Value::UInt(0))
    },
    TokenType::Float => 
    {
      leftTokenData.parse::<f64>()
        .map(Value::Float)
        .unwrap_or(Value::Float(0.0))
    },
    TokenType::UFloat => 
    {
      leftTokenData.parse::<f64>()
        .map(uf64::from)
        .map(Value::UFloat)
        .unwrap_or(Value::UFloat(uf64::from(0.0)))
    },
    TokenType::Char => 
    {
      leftTokenData.parse::<char>()
        .map(|x| Value::Char(x))
        .unwrap_or(Value::Char('\0'))
    },
    TokenType::String => 
    {
      leftTokenData.parse::<String>()
        .map(|x| Value::String(x))
        .unwrap_or(Value::String("".to_string()))
    },
    TokenType::Bool => 
    {
      if leftTokenData == "1" { Value::UInt(1) } 
      else                    { Value::UInt(0)}
    },
    _ => Value::UInt(0),
  };
  let rightTokenData:     String    = rightToken.getData().unwrap_or_default();
  let rightTokenDataType: TokenType = rightToken.getDataType().unwrap_or_default();
  let rightValue = match rightTokenDataType {
    TokenType::Int    => 
    { 
      rightTokenData.parse::<i64>()
        .map(Value::Int)
        .unwrap_or(Value::Int(0)) 
    },
    TokenType::UInt   => 
    { 
      rightTokenData.parse::<u64>()
        .map(Value::UInt)
        .unwrap_or(Value::UInt(0)) 
    },
    TokenType::Float  => 
    { 
      rightTokenData.parse::<f64>()
        .map(Value::Float)
        .unwrap_or(Value::Float(0.0)) 
    },
    TokenType::UFloat => 
    { 
      rightTokenData.parse::<f64>()
        .map(uf64::from)
        .map(Value::UFloat)
        .unwrap_or(Value::UFloat(uf64::from(0.0))) 
    },
    TokenType::Char   => 
    { 
      rightTokenData.parse::<char>()
        .map(|x| Value::Char(x))
        .unwrap_or(Value::Char('\0')) 
    },
    TokenType::String => 
    { 
      rightTokenData.parse::<String>()
        .map(|x| Value::String(x))
        .unwrap_or(Value::String("".to_string())) 
    },
    TokenType::Bool   => 
    { 
      if rightTokenData == "1" { Value::UInt(1) } 
      else                     { Value::UInt(0) } 
    },
    _ => Value::UInt(0),
  };
  // calculate and set pre-result type
  let mut resultType: TokenType = TokenType::UInt;
  let resultValue: String = match *op 
  {
    TokenType::Plus     => (leftValue + rightValue).to_string(),
    TokenType::Minus    => (leftValue - rightValue).to_string(),
    TokenType::Multiply => (leftValue * rightValue).to_string(),
    TokenType::Divide   => (leftValue / rightValue).to_string(),
    TokenType::Inclusion           => 
    { 
      resultType = TokenType::Bool; 
      (leftValue.toBool() || rightValue.toBool()).to_string() 
    }
    TokenType::Joint               => 
    { 
      resultType = TokenType::Bool; 
      (leftValue.toBool() && rightValue.toBool()).to_string() 
    }
    TokenType::Equals              => 
    { 
      resultType = TokenType::Bool; 
      (leftValue == rightValue).to_string() 
    }
    TokenType::NotEquals           => 
    { 
      resultType = TokenType::Bool; 
      (leftValue != rightValue).to_string() 
    }
    TokenType::GreaterThan         => 
    { 
      resultType = TokenType::Bool; 
      (leftValue > rightValue).to_string() 
    }
    TokenType::LessThan            => 
    { 
      resultType = TokenType::Bool; 
      (leftValue < rightValue).to_string() 
    }
    TokenType::GreaterThanOrEquals => 
    { 
      resultType = TokenType::Bool; 
      (leftValue >= rightValue).to_string() 
    }
    TokenType::LessThanOrEquals    => 
    { 
      resultType = TokenType::Bool; 
      (leftValue <= rightValue).to_string() 
    }
    _ => "0".to_string(),
  };
  // set result type
  if resultType != TokenType::Bool 
  {
    if leftTokenDataType == TokenType::String || rightTokenDataType == TokenType::String 
    {
      resultType = TokenType::String;
    } else
    if (leftTokenDataType == TokenType::Int   || leftTokenDataType == TokenType::UInt) && // todo: ?
        rightTokenDataType == TokenType::Char 
    {
      resultType = leftTokenDataType.clone();
    } else
    if leftTokenDataType == TokenType::Char 
    {
      resultType = TokenType::Char;
    } else
    if leftTokenDataType == TokenType::Float  || rightTokenDataType == TokenType::Float 
    {
      resultType = TokenType::Float;
    } else
    if leftTokenDataType == TokenType::UFloat || rightTokenDataType == TokenType::UFloat 
    {
      resultType = TokenType::UFloat;
    } else
    if leftTokenDataType == TokenType::Int    || rightTokenDataType == TokenType::Int 
    {
      resultType = TokenType::Int;
    }
  }
  return Token::new( Some(resultType), Some(resultValue) );
}

// get structure result type
pub fn getStructureResultType(word: String) -> TokenType 
{
  match word.as_str() 
  {
    "Int"      => TokenType::Int,
    "UInt"     => TokenType::UInt,
    "Float"    => TokenType::Float,
    "UFloat"   => TokenType::UFloat,
    "Rational" => TokenType::Rational,
    "Complex"  => TokenType::Complex,
    "Array"    => TokenType::Array,
    "Char"     => TokenType::Char,
    "String"   => TokenType::String,
    "Bool"     => TokenType::Bool,
    _ => TokenType::Custom(word),
  }
}

pub struct Structure 
{
  pub           name: String,                        // unique name
                                                     // todo: Option
  pub          lines: Vec< Arc<RwLock<Line>> >,      // nesting lines
                                                     // todo: Option
  pub     parameters: Option< Vec<Token> >,          // parameters
  pub         result: Option<Token>,                 // result type
      // if result type = None, => procedure
      // else => function
  pub     structures: Option< Vec< Arc<RwLock<Structure>> > >,
  pub         parent: Option< Arc<RwLock<Structure>> >,
}
impl Structure 
{
  pub fn new
  (
      name: String,
     lines: Vec< Arc<RwLock<Line>> >,
    parent: Option< Arc<RwLock<Structure>> >,
  ) -> Self 
  {
    Structure 
    {
                name,
               lines,
          parameters: None, // todo: remove
              result: None,
          structures: None,
              parent
    }
  }

  // get Structure by name
  pub fn getStructureByName(&self, name: &str) -> Option<Arc<RwLock<Structure>>> 
  {
    if let Some(someStructures) = &self.structures 
    {
      for childStructureLink in someStructures 
      {
        let childStructure: RwLockReadGuard<'_, Structure> = childStructureLink.read().unwrap();
        if name == childStructure.name 
        {
          return Some(childStructureLink.clone());
        }
      }
    }
    // check the parent structure if it exists
    if let Some(parentLink) = &self.parent 
    {
      if let Ok(parentStructure) = parentLink.try_read() 
      {
        parentStructure.getStructureByName(name)
      } else 
      {
        None
      }
    } else { None }
  }

  // push memoryCell to self memoryCellList
  pub fn pushStructure(&mut self, mut structure: Structure) -> ()
  { 
    // if self.structures == None, create new
    if self.structures.is_none() 
    {
      self.structures = Some(vec!());
    }
    // add new structure
    if let Some(ref mut structures) = self.structures 
    {
      structures.push( Arc::new(RwLock::new(structure)) );
    }
  }

  // get structure nesting
  fn setStructureNesting(&self, structureNesting: &Vec<Token>, structureLines: &Vec< Arc<RwLock<Line>> >, newTokens: Vec<Token>) -> () 
  {
    println!("structureNesting [{}]",structureNesting.len());
    if structureNesting.len()-1 > 1 
    { // go next
      let nextStructureNesting: &[Token] = &structureNesting[1..];
    } else 
    {
      let nestingNum: usize = 
        structureNesting[0]
          .getData().unwrap_or_default()
          .parse::<usize>().unwrap_or_default();
      if let Some(nestingLine) = structureLines.get( nestingNum ) 
      {
        let mut nestingLine = nestingLine.write().unwrap(); // todo: type
        println!("structureLines [{:?}]",nestingLine.tokens);
        nestingLine.tokens = newTokens;
      }

    }
  }

  // structure op
  pub fn structureOp(&mut self, structureLink: Arc<RwLock<Structure>>, op: TokenType, leftValue: Vec<Token>, rightValue: Vec<Token>) -> ()
  {
    if op != TokenType::Equals         &&
       op != TokenType::PlusEquals     && op != TokenType::MinusEquals &&
       op != TokenType::MultiplyEquals && op != TokenType::DivideEquals 
      { return; }

    // calculate new values
    /*
    let rightValue: Token = 
      if let Some(mut opValueTokens) = rightValue.tokens.clone() 
      {
        self.expression(&mut opValueTokens)
      } else 
      { // error
        Token::newEmpty(None)
      };
    */
    let mut structure = structureLink.write().unwrap();
    // =
    if op == TokenType::Equals 
    {
      println!("  Equals, leftValue {:?}",leftValue);
      let mut structureNesting: Vec<Token> = Vec::new();
      for value in leftValue 
      {
        if value.getDataType().unwrap_or_default() == TokenType::SquareBracketBegin 
        {
          if let Some(mut valueTokens) = value.tokens 
          {
            structureNesting.push( self.expression(&mut valueTokens) );
          }
        }
      }
      if structureNesting.len() > 0 
      { // nesting
        self.setStructureNesting(&structureNesting, &structure.lines, rightValue);
      } else 
      { // not nesting
        structure.lines = 
          vec![ 
            Arc::new(RwLock::new( 
              Line {
                tokens: rightValue,
                indent: 0,
                index:  0,
                lines:  None,
                parent: None
              }
            ))
          ];
      }
    } else 
    { // += -= *= /=
      println!("  Else");
      //let leftValue: Token = structure.value.clone();
      //if op == TokenType::PlusEquals     { structure.value = calculate(&TokenType::Plus,     &leftValue, &rightValue); } else 
      //if op == TokenType::MinusEquals    { structure.value = calculate(&TokenType::Minus,    &leftValue, &rightValue); } else 
      //if op == TokenType::MultiplyEquals { structure.value = calculate(&TokenType::Multiply, &leftValue, &rightValue); } else 
      //if op == TokenType::DivideEquals   { structure.value = calculate(&TokenType::Divide,   &leftValue, &rightValue); }
    }
  }

  // update value
  fn replaceStructureByName(&mut self, value: &mut Vec<Token>, length: &mut usize, index: usize) {
    fn setNone(value: &mut Vec<Token>, index: usize) 
    { // error -> skip
      value[index].setData    (None);
      value[index].setDataType(None);
    }

    if let Some(structureName) = value[index].getData()  // todo: use getStructureByName()
    {
      if let Some(structureLink) = self.getStructureByName(&structureName) 
      {
        let structure: RwLockReadGuard<'_, Structure> = structureLink.read().unwrap();
        if index+1 < *length && 
           value[index+1].getDataType().unwrap_or_default() == TokenType::SquareBracketBegin 
        { // array
          let arrayIndex: Option<usize> = 
            value[index+1]
              .tokens
              .as_mut()
              .and_then(|tokens| self.expression(tokens).getData()?.parse::<usize>().ok());
          value.remove(index+1);
          *length -= 1;

          if let Some(idx) = arrayIndex 
          { // n-line structure
            // todo: fix memoryCell nesting
            let result = self.expression(&mut structure.lines[idx].write().unwrap().tokens); // todo: type
            value[index].setData    ( result.getData().clone() );
            value[index].setDataType( result.getDataType().clone() );
          } else 
          {
            setNone(value, index);
          }
        } else 
        { 
          if structure.lines.len() == 1 
          { // first-line structure
            let result = self.expression(&mut structure.lines[0].write().unwrap().tokens); // todo: type
            value[index].setData    ( result.getData().clone() );
            value[index].setDataType( result.getDataType().clone() );
          } else 
          if structure.lines.len() > 1 
          { // array structure
            let mut linesResult = Vec::new(); // todo: type
            for line in &structure.lines 
            {
              linesResult.push(
                self.expression(&mut line.write().unwrap().tokens)
              );
            }
            value[index] = Token::newNesting( Some(linesResult) );
            value[index].setDataType( Some(TokenType::Array) );
          } else
          { // empty structure
            value[index].setData    ( Some(structureName) );
            value[index].setDataType( Some(TokenType::Array) );
          }
        }
      } else 
      {
        setNone(value, index);
      }
    } else 
    {
      setNone(value, index);
    }
  }

  // get link expression
  fn linkExpression(&mut self, link: &mut Vec<&str>) -> String
  {
    println!("linkExpression {:?}",link);
    match link[0].parse::<usize>() 
    { // check type
      Ok(lineNumber) => 
      { // line num
        println!("structure.line [{}]", lineNumber);
        if let Some(line) = self.lines.get(lineNumber) 
        { // get line of num and return result
          let line = line.read().unwrap(); // todo: type
          println!("  line {:?}:[{}]", line.tokens,line.tokens.len());
          let mut lineTokens: Vec<Token> = line.tokens.clone();
          drop(line);

          let lineResult: String = self.expression(&mut lineTokens).getData().unwrap_or_default();
          println!("  link.len [{}] lineResult [{}]",link.len(),lineResult);
          if link.len() == 1 
          { // read end
            return lineResult;
          } else 
          { // read next
            if let Some(structureLink) = self.getStructureByName(&lineResult) 
            {
              let mut structure: RwLockWriteGuard<'_, Structure> = structureLink.write().unwrap();
              println!("  structure.name [{}]", structure.name);
              link.remove(0);
              return structure.linkExpression(link);
            }
          }
        }
      }
      Err(_) => 
      { // name
        if let Some(structureLink) = self.getStructureByName(link[0]) 
        {
          let mut structure: RwLockWriteGuard<'_, Structure> = structureLink.write().unwrap();
          println!("structure.name [{}]", structure.name);

          link.remove(0);
          if link.len() != 0 
          { // has nesting
            return structure.linkExpression(link);
          } else 
          if structure.lines.len() == 1 
          { // single value
            println!("  basic word!");
            if let Some(line) = structure.lines.get(0) 
            { // get first line and return result
              let line = line.read().unwrap(); // todo: type
              println!("  line {:?}:[{}] {}", line.tokens,line.tokens.len(),line.index);
              let mut lineTokens: Vec<Token> = line.tokens.clone();
              drop(line);
              return structure.expression(&mut lineTokens).getData().unwrap_or_default();
            }
          } else 
          { // basic word
            return structure.name.clone();
          }
        }
      }
    }
    String::new()
  }

  // format quote
  fn formatQuote(&mut self, quote: String) -> String 
  {
    let mut result:           String    = String::new();
    let mut expressionBuffer: String    = String::new();
    let mut expressionRead:   bool      = false;
    let     chars:            Vec<char> = quote.chars().collect();

    let mut i:      usize = 0;
    let     length: usize = chars.len();
    let mut c:      char;

    while i < length 
    {
      c = chars[i];
      if c == '{' 
      {
        expressionRead = true;
      } else
      if c == '}' 
      {
        expressionRead = false;
        expressionBuffer += "\n";
        unsafe
        { 
          let     expressionLineLink:     &Arc<RwLock< Line >>      = &readTokens( expressionBuffer.as_bytes().to_vec(), false )[0];
          let     expressionLine:         RwLockReadGuard<'_, Line> = expressionLineLink.read().unwrap();
          let mut expressionBufferTokens: Vec<Token>                = expressionLine.tokens.clone();
          if let Some(expressionData) = self.expression(&mut expressionBufferTokens).getData() 
          {
            result += &expressionData;
          }
        }
        expressionBuffer = String::new();
      } else 
      {
        if expressionRead 
        {
          expressionBuffer.push(c);
        } else 
        {
          result.push(c);
        }
      }
      i += 1;
    }
    result
  }

  // get structure parameters
  pub fn getStructureParameters(&self, value: &mut Vec<Token>) -> Vec<Token> 
  {
    let mut result = Vec::new();

    let mut expressionBuffer = Vec::new(); // buffer of current expression
    for (l, token) in value.iter().enumerate() 
    { // read tokens
      if token.getDataType().unwrap_or_default() == TokenType::Comma || l+1 == value.len() 
      { // comma or line end
        if token.getDataType().unwrap_or_default() != TokenType::Comma 
        {
          expressionBuffer.push( token.clone() );
        }

        if expressionBuffer.len() == 3 
        {
          if let Some(expressionData) = expressionBuffer[2].getData() 
          {
            expressionBuffer[0].setDataType( Some(getStructureResultType(expressionData)) );
          }
        }
        result.push( expressionBuffer[0].clone() );

        expressionBuffer.clear();
      } else 
      { // push new expression token
        expressionBuffer.push( token.clone() );
      }
    }

    result
  }

  // get function parameters
  fn getFunctionParameters(&mut self, value: &mut Vec<Token>, i: usize) -> Vec<Token> 
  {
    let mut result: Vec<Token> = Vec::new();

    if let Some(tokens) = value.get(i+1).map(|v| &v.tokens) 
    {
      if let Some(tokens) = tokens
      { // get bracket tokens
        let mut expressionBuffer: Vec<Token> = Vec::new(); // buffer of current expression
        for (l, token) in tokens.iter().enumerate() 
        { // read tokens
          if token.getDataType().unwrap_or_default() == TokenType::Comma || l+1 == tokens.len() 
          { // comma or line end
            if token.getDataType().unwrap_or_default() != TokenType::Comma 
            {
              expressionBuffer.push( token.clone() );
            }
            result.push( self.expression(&mut expressionBuffer) );
            expressionBuffer.clear();
          } else 
          { // push new expression token
            expressionBuffer.push( token.clone() );
          }
        }
        value.remove(i+1); // remove bracket
      }
    }

    result
  }

  // expression
  pub fn expression(&mut self, value: &mut Vec<Token>) -> Token 
  {
    let mut valueLength: usize = value.len();

    // 1 number
    if valueLength == 1 
    {
      if value[0].getDataType().unwrap_or_default() != TokenType::CircleBracketBegin 
      {
        if value[0].getDataType().unwrap_or_default() == TokenType::Link 
        { 
          let data = value[0].getData().unwrap_or_default(); // todo: type
          value[0].setData(Some(
            self.linkExpression(&mut data.split('.').collect())
          ));
        } else
        if value[0].getDataType().unwrap_or_default() == TokenType::Word 
        { 
          let data = value[0].getData().unwrap_or_default(); // todo: type
          value[0].setData(Some(
            self.linkExpression(&mut vec![&data])
          ));
        } else 
        if value[0].getDataType().unwrap_or_default() == TokenType::FormattedRawString ||
           value[0].getDataType().unwrap_or_default() == TokenType::FormattedString    ||
           value[0].getDataType().unwrap_or_default() == TokenType::FormattedChar 
        { 
          if let Some(valueData) = value[0].getData() 
          { 
            let newData: String = self.formatQuote(valueData);
            value[0].setData( Some(newData) );
          }
        }
        return value[0].clone();
      }
    }

    //
    let mut i: usize = 0;
    let mut token: Token;
    // MemoryCell & function
    while i < valueLength 
    {
      if value[i].getDataType().unwrap_or_default() == TokenType::Word 
      { // function call
        if i+1 < valueLength && value[i+1].getDataType().unwrap_or_default() == TokenType::CircleBracketBegin 
        {
          // todo: uint float ufloat ...
          // todo: replace if -> match
          if let Some(functionName) = value[i].getData() 
          { // begin of list of functions
            if functionName == "int" 
            { // get expressions
              let expressions: Vec<Token> = self.getFunctionParameters(value, i);
              if let Some(expressionData) = expressions.get(0).and_then(|expr| expr.getData())
              { // functional
                value[i].setData    ( Some(expressionData) );
                value[i].setDataType( Some(TokenType::Int) );
              } else 
              { // error -> skip
                value[i].setData    ( None );
                value[i].setDataType( None );
              }
              valueLength -= 1;
              continue;
            } else 
            if functionName == "char" 
            { // get expressions
              let expressions: Vec<Token> = self.getFunctionParameters(value, i);
              if let Some(expressionData) = expressions.get(0).and_then(|expr| expr.getData())
              { // functional
                value[i] = expressions[0].clone();

                let newData: String = (expressionData.parse::<u8>().unwrap() as char).to_string();
                value[i].setData( Some(newData) );

                value[i].setDataType( Some(TokenType::Char) );
              } else 
              { // error -> skip
                value[i].setData    ( None );
                value[i].setDataType( None );
              }
              valueLength -= 1;
              continue;
            } else 
            if functionName == "str" 
            { // get expressions
              let expressions: Vec<Token> = self.getFunctionParameters(value, i);
              if let Some(expressionData) = expressions.get(0).and_then(|expr| expr.getData())
              { // functional
                value[i].setData    ( Some(expressionData) );
                value[i].setDataType( Some(TokenType::String ) );
              } else
              { // error -> skip
                value[i].setData    ( None );
                value[i].setDataType( None );
              }
              valueLength -= 1;
              continue;
            } else 
            if functionName == "type" 
            { // get expressions
              let expressions: Vec<Token> = self.getFunctionParameters(value, i);
              if expressions.len() > 0
              { // functional
                value[i].setData    ( Some(expressions[0].getDataType().unwrap_or_default().to_string()) );
                value[i].setDataType( Some(TokenType::String) );
              } else 
              { // error -> skip
                value[i].setData    ( None );
                value[i].setDataType( None );
              }
              valueLength -= 1;
              continue;
            } else
            if functionName == "input" 
            { // get expressions
              let expressions: Vec<Token> = self.getFunctionParameters(value, i);

              // functional
              if expressions.len() > 0 
              {
                if let Some(expressionData) = expressions[0].getData() 
                {
                  print!("{}",expressionData);
                  io::stdout().flush().unwrap(); // forced withdrawal of old
                } // else -> skip
              }   // else -> skip

              value[i].setData( None );

              if let Some(mut valueData) = value[i].getData() {
                io::stdin().read_line(&mut valueData).expect("Input error"); // todo: delete error
                value[i].setData( 
                  Some( valueData.trim_end().to_string() )
                );
              } else 
              { // error -> skip
                value[i].setData( 
                  None
                );
              }

              value[i].setDataType( Some(TokenType::String) );

              valueLength -= 1;
              continue;
            } else 
            if functionName == "exec"
            { // execute
              let expressions: Vec<Token> = self.getFunctionParameters(value, i);
              if expressions.len() > 0 
              { // functional
                let expressionValue: Option< String > = expressions[0].getData();
                if let Some(expressionValue) = expressionValue 
                { // functional
                  let mut parts: SplitWhitespace<'_> = expressionValue.split_whitespace();

                  let command: &str      = parts.next().expect("No command found in expression"); // todo: no errors
                  let    args: Vec<&str> = parts.collect();

                  let output: Output = 
                    Command::new(command)
                      .args(&args)
                      .output()
                      .expect("Failed to execute process"); // todo: no errors
                  let outputString = String::from_utf8_lossy(&output.stdout).to_string();
                  if !outputString.is_empty() 
                  { // result
                    value[i].setData    ( Some(outputString) );
                    value[i].setDataType( Some(TokenType::String) );
                  }
                } else 
                { // error -> skip
                  value[i].setData    ( None );
                  value[i].setDataType( None );
                }
                valueLength -= 1;
                continue;
              }
            }
            if functionName == "randUInt" 
            { // get expressions
              let expressions: Vec<Token> = self.getFunctionParameters(value, i);
              if expressions.len() > 1 
              { // functional
                let min: usize = 
                  if let Some(expressionData) = expressions[0].getData() {
                    expressionData.parse::<usize>().unwrap_or_default()
                  } else 
                  {
                    0
                  };
                let max: usize = 
                  if let Some(expressionData) = expressions[1].getData() {
                    expressionData.parse::<usize>().unwrap_or_default()
                  } else 
                  {
                    0
                  };

                let randomNumber: usize = 
                  if min < max 
                  {
                    rand::thread_rng().gen_range(min..=max)
                  } else 
                  {
                    0
                  };

                value[i].setData    ( Some(randomNumber.to_string()) );
                value[i].setDataType( Some(TokenType::UInt) );
              } else 
              { // error -> skip
                value[i].setData    ( None );
                value[i].setDataType( None );
              }
              valueLength -= 1;
              continue;
            } else 
            if functionName == "len" 
            { // get expressions
              let expressions: Vec<Token> = self.getFunctionParameters(value, i);
              if expressions.len() > 0
              { // functional
                if expressions[0].getDataType().unwrap_or_default() == TokenType::Array 
                {
                  if let Some(memoryCellName) = expressions[0].getData() 
                  {
                    if let Some(memoryCellLink) = self.getStructureByName(&memoryCellName) 
                    {
                      let memoryCell: RwLockReadGuard<'_, Structure> = memoryCellLink.read().unwrap();
                      /*
                      value[i].setData(Some(
                        memoryCell.value.tokens
                          .clone().unwrap_or_default()
                          .len().to_string()
                      ));
                      */
                    } else 
                    { // error -> skip
                      value[i].setData( Some(String::from("0")) );
                    }
                  } else 
                  { // error -> skip
                    value[i].setData( Some(String::from("0")) );
                  }
                } else 
                { // get basic cell len
                  // todo:
                  value[i].setData  ( Some(String::from("0")) );
                }
                value[i].setDataType( Some(TokenType::UInt) );
              } else 
              { // error -> skip
                value[i].setData    ( None );
                value[i].setDataType( None );
              }
              valueLength -= 1;
              continue;
            } else
            { // custom structure result
              let mut lineBuffer = Line::newEmpty();
              lineBuffer.tokens  = value.clone();
              unsafe{ self.procedureCall( Arc::new(RwLock::new(lineBuffer)) ); }

              if let Some(structureName) = value[0].getData() 
              { // get structure name
                if let Some(structureLink) = self.getStructureByName(&structureName) 
                { // get structure
                  let structure = structureLink.read().unwrap();
                  if let Some(result) = &structure.result 
                  { // functional
                    value[i].setData    ( result.getData() );
                    value[i].setDataType( result.getDataType().clone() );
                  } else 
                  { // error -> skip
                    value[i].setData    ( None );
                    value[i].setDataType( None );
                  }
                } else 
                { // error -> skip
                  value[i].setData    ( None );
                  value[i].setDataType( None );
                }
              }
              // end of list of functions
            }
            value.remove(i+1);
            valueLength -= 1;
            continue;
          }
        } else 
        { // array & basic cell
          self.replaceStructureByName(value, &mut valueLength, i);
        }
      }
      if valueLength == 1 
      {
        break;
      }
      i += 1;
    }
    // bracket
    i = 0;
    while i < valueLength 
    {
      token = value[i].clone();
      if token.getDataType().unwrap_or_default() == TokenType::CircleBracketBegin 
      {
        value[i] = 
          if let Some(mut tokenTokens) = token.tokens.clone() 
          {
            self.expression(&mut tokenTokens)
          } else
          {
            Token::newEmpty(None)
          }
      }
      i += 1;
    }
    // =
    i = 0;
    while i < valueLength 
    {
      if valueLength == 1 
      {
        break;
      }
      if i == 0 {
        i += 1;
        continue;
      }

      token = value[i].clone();
      if i+1 < valueLength && 
        (token.getDataType().unwrap_or_default() == TokenType::Inclusion           || 
         token.getDataType().unwrap_or_default() == TokenType::Joint               || 
         token.getDataType().unwrap_or_default() == TokenType::Equals              || 
         token.getDataType().unwrap_or_default() == TokenType::NotEquals           ||
         token.getDataType().unwrap_or_default() == TokenType::GreaterThan         || 
         token.getDataType().unwrap_or_default() == TokenType::LessThan            ||
         token.getDataType().unwrap_or_default() == TokenType::GreaterThanOrEquals || 
         token.getDataType().unwrap_or_default() == TokenType::LessThanOrEquals) {
        value[i-1] = calculate(&token.getDataType().unwrap_or_default(), &value[i-1], &value[i+1]);
        
        value.remove(i); // remove op
        value.remove(i); // remove right value
        valueLength -= 2;
        continue;
      }

      i += 1;
    }
    // * and /
    i = 0;
    while i < valueLength 
    {
      if valueLength == 1 
      {
        break;
      }
      if i == 0 
      {
        i += 1;
        continue;
      }

      token = value[i].clone();
      if i+1 < valueLength && 
        (token.getDataType().unwrap_or_default() == TokenType::Multiply || 
         token.getDataType().unwrap_or_default() == TokenType::Divide) 
      {
        value[i-1] = calculate(&token.getDataType().unwrap_or_default(), &value[i-1], &value[i+1]);

        value.remove(i); // remove op
        value.remove(i); // remove right value
        valueLength -= 2;
        continue;
      }

      i += 1;
    }
    // + and -
    i = 0;
    while i < valueLength 
    {
      if valueLength == 1 
      {
        break;
      }
      if i == 0 
      {
        i += 1;
        continue;
      }

      token = value[i].clone();
      // + and -
      if i+1 < valueLength && 
        (token.getDataType().unwrap_or_default() == TokenType::Plus || 
         token.getDataType().unwrap_or_default() == TokenType::Minus) 
      {
        value[i-1] = calculate(&token.getDataType().unwrap_or_default(), &value[i-1], &value[i+1]);

        value.remove(i); // remove op
        value.remove(i); // remove right value
        valueLength -= 2;
        continue;
      } else
      // value -value2
      if token.getDataType().unwrap_or_default() == TokenType::Int || 
         token.getDataType().unwrap_or_default() == TokenType::Float 
      {
        value[i-1] = calculate(&TokenType::Plus, &value[i-1], &value[i]);

        value.remove(i); // remove UInt
        valueLength -= 1;
        continue;
      }

      i += 1;
    }
    //
    if value.len() > 0 
    {
      value[0].clone()
    } else {
      Token::newEmpty(None)
    }
  }

  /* search procedure call
     e:
       procedureCall(parameters)
  */
  pub unsafe fn procedureCall(&mut self, lineLink: Arc<RwLock<Line>>) -> bool 
  {
    let line: RwLockReadGuard<'_, Line> = lineLink.read().unwrap();
    if line.tokens.get(0).and_then(|t| t.getDataType()).unwrap_or_default() == TokenType::Word
    { // add structure call
      if line.tokens.get(1).and_then(|t| t.getDataType()).unwrap_or_default() == TokenType::CircleBracketBegin
      { // check lower first char
        let token: &Token = &line.tokens[0];
        if let Some(tokenData) = token.getData() 
        {
          if tokenData.starts_with(|c: char| c.is_lowercase()) 
          {
            // todo: multi-param
            // basic structures
            let mut result = true;
            match tokenData.as_str() {
              "go" =>
              { // go block up
                if let Some(parentLink) = &line.parent 
                {
                  if let Some(structureParent) = &self.parent 
                  {
                    // todo: check expressionValue
                    //searchCondition(parentLink.clone(), structureParent.clone());
                  }
                }
              }
              "ex" =>
              { // exit block up
                println!("ex"); 
              }
              "println" =>
              { // println
                // expression tokens
                let expressionValue: Option< Vec<Token> > = line.tokens[1].tokens.clone();
                if let Some(mut expressionValue) = expressionValue 
                { // expression value
                  let expressionValue: Option< String > = self.expression(&mut expressionValue).getData();
                  if let Some(expressionValue) = expressionValue 
                  { // functional
                    println!("{}",formatPrint(&expressionValue));
                  } else 
                  { // else -> skip
                    println!();
                  }
                } else 
                { // else -> skip
                  println!();
                }
                io::stdout().flush().unwrap(); // forced withdrawal of old
              }
              "print" =>
              { // print
                // expression tokens
                let expressionValue: Option< Vec<Token> > = line.tokens[1].tokens.clone();
                if let Some(mut expressionValue) = expressionValue 
                { // expression value
                  let expressionValue: Option< String > = self.expression(&mut expressionValue).getData();
                  if let Some(expressionValue) = expressionValue 
                  { // functional
                    print!("{}",formatPrint(&expressionValue));
                  } else 
                  { // else -> skip
                    print!("");
                  }
                } else 
                { // else -> skip
                  print!("");
                }
                io::stdout().flush().unwrap(); // forced withdrawal of old
              }
              "exec" =>
              { // execute
                // run function
                self.expression(&mut line.tokens.clone()).getData();
                // else -> skip
              }
              "clear" =>
              { // clear
                Command::new("clear")
                  .status()
                  .expect("Failed to clear console"); // todo: move to functions ?
              }
              "sleep" =>
              { // sleep
                // expression tokens
                let expressionTokens: Option< Vec<Token> > = line.tokens[1].tokens.clone();
                if let Some(mut expressionTokens) = expressionTokens 
                { // expression value
                  let expressionValue: Option< String > = self.expression(&mut expressionTokens).getData();
                  if let Some(expressionValue) = expressionValue 
                  { // functional
                    let valueNumber: u64 = expressionValue.parse::<u64>().unwrap_or_default(); // todo: depends on Value.rs
                    if valueNumber > 0 
                    {
                      sleep( Duration::from_millis(valueNumber) );
                    }
                  } // else -> skip
                }   // else -> skip
              }
              "exit" =>
              { // exit
                _exitCode = true;
              }
              _ =>
              { // custom structure
                result = false;
              }
            }
            // custom structures
            if !result 
            {
              if let Some(calledStructureLink) = self.getStructureByName(&tokenData) 
              {
                //
                // set parameters
                // todo: merge with up structure
                {
                  let expressionValue: Option< Vec<Token> > = line.tokens[1].tokens.clone();
                  let parameters:      Option< Vec<Token> > = 
                    if let Some(mut expressionValue) = expressionValue
                    {
//                      println!("2: expressionValue {:?}",expressionValue);
                      Some( self.getStructureParameters(&mut expressionValue) )
                    } else 
                    {
                      None
                    };
                  if let Some(parameters) = parameters 
                  {
                    let calledStructure: RwLockWriteGuard<'_, Structure> = calledStructureLink.write().unwrap();
                    for (l, parameter) in parameters.iter().enumerate() 
                    {
//                      println!("2: call parameter [{}]:[{}]",parameter,parameter.getDataType().unwrap_or_default().to_string());
                      if let Some(calledStructureStructures) = &calledStructure.structures
                      {
//                        println!("  calledStructure.structures.len [{}]",calledStructureStructures.len());
                        let parameterResult = self.expression(&mut vec![parameter.clone()]); // todo: type
//                        println!("  parameterResult [{}]",parameterResult);
                        if let Some(parameterStructure) = calledStructureStructures.get(l) 
                        {
                          let mut parameterStructure = parameterStructure.write().unwrap(); // todo: type
//                          println!("    parameterStructure [{}]",parameterStructure.name);
                          // add new structure
                          parameterStructure.lines = 
                            vec![
                              Arc::new(
                              RwLock::new(
                                Line {
                                  tokens: vec![parameterResult],
                                  indent: 0,
                                  index:  0,
                                  lines:  None,
                                  parent: None
                                }
                              ))
                            ];
                        }
                      }
                      //
                    }
                  }
                }
                // run
                let mut lineIndexBuffer:   usize = 0;
                let mut linesLengthBuffer: usize = // todo: remove this, use calledStructure.lines.len() in readLines()
                  {
                    let calledStructure: RwLockReadGuard<'_, Structure> = calledStructureLink.read().unwrap();
                    calledStructure.lines.len()
                  };
                readLines(calledStructureLink, &mut lineIndexBuffer, &mut linesLengthBuffer, false);
                return true;
              }
            }
            return result;
          }
          //
        }
        //
      }
    }
    return false;
  }
}
