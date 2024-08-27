/*
  Memory Cell List
*/

use crate::parser::memoryCell::*;
use crate::parser::value::*;
use crate::parser::uf64::*;
use crate::tokenizer::token::*;

use std::sync::{Arc, RwLock, RwLockReadGuard};

// MemoryCellList
#[derive(Clone)]
pub struct MemoryCellList 
{
  pub value: Vec< Arc<RwLock<MemoryCell>> >,
}
impl MemoryCellList 
{
  pub fn new() -> Self 
  {
    MemoryCellList { value: Vec::new() }
  }
}

// get memory cell by name
pub fn getMemoryCellByName(memoryCellListLink: Arc<RwLock<MemoryCellList>>, name: &str) -> Option<Arc<RwLock<MemoryCell>>> 
{
  let memoryCellList: RwLockReadGuard<'_, MemoryCellList> = memoryCellListLink.read().unwrap();
  for memoryCellLink in &memoryCellList.value 
  {
      let memoryCell = memoryCellLink.read().unwrap();
      if name == memoryCell.name 
      {
          return Some(memoryCellLink.clone());
      }
  }
  None
}

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
