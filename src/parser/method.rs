/*
    Method
*/

use crate::logger::*;
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
use std::{borrow::Cow, str::SplitWhitespace};
use std::process::{Command, Output};
use std::thread::sleep;
use std::time::Duration;
use rand::Rng;

// get method result type
pub fn getMethodResultType(word: String) -> TokenType 
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

pub struct Method 
{
  pub           name: String,                        // unique name
                                                     // todo: Option
  pub          lines: Vec< Arc<RwLock<Line>> >,      // nesting lines
                                                     // todo: Option
  pub     parameters: Vec<Token>,                    // parameters
                                                     // todo: Option< Arc<RwLock<Token>> >
  pub         result: Option<Token>,                 // result type
      // if result type = None, => procedure
      // else => function
  pub memoryCellList: Arc<RwLock<MemoryCellList>>,   // todo: option< Arc<RwLock<MemoryCellList>> > ?
  pub        methods:    Vec< Arc<RwLock<Method>> >,
  pub         parent: Option< Arc<RwLock<Method>> >,
}
impl Method 
{
  pub fn new
  (
      name: String,
     lines: Vec< Arc<RwLock<Line>> >,
    parent: Option< Arc<RwLock<Method>> >,
  ) -> Self 
  {
    Method 
    {
                name,
               lines,
          parameters: Vec::new(),
              result: None,
      memoryCellList: Arc::new(RwLock::new(MemoryCellList::new())),
             methods: Vec::new(),
              parent
    }
  }

  // get method by name
  pub fn getMethodByName(&self, name: &str) -> Option<Arc<RwLock<Method>>> 
  {
    for childMethodLink in &self.methods 
    {
      let childMethod = childMethodLink.read().unwrap();
      if name == childMethod.name 
      {
        return Some(childMethodLink.clone());
      }
    }

    // check the parent method if it exists
    if let Some(parentLink) = &self.parent 
    {
      let parentMethod: RwLockReadGuard<'_, Method> = parentLink.read().unwrap();
      parentMethod.getMethodByName(name)
    } else { None }
  }

  // push memoryCell to self memoryCellList
  pub fn pushMemoryCell(&self, mut memoryCell: MemoryCell) -> ()
  {
    // basic
    if memoryCell.valueType != TokenType::Array 
    {
      memoryCell.value = self.memoryCellExpression(&mut memoryCell.value.tokens.clone());
    }
    // array
    let mut memoryCellList: RwLockWriteGuard<'_, MemoryCellList> = self.memoryCellList.write().unwrap();
    memoryCellList.value.push( Arc::new(RwLock::new(memoryCell)) );
  }

  // get memory cell by name
  pub fn getMemoryCellByName(&self, memoryCellName: &str) -> Option<Arc<RwLock<MemoryCell>>> 
  {
    // search in self
    if let Some(memoryCell) = getMemoryCellByName(self.memoryCellList.clone(), memoryCellName) 
    {
      return Some(memoryCell);
    }
    // search in parent
    if let Some(parentLink) = &self.parent 
    {
      let parent: RwLockReadGuard<'_, Method> = parentLink.read().unwrap();
      return parent.getMemoryCellByName(memoryCellName);
    }
    //
    None
  }

  // memory cell op
  pub fn memoryCellOp(&self, memoryCellLink: Arc<RwLock<MemoryCell>>, op: TokenType, opValue: Token) -> ()
  {
    if op != TokenType::Equals         &&
       op != TokenType::PlusEquals     && op != TokenType::MinusEquals &&
       op != TokenType::MultiplyEquals && op != TokenType::DivideEquals 
      { return; }

    // calculate new values
    let rightValue: Token = self.memoryCellExpression(&mut opValue.tokens.clone());
    let mut memoryCell = memoryCellLink.write().unwrap();
    // =
    if op == TokenType::Equals 
    {
      memoryCell.value = rightValue;
    } else 
    { // += -= *= /=
      let leftValue: Token = memoryCell.value.clone();
      if op == TokenType::PlusEquals     { memoryCell.value = calculate(&TokenType::Plus,     &leftValue, &rightValue); } else 
      if op == TokenType::MinusEquals    { memoryCell.value = calculate(&TokenType::Minus,    &leftValue, &rightValue); } else 
      if op == TokenType::MultiplyEquals { memoryCell.value = calculate(&TokenType::Multiply, &leftValue, &rightValue); } else 
      if op == TokenType::DivideEquals   { memoryCell.value = calculate(&TokenType::Divide,   &leftValue, &rightValue); }
    }
  }

  // update value
  fn replaceMemoryCellByName(&self, value: &mut Vec<Token>, length: &mut usize, index: usize) -> ()
  {
    if let Some(memoryCellLink) = self.getMemoryCellByName( value[index].getData() ) 
    {
      let memoryCell = memoryCellLink.read().unwrap();
      if index+1 < *length && *value[index+1].getDataType() == TokenType::SquareBracketBegin 
      {
        let arrayIndex = // todo: rewrite if no UInt type ...
            self
                .memoryCellExpression(&mut value[index+1].tokens)
                .getData().parse::<usize>();

        value.remove(index+1);
        *length -= 1;
        match arrayIndex 
        {
          Ok(idx) => 
          {
            value[index].setData    ( memoryCell.value.tokens[idx].getData().to_string() );
            value[index].setDataType( memoryCell.value.tokens[idx].getDataType().clone() );
          }
          Err(_) => 
          { // parsing errors
            value[index].setData    ( String::new() );
            value[index].setDataType( TokenType::None );
          }
        }
      } else 
      {
        value[index].setData    ( memoryCell.value.getData().to_string() );
        value[index].setDataType( memoryCell.value.getDataType().clone() );
      }
    } else 
    { // error -> skip
      value[index].setData    ( String::new() );
      value[index].setDataType( TokenType::None );
    }
  }

  // format quote
  fn formatQuote(&self, quote: String) -> String 
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
          let expressionLineLink = &readTokens( expressionBuffer.as_bytes().to_vec(), false )[0];
          let expressionLine     = expressionLineLink.read().unwrap();
          let mut expressionBufferTokens: Vec<Token> = expressionLine.tokens.clone();
          result += self.memoryCellExpression(&mut expressionBufferTokens).getData();
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

  // get method parameters
  pub fn getMethodParameters(&self, value: &mut Vec<Token>) -> Vec<Token> 
  {
    let mut result = Vec::new();

    let mut expressionBuffer = Vec::new(); // buffer of current expression
    for (l, token) in value.iter().enumerate() 
    { // read tokens
      if *token.getDataType() == TokenType::Comma || l+1 == value.len() 
      { // comma or line end
        if *token.getDataType() != TokenType::Comma 
        {
          expressionBuffer.push( token.clone() );
        }

        let mut parameterBuffer: Token = Token::new(TokenType::None, expressionBuffer[0].getData().to_string());
        if expressionBuffer.len() == 3 
        {
          parameterBuffer.setDataType( 
            getMethodResultType( expressionBuffer[2].getData().to_string() ) 
          );
        }
        result.push( parameterBuffer );

        expressionBuffer.clear();
      } else 
      { // push new expression token
        expressionBuffer.push( token.clone() );
      }
    }

    result
  }

  // get expression parameters
  fn getExpressionParameters(&self, value: &mut Vec<Token>, i: usize) -> Vec<Token> 
  {
    let mut result = Vec::new();

    if let Some(tokens) = value.get(i+1).map(|v| &v.tokens) 
    { // get bracket tokens
      let mut expressionBuffer = Vec::new(); // buffer of current expression
      for (l, token) in tokens.iter().enumerate() 
      { // read tokens
        if *token.getDataType() == TokenType::Comma || l+1 == tokens.len() 
        { // comma or line end
          if *token.getDataType() != TokenType::Comma 
          {
            expressionBuffer.push( token.clone() );
          }
          result.push( self.memoryCellExpression(&mut expressionBuffer) );
          expressionBuffer.clear();
        } else 
        { // push new expression token
          expressionBuffer.push( token.clone() );
        }
      }
      value.remove(i+1); // remove bracket
    }

    result
  }

  // expression
  pub fn memoryCellExpression(&self, value: &mut Vec<Token>) -> Token 
  {
    let mut valueLength: usize = value.len();

    // 1 number
    if valueLength == 1 
    {
      if *value[0].getDataType() != TokenType::CircleBracketBegin 
      {
        if *value[0].getDataType() == TokenType::Word 
          { self.replaceMemoryCellByName(value, &mut valueLength, 0); } 
        else 
        if *value[0].getDataType() == TokenType::FormattedRawString ||
           *value[0].getDataType() == TokenType::FormattedString    ||
           *value[0].getDataType() == TokenType::FormattedChar 
        { 
          let newData = self.formatQuote(value[0].getData().to_string());
          value[0].setData(newData);
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
        if *value[i].getDataType() == TokenType::Word 
        {
          // function
          if i+1 < valueLength && *value[i+1].getDataType() == TokenType::CircleBracketBegin 
          {
            let functionName: String = value[i].getData().to_string();
            // todo: uint float ufloat ...
            if functionName == "int" 
            {
              // get expressions
              let expressions: Vec<Token> = self.getExpressionParameters(value, i);
              // 
              if expressions.len() > 0 
              {
                value[i]            = expressions[0].clone();
                value[i].setDataType( TokenType::Int );
              } else 
              {
                value[i].setData    ( String::new() );
                value[i].setDataType( TokenType::None );
              }
              valueLength -= 1;
              continue;
            } else 
            if functionName == "char" 
            {
              // get expressions
              let expressions: Vec<Token> = self.getExpressionParameters(value, i);
              // 
              if expressions.len() > 0 
              {
                value[i] = expressions[0].clone();

                let newData = (value[i].getData().parse::<u8>().unwrap() as char).to_string();
                value[i].setData(newData);

                value[i].setDataType( TokenType::Char );
              } else 
              {
                value[i].setData    ( String::new() );
                value[i].setDataType( TokenType::None );
              }
              valueLength -= 1;
              continue;
            } else 
            if functionName == "str" 
            {
              // get expressions
              let expressions: Vec<Token> = self.getExpressionParameters(value, i);
              // 
              if expressions.len() > 0 
              {
                value[i]            = expressions[0].clone();
                value[i].setDataType( TokenType::String );
              } else
              {
                value[i].setData    ( String::new() );
                value[i].setDataType( TokenType::None );
              }
              valueLength -= 1;
              continue;
            } else 
            if functionName == "type" 
            {
              // get expressions
              let expressions: Vec<Token> = self.getExpressionParameters(value, i);
              // 
              if expressions.len() > 0 
              {
                value[i].setData    ( expressions[0].getDataType().to_string() );
                value[i].setDataType( TokenType::String );
              } else 
              {
                value[i].setData    ( String::new() );
                value[i].setDataType( TokenType::None );
              }
              valueLength -= 1;
              continue;
            } else
            if functionName == "input" 
            {
              // get expressions
              let expressions: Vec<Token> = self.getExpressionParameters(value, i);
              //
              if expressions.len() > 0 
              {
                print!("{}",expressions[0].getData());
                io::stdout().flush().unwrap(); // forced withdrawal of old
              }

              value[i].setData( String::new() );
              io::stdin().read_line(&mut value[i].getData().to_string()).expect("Input error"); // todo: delete error

              let trimmedData = value[i].getData().trim_end().to_string();
              value[i].setData( trimmedData );

              value[i].setDataType( TokenType::String );

              valueLength -= 1;
              continue;
            } else 
            if functionName == "randUInt" 
            {
              // get expressions
              let expressions: Vec<Token> = self.getExpressionParameters(value, i);
              // 
              if expressions.len() > 1 
              {
                let mut rng = rand::thread_rng();
                let min: usize = expressions[0].getData().parse::<usize>().unwrap_or(0);
                let max: usize = expressions[1].getData().parse::<usize>().unwrap_or(0);
                let randomNumber: usize = rng.gen_range(min..=max);

                value[i].setData    ( randomNumber.to_string() );
                value[i].setDataType( TokenType::UInt );
              } else 
              {
                value[i].setData    ( String::new() );
                value[i].setDataType( TokenType::None );
              }

              valueLength -= 1;
              continue;
            } else 
            {
              let mut lineBuffer = Line::newEmpty();
              lineBuffer.tokens  = value.clone();
              unsafe{ self.methodCall( Arc::new(RwLock::new(lineBuffer)) ); }

              // todo: rewrite
              if let Some(methodLink) = self.getMethodByName(value[0].getData()) 
              {
                let method = methodLink.read().unwrap();
                if let Some(result) = &method.result 
                {
                  value[i].setData    ( result.getData().to_string() );
                  value[i].setDataType( result.getDataType().clone() );
                } else 
                {
                  value[i].setData    ( String::new() );
                  value[i].setDataType( TokenType::None );
                }
              } else 
              {
                value[i].setData    ( String::new() );
                value[i].setDataType( TokenType::None );
              }

              value.remove(i+1);
              valueLength -= 1;
              continue;
            }
          // array & basic cell
          } else 
          {
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
    while i < valueLength 
    {
      token = value[i].clone();
      if *token.getDataType() == TokenType::CircleBracketBegin 
      {
        value[i] = self.memoryCellExpression(&mut token.tokens.clone());
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
        (*token.getDataType() == TokenType::Inclusion           || 
         *token.getDataType() == TokenType::Joint               || 
         *token.getDataType() == TokenType::Equals              || 
         *token.getDataType() == TokenType::NotEquals           ||
         *token.getDataType() == TokenType::GreaterThan         || 
         *token.getDataType() == TokenType::LessThan            ||
         *token.getDataType() == TokenType::GreaterThanOrEquals || 
         *token.getDataType() == TokenType::LessThanOrEquals) {
        value[i-1] = calculate(token.getDataType(), &value[i-1], &value[i+1]);
        
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
      if i+1 < valueLength && (*token.getDataType() == TokenType::Multiply || *token.getDataType() == TokenType::Divide) 
      {
        value[i-1] = calculate(token.getDataType(), &value[i-1], &value[i+1]);

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
      if i+1 < valueLength && (*token.getDataType() == TokenType::Plus || *token.getDataType() == TokenType::Minus) 
      {
        value[i-1] = calculate(token.getDataType(), &value[i-1], &value[i+1]);

        value.remove(i); // remove op
        value.remove(i); // remove right value
        valueLength -= 2;
        continue;
      } else
      // value -value2
      if *token.getDataType() == TokenType::Int || *token.getDataType() == TokenType::Float 
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
      Token::newEmpty(TokenType::None)
    }
  }

  /* search methods call
     e:
       methodCall(parameters)
  */
  pub unsafe fn methodCall(&self, lineLink: Arc<RwLock<Line>>) -> bool 
  {
    let line: RwLockReadGuard<'_, Line> = lineLink.read().unwrap();
    if *line.tokens[0].getDataType() == TokenType::Word 
    {
      // add method call
      if line.tokens.len() > 1 && *line.tokens[1].getDataType() == TokenType::CircleBracketBegin 
      {
        // check lower first char
        let token: &Token = &line.tokens[0];
        if token.getData().starts_with(|c: char| c.is_lowercase()) 
        {
          let mut expressionValue: Vec<Token> = line.tokens[1].tokens.clone();
          // todo: multi-param
          // basic methods
          let mut result = true;
          {
            // go block up
            if token.getData() == "go" 
            {
              if let Some(parentLink) = &line.parent 
              {
                if let Some(methodParent) = &self.parent 
                {
                    // todo: check expressionValue
                    searchCondition(parentLink.clone(), methodParent.clone());
                }
              }
            } else
            // exit block up
            if token.getData() == "ex" 
            {
              println!("ex");
            } else
            // println
            if token.getData() == "println" 
            {
              println!("{}",formatPrint(
                &self.memoryCellExpression(&mut expressionValue).getData().to_string()
              ));
              io::stdout().flush().unwrap(); // forced withdrawal of old
            } else 
            // print
            if token.getData() == "print" {
              print!("{}",formatPrint(
                &self.memoryCellExpression(&mut expressionValue).getData().to_string()
              ));
              io::stdout().flush().unwrap(); // forced withdrawal of old
            } else 
            // sleep
            if token.getData() == "sleep" {
              let value = &self.memoryCellExpression(&mut expressionValue).getData().to_string();
              let valueNumber = value.parse::<u64>().unwrap_or(0);
              sleep(Duration::from_millis(valueNumber));
            } else 
            // exec
            if token.getData() == "exec" {
              let expression: String              = self.memoryCellExpression(&mut expressionValue).getData().to_string();
              let mut  parts: SplitWhitespace<'_> = expression.split_whitespace();

              let command: &str      = parts.next().expect("No command found in expression"); // todo: 
              let    args: Vec<&str> = parts.collect();

              let output: Output = 
                Command::new(command)
                  .args(&args)
                  .output()
                  .expect("Failed to execute process"); // todo: 
              let outputString: Cow<'_, str> = String::from_utf8_lossy(&output.stdout);
              if !outputString.is_empty() 
              {
                print!("{}", outputString);
              }
            } else 
            // exit
            if token.getData() == "exit" 
            {
              _exitCode = true;
            // custom method
            } else 
            {
              result = false;
            }
          }
          // custom methods
          if !result 
          {
            if let Some(calledMethodLink) = self.getMethodByName(token.getData()) 
            {
              //
              let mut   lineIndexBuffer: usize = 0;
              let mut linesLengthBuffer: usize = 
                {
                  let calledMethod = calledMethodLink.read().unwrap(); // todo: type
                  calledMethod.lines.len()
                };
              // set parameters
              // todo: merge with up method
              {
                let mut parameters: Option< Vec<Token> > = Some( self.getMethodParameters(&mut expressionValue) );
                if let Some(parameters) = parameters 
                {
                  let calledMethod = calledMethodLink.read().unwrap(); // todo: type
                  let mut memoryCellList = calledMethod.memoryCellList.write().unwrap(); // todo: type
                  for (l, parameter) in parameters.iter().enumerate() 
                  {
                    let mut memoryCell = memoryCellList.value[l].write().unwrap(); // todo: type
                    memoryCell.value.setData( parameter.getData().to_string() );
                  }
                }
              }
              // run
              readLines(calledMethodLink, &mut lineIndexBuffer, &mut linesLengthBuffer);
              return true;
            }
          }
          return result;
        }
        //
      }
    }
    return false;
  }
}
