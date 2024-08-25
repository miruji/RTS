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
      memoryCell.value =
        if let Some(mut memoryCellTokens) = memoryCell.value.tokens.clone()
        {
          self.memoryCellExpression(&mut memoryCellTokens)
        } else 
        { // error
          Token::newEmpty(None)
        }
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
    let rightValue: Token = 
      if let Some(mut opValueTokens) = opValue.tokens.clone() 
      {
        self.memoryCellExpression(&mut opValueTokens)
      } else 
      { // error
        Token::newEmpty(None)
      };
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
    if let Some(memoryCellName) = value[index].getData() 
    {
      if let Some(memoryCellLink) = self.getMemoryCellByName( &memoryCellName ) 
      {
        //
        let memoryCell = memoryCellLink.read().unwrap();
        if index+1 < *length && value[index+1].getDataType().unwrap_or(TokenType::None) == TokenType::SquareBracketBegin 
        {
          let arrayIndex: Result<usize, _> = // todo: rewrite, (if no UInt type) ...
            if let Some(ref mut tokens) = value[index+1].tokens.as_mut()
            {
              if let Some(expressionData) = self.memoryCellExpression(tokens).getData() 
              {
                expressionData.parse::<usize>()
              } else 
              { // error -> skip
                Ok(0)
              }
            } else 
            { // error -> skip
              Ok(0)
            };

          value.remove(index+1);
          *length -= 1;
          match arrayIndex 
          {
            Ok(idx) => 
            {
              if let Some(memoryCellTokens) = &memoryCell.value.tokens 
              {
                value[index].setData    ( memoryCellTokens[idx].getData() );
                value[index].setDataType( memoryCellTokens[idx].getDataType().clone() );
              } else 
              { // error -> skip
                value[index].setData    ( None );
                value[index].setDataType( None );
              }
            }
            Err(_) => 
            { // error -> skip
              value[index].setData    ( None );
              value[index].setDataType( None );
            }
          }
        } else
        {
          value[index].setData    ( memoryCell.value.getData() );
          value[index].setDataType( memoryCell.value.getDataType().clone() );
        }
      } else 
      { // error -> skip
        value[index].setData    ( None );
        value[index].setDataType( None );
      }
    } else 
    { // error -> skip
      value[index].setData    ( None );
      value[index].setDataType( None );
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
          if let Some(expressionData) = self.memoryCellExpression(&mut expressionBufferTokens).getData() 
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

  // get method parameters
  pub fn getMethodParameters(&self, value: &mut Vec<Token>) -> Vec<Token> 
  {
    let mut result = Vec::new();

    let mut expressionBuffer = Vec::new(); // buffer of current expression
    for (l, token) in value.iter().enumerate() 
    { // read tokens
      if token.getDataType().unwrap_or(TokenType::None) == TokenType::Comma || l+1 == value.len() 
      { // comma or line end
        if token.getDataType().unwrap_or(TokenType::None) != TokenType::Comma 
        {
          expressionBuffer.push( token.clone() );
        }

        let mut parameterBuffer: Token = Token::new(None, expressionBuffer[0].getData());
        if expressionBuffer.len() == 3 
        {
          if let Some(expressionData) = expressionBuffer[2].getData() 
          {
            parameterBuffer.setDataType( Some(getMethodResultType(expressionData)) );
          }
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
    let mut result: Vec<Token> = Vec::new();

    if let Some(tokens) = value.get(i+1).map(|v| &v.tokens) 
    {
      if let Some(tokens) = tokens
      { // get bracket tokens
        let mut expressionBuffer: Vec<Token> = Vec::new(); // buffer of current expression
        for (l, token) in tokens.iter().enumerate() 
        { // read tokens
          if token.getDataType().unwrap_or(TokenType::None) == TokenType::Comma || l+1 == tokens.len() 
          { // comma or line end
            if token.getDataType().unwrap_or(TokenType::None) != TokenType::Comma 
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
      if value[0].getDataType().unwrap_or(TokenType::None) != TokenType::CircleBracketBegin 
      {
        if value[0].getDataType().unwrap_or(TokenType::None) == TokenType::Word 
          { self.replaceMemoryCellByName(value, &mut valueLength, 0); } 
        else 
        if value[0].getDataType().unwrap_or(TokenType::None) == TokenType::FormattedRawString ||
           value[0].getDataType().unwrap_or(TokenType::None) == TokenType::FormattedString    ||
           value[0].getDataType().unwrap_or(TokenType::None) == TokenType::FormattedChar 
        { 
          if let Some(mut valueData) = value[0].getData() 
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
        if value[i].getDataType().unwrap_or(TokenType::None) == TokenType::Word 
        {
          // function
          if i+1 < valueLength && value[i+1].getDataType().unwrap_or(TokenType::None) == TokenType::CircleBracketBegin 
          {
            // todo: uint float ufloat ...
            // todo: replace if -> match
            if let Some(functionName) = value[i].getData() 
            { // begin of list of functions
              if functionName == "int" 
              { // get expressions
                let expressions: Vec<Token> = self.getExpressionParameters(value, i);
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
                let expressions: Vec<Token> = self.getExpressionParameters(value, i);
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
                let expressions: Vec<Token> = self.getExpressionParameters(value, i);
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
                let expressions: Vec<Token> = self.getExpressionParameters(value, i);
                if expressions.len() > 0
                { // functional
                  value[i].setData    ( Some(expressions[0].getDataType().unwrap_or(TokenType::None).to_string()) );
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
                let expressions: Vec<Token> = self.getExpressionParameters(value, i);

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
              if functionName == "randUInt" 
              { // get expressions
                let expressions: Vec<Token> = self.getExpressionParameters(value, i);
                if expressions.len() > 1 
                { // functional
                  let min: usize = 
                    if let Some(expressionData) = expressions[0].getData() {
                      expressionData.parse::<usize>().unwrap_or(0)
                    } else 
                    {
                      0
                    };
                  let max: usize = 
                    if let Some(expressionData) = expressions[1].getData() {
                      expressionData.parse::<usize>().unwrap_or(0)
                    } else 
                    {
                      0
                    };

                  let randomNumber: usize = 
                    if min != max 
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
              { // custom method result
                let mut lineBuffer = Line::newEmpty();
                lineBuffer.tokens  = value.clone();
                unsafe{ self.methodCall( Arc::new(RwLock::new(lineBuffer)) ); }

                if let Some(methodName) = value[0].getData() 
                { // get method name
                  if let Some(methodLink) = self.getMethodByName(&methodName) 
                  { // get method
                    let method = methodLink.read().unwrap();
                    if let Some(result) = &method.result 
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
      if token.getDataType().unwrap_or(TokenType::None) == TokenType::CircleBracketBegin 
      {
        value[i] = 
          if let Some(mut tokenTokens) = token.tokens.clone() 
          {
            self.memoryCellExpression(&mut tokenTokens)
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
        (token.getDataType().unwrap_or(TokenType::None) == TokenType::Inclusion           || 
         token.getDataType().unwrap_or(TokenType::None) == TokenType::Joint               || 
         token.getDataType().unwrap_or(TokenType::None) == TokenType::Equals              || 
         token.getDataType().unwrap_or(TokenType::None) == TokenType::NotEquals           ||
         token.getDataType().unwrap_or(TokenType::None) == TokenType::GreaterThan         || 
         token.getDataType().unwrap_or(TokenType::None) == TokenType::LessThan            ||
         token.getDataType().unwrap_or(TokenType::None) == TokenType::GreaterThanOrEquals || 
         token.getDataType().unwrap_or(TokenType::None) == TokenType::LessThanOrEquals) {
        value[i-1] = calculate(&token.getDataType().unwrap_or(TokenType::None), &value[i-1], &value[i+1]);
        
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
        (token.getDataType().unwrap_or(TokenType::None) == TokenType::Multiply || 
         token.getDataType().unwrap_or(TokenType::None) == TokenType::Divide) 
      {
        value[i-1] = calculate(&token.getDataType().unwrap_or(TokenType::None), &value[i-1], &value[i+1]);

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
        (token.getDataType().unwrap_or(TokenType::None) == TokenType::Plus || 
         token.getDataType().unwrap_or(TokenType::None) == TokenType::Minus) 
      {
        value[i-1] = calculate(&token.getDataType().unwrap_or(TokenType::None), &value[i-1], &value[i+1]);

        value.remove(i); // remove op
        value.remove(i); // remove right value
        valueLength -= 2;
        continue;
      } else
      // value -value2
      if token.getDataType().unwrap_or(TokenType::None) == TokenType::Int || 
         token.getDataType().unwrap_or(TokenType::None) == TokenType::Float 
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

  /* search methods call
     e:
       methodCall(parameters)
  */
  pub unsafe fn methodCall(&self, lineLink: Arc<RwLock<Line>>) -> bool 
  {
    let line: RwLockReadGuard<'_, Line> = lineLink.read().unwrap();
    if line.tokens[0].getDataType().unwrap_or(TokenType::None) == TokenType::Word 
    { // add method call
      if line.tokens.len() > 1 && line.tokens[1].getDataType().unwrap_or(TokenType::None) == TokenType::CircleBracketBegin 
      { // check lower first char
        let token: &Token = &line.tokens[0];
        if let Some(tokenData) = token.getData() {
          if tokenData.starts_with(|c: char| c.is_lowercase()) 
          {
            // todo: multi-param
            // basic methods
            let mut result = true;
            {
              if tokenData == "go" 
              { // go block up
                if let Some(parentLink) = &line.parent 
                {
                  if let Some(methodParent) = &self.parent 
                  {
                      // todo: check expressionValue
                      searchCondition(parentLink.clone(), methodParent.clone());
                  }
                }
              } else if tokenData == "ex" 
              { // exit block up
                println!("ex");
              } else if tokenData == "println" 
              { // println
                // expression tokens
                let mut expressionValue: Option< Vec<Token> > = line.tokens[1].tokens.clone();
                if let Some(mut expressionValue) = expressionValue 
                { // expression value
                  let expressionValue: Option< String > = self.memoryCellExpression(&mut expressionValue).getData();
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
              } else if tokenData == "print" 
              { // print
                // expression tokens
                let mut expressionValue: Option< Vec<Token> > = line.tokens[1].tokens.clone();
                if let Some(mut expressionValue) = expressionValue 
                { // expression value
                  let expressionValue: Option< String > = self.memoryCellExpression(&mut expressionValue).getData();
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
              } else if tokenData == "sleep" 
              { // sleep
                // expression tokens
                let mut expressionTokens: Option< Vec<Token> > = line.tokens[1].tokens.clone();
                if let Some(mut expressionTokens) = expressionTokens 
                { // expression value
                  let expressionValue: Option< String > = self.memoryCellExpression(&mut expressionTokens).getData();
                  if let Some(expressionValue) = expressionValue 
                  { // functional
                    let valueNumber: u64 = expressionValue.parse::<u64>().unwrap_or(0);
                    if valueNumber > 0 
                    {
                      sleep( Duration::from_millis(valueNumber) );
                    }
                  } // else -> skip
                }   // else -> skip
              } else if tokenData == "exec" 
              { // exec
                // expression tokens
                let mut expressionTokens: Option< Vec<Token> > = line.tokens[1].tokens.clone();
                if let Some(mut expressionTokens) = expressionTokens 
                { // expression value
                  let expressionValue: Option< String > = self.memoryCellExpression(&mut expressionTokens).getData();
                  if let Some(expressionValue) = expressionValue 
                  { // functional
                    let mut parts: SplitWhitespace<'_> = expressionValue.split_whitespace();

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
                  } // else -> skip
                }   // else -> skip
              } else if tokenData == "exit" 
              { // exit
                _exitCode = true;
              } else 
              { // custom method
                result = false;
              }
            }
            // custom methods
            if !result 
            {
              if let Some(calledMethodLink) = self.getMethodByName(&tokenData) 
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
                  let mut expressionValue: Option< Vec<Token> > = line.tokens[1].tokens.clone();
                  let mut parameters:      Option< Vec<Token> > = 
                    if let Some(mut expressionValue) = expressionValue
                    {
                      Some( self.getMethodParameters(&mut expressionValue) )
                    } else 
                    {
                      None
                    };
                  if let Some(parameters) = parameters 
                  {
                    let calledMethod = calledMethodLink.read().unwrap(); // todo: type
                    let mut memoryCellList = calledMethod.memoryCellList.write().unwrap(); // todo: type
                    for (l, parameter) in parameters.iter().enumerate() 
                    {
                      let mut memoryCell = memoryCellList.value[l].write().unwrap(); // todo: type
                      memoryCell.value.setData( 
                        {
                          if let Some(parameterData) = parameter.getData() 
                          {
                            Some( parameterData.to_string() )
                          } else 
                          { // error
                            None
                          }
                        }
                      );
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
        //
      }
    }
    return false;
  }
}
