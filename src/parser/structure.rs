/* /parser/structure
  структура, которая представляет свободную ячейку данных в памяти;
  имеет свои настройки, место хранения.
*/

use crate::{
  logger::*,
  _exitCode,
  tokenizer::{line::*, token::*, readTokens},
  parser::{readLines, value::*, uf64::*},
};

use std::{
  io::{self, Write},
  process::{Command, Output},
  str::SplitWhitespace,
  sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard},
  thread::sleep,
  time::Duration,
};

use rand::Rng;

// вычисляет по математической операции значение и тип нового токена из двух
pub fn calculate(op: &TokenType, leftToken: &Token, rightToken: &Token) -> Token 
{
  // получаем значение левой части выражения
  let leftTokenDataType: TokenType = leftToken.getDataType().unwrap_or_default();
  let leftValue: Value = getValue(leftToken.getData().unwrap_or_default(), &leftTokenDataType);
  // получаем значение правой части выражения
  let rightTokenDataType: TokenType = rightToken.getDataType().unwrap_or_default();
  let rightValue: Value = getValue(rightToken.getData().unwrap_or_default(), &rightTokenDataType);
  // получаем значение выражения, а также предварительный тип
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
  // после того как значение было получено,
  // смотрим какой точно тип выдать новому токену
  if resultType != TokenType::Bool 
  {
    if leftTokenDataType == TokenType::String || rightTokenDataType == TokenType::String 
    { 
      resultType = TokenType::String;
    } else
    if (matches!(leftTokenDataType, TokenType::Int | TokenType::UInt) && 
        rightTokenDataType == TokenType::Char) 
    { // 
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
// зависимость для calculate
// считает значение левой и правой части выражения
fn getValue(tokenData: String, tokenDataType: &TokenType) -> Value {
  return
    match tokenDataType {
      TokenType::Int    => 
      { 
        tokenData.parse::<i64>()
          .map(Value::Int)
          .unwrap_or(Value::Int(0)) 
      },
      TokenType::UInt   => 
      { 
        tokenData.parse::<u64>()
          .map(Value::UInt)
          .unwrap_or(Value::UInt(0)) 
      },
      TokenType::Float  => 
      { 
        tokenData.parse::<f64>()
          .map(Value::Float)
          .unwrap_or(Value::Float(0.0)) 
      },
      TokenType::UFloat => 
      { 
        tokenData.parse::<f64>()
          .map(uf64::from)
          .map(Value::UFloat)
          .unwrap_or(Value::UFloat(uf64::from(0.0))) 
      },
      TokenType::Char   => 
      { 
        tokenData.parse::<char>()
          .map(|x| Value::Char(x))
          .unwrap_or(Value::Char('\0')) 
      },
      TokenType::String => 
      { 
        tokenData.parse::<String>()
          .map(|x| Value::String(x))
          .unwrap_or(Value::String("".to_string())) 
      },
      TokenType::Bool   => 
      { 
        if tokenData == "1" { Value::UInt(1) } 
        else                { Value::UInt(0) } 
      },
      _ => Value::UInt(0),
    };
}

// get structure result type
// todo: вообще лучше бы это было в самом Token,
//       поскольку там есть перевод уже TokenType -> String
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

  // ищет структуру по имени и возвращает либо None, либо ссылку на неё
  pub fn getStructureByName(&self, name: &str) -> Option< Arc<RwLock<Structure>> > 
  {
    if let Some(someStructures) = &self.structures 
    {
      for childStructureLink in someStructures 
      {
        if name == childStructureLink.read().unwrap().name 
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

  // добавляет новую вложенную структуру в текущую структуру
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
  // todo: описание
  fn setStructureNesting(&self, structureNesting: &Vec<Token>, structureLines: &Vec< Arc<RwLock<Line>> >, newTokens: Vec<Token>) -> () 
  {
//    println!("structureNesting [{}]",structureNesting.len());
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
        nestingLine.write().unwrap()
          .tokens = newTokens;
      }

    }
  }

  // выполняет операцию со структурой,
  // для этого требует левую и правую часть выражения,
  // кроме того, требует передачи родительской структуры,
  // чтобы было видно возможные объявления в ней
  pub fn structureOp(&mut self, structureLink: Arc<RwLock<Structure>>, op: TokenType, leftValue: Vec<Token>, rightValue: Vec<Token>) -> ()
  {
    match op {
      TokenType::Equals | TokenType::PlusEquals | TokenType::MinusEquals | 
      TokenType::MultiplyEquals | TokenType::DivideEquals => {},
      _ => {return},
    }

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
    let mut structure: RwLockWriteGuard<'_, Structure> = structureLink.write().unwrap();
    // =
    if op == TokenType::Equals 
    {
//      println!("  Equals, leftValue {:?}",leftValue);
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

  // вроде как возвращает результат за место 
  // struct, array struct, array с одним вложением
  // todo: более подробное описание
  fn replaceStructureByName(&mut self, value: &mut Vec<Token>, length: &mut usize, index: usize) -> ()
  {
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
            let result: Token = self.expression(
              &mut structure.lines[idx]
                .write().unwrap()
                .tokens
            );
            value[index].setData    ( result.getData().clone() );
            value[index].setDataType( result.getDataType().clone() );
          } else 
          {
            setNone(value, index);
          }
        } else 
        { 
          if structure.lines.len() == 1 
          { // структура с одним вложением
            let result: Token = self.expression(
              &mut structure.lines[0]
                .write().unwrap()
                .tokens
            );
            value[index].setData    ( result.getData().clone() );
            value[index].setDataType( result.getDataType().clone() );
          } else 
          if structure.lines.len() > 1 
          { // это структура с вложением
            let mut linesResult: Vec<Token> = Vec::new();
            for line in &structure.lines 
            {
              linesResult.push(
                // в данном случае они дублируются чтобы использовать повторно
                // todo: можно лучше?
                self.expression(
                  &mut line.write().unwrap()
                    .tokens.clone()
                )
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

  /* Получает значение из ссылки на структуру;
     Ссылка на структуру может состоять как из struct name, так и просто из цифр.
  */
  fn linkExpression(&mut self, currentStructureLink: Option< Arc<RwLock<Structure>>  >, link: &mut Vec<String>, parameters: Option< Vec<Token> >) -> Token
  {
    match link[0].parse::<usize>() 
    { // проверяем тип
      Ok(lineNumber) => 
      { // если мы нашли цифру в ссылке, значит это номер на линию в структуре;
        // номер ссылкается только на пространство currentStructureLink
        link.remove(0);

        if let Some(ref currentStructureLock) = currentStructureLink 
        { // это структура, которая была передана предыдущем уровнем ссылки;
          // только в ней мы можем найти нужную линию
          let currentStructure: RwLockReadGuard<'_, Structure> = currentStructureLock.read().unwrap(); // todo: это можно вынести в временный блок
          if let Some(line) = currentStructure.lines.get(lineNumber)                                   //       для получения линии и выхода из read().unwrap()
          { // тогда просто берём такую линию по её номеру
            let mut lineTokens: Vec<Token> = 
              {
                line.read().unwrap()
                  .tokens.clone()
              };
            drop(line);

            if lineTokens.len() > 0 
            { // если в линии нет хотя бы 1 токена, значит нам просто нечего вычислять;
              if link.len() != 0 
              { // если дальше есть продолжение ссылки
                link.insert(0, lineTokens[0].getData().unwrap_or_default());

                // то мы сначала проверяем что такая структура есть во внутреннем пространстве
                if let Some(childStructureLink) = currentStructure.getStructureByName( 
                  &lineTokens[0].getData().unwrap_or_default() 
                )
                {
                  drop(currentStructure);
                  let mut currentStructure: RwLockWriteGuard<'_, Structure> = currentStructureLock.write().unwrap();
                  return currentStructure.linkExpression(None, link, parameters);
                }
                // а если такой ссылки там не было, то значит она в self
                drop(currentStructure);
                return self.linkExpression(currentStructureLink, link, parameters);
              } else 
              if let Some(parameters) = parameters 
              { // если это был просто запуск метода, то запускаем его
                drop(currentStructure);

                let mut currentStructure: RwLockWriteGuard<'_, Structure> = currentStructureLock.write().unwrap();
                let mut parametersToken: Token = Token::newNesting( Some(Vec::new()) ); // todo: add parameters
                parametersToken.setDataType( Some(TokenType::CircleBracketBegin) );

                let mut expressionTokens: Vec<Token> = vec![
                  Token::new( Some(TokenType::Word), lineTokens[0].getData() ),
                  parametersToken
                ];

                return currentStructure.expression( &mut expressionTokens );
              } else 
              { // если дальше нет продолжения ссылки
                if lineTokens[0].getDataType().unwrap_or_default() == TokenType::Word 
                { // если это слово, то это либо ссылка т.к. там много значений в ней;
                  // либо это структура с одиночным вложением и мы можем его забрать сейчас.

                  if let Some(childStructureLink) = currentStructure.getStructureByName( 
                    &lineTokens[0].getData().unwrap_or_default() 
                  )
                  { // пробуем проверить что там 1 линия вложена в структуре;
                    // после чего сможем посчитать её значение.
                    let childStructure: RwLockReadGuard<'_, Structure> = childStructureLink.read().unwrap();
                    if childStructure.lines.len() == 1 
                    {
                      if let Some(line) = childStructure.lines.get(0) 
                      { // по сути это просто 0 линия через expression
                        let mut lineTokens: Vec<Token> = 
                          {
                            line.read().unwrap()
                              .tokens.clone()
                          };
                        drop(line);
                        drop(childStructure);
                        return self.expression(&mut lineTokens);
                      }
                    }
                  }
                  // если ничего не получилось, значит оставляем ссылку
                  return Token::new( Some(TokenType::Link), lineTokens[0].getData() );
                } else 
                { // если это не слово, то смотрим на результат expression
                  return self.expression(&mut lineTokens);
                }
              }
            } else 
            {
              return Token::newEmpty( Some(TokenType::None) );
            }
          }
          //
        }
      }
      Err(_) => 
      { // если мы не нашли цифры в ссылке, значит это просто struct name;
        // они работают в пространстве первого self, но могут и внутри себя
        let mut structureLink: Option< Arc<RwLock<Structure>> > = 
          if let Some(currentStructureLink) = currentStructureLink
          { // если есть в локальном окружении
            let structure: RwLockReadGuard<'_, Structure> = currentStructureLink.read().unwrap();
            let hasLines: bool = 
            {
              let childStructureLink = structure.getStructureByName(&link[0]);
              if let Some(childStructureLink) = childStructureLink 
              {
                let childStructure: RwLockReadGuard<'_, Structure> = childStructureLink.read().unwrap();
                if childStructure.lines.len() != 0 { true } else { false }
              } else { false }
            };

            if hasLines
            {
              structure.getStructureByName(&link[0])
            } else 
            {
              self.getStructureByName(&link[0]) 
            }
          } else 
          { // если нет в локальном окружении, 
            // то просто берём из self
            self.getStructureByName(&link[0]) 
          };
        //
        link.remove(0);
        if let Some(ref structureLink) = structureLink
        {
          let structure: RwLockReadGuard<'_, Structure> = structureLink.read().unwrap();
        }
        if let Some(structureLink) = structureLink
        { // это структура которую мы нашли по имени в self пространстве
          if link.len() != 0
          { // если ссылка ещё не закончилась, значит продолжаем её чтение
            return self.linkExpression(Some(structureLink), link, parameters);
          } else 
          { // если это конец ссылки, то берём структуру и работаем с ней
            let structure: RwLockReadGuard<'_, Structure> = structureLink.read().unwrap();
            if structure.lines.len() == 1 
            { // если это просто одиночное значение, то просто выдаём его
              if let Some(line) = structure.lines.get(0) 
              { // по сути это просто 0 линия через expression
                let mut lineTokens: Vec<Token> = 
                  {
                    line.read().unwrap()
                      .tokens.clone()
                  };
                drop(line);
                drop(structure);
                return self.expression(&mut lineTokens);
              }
            } else
            if let Some(parameters) = parameters 
            { // если это был просто запуск метода, то запускаем его
              let mut parametersToken: Token = Token::newNesting( Some(Vec::new()) ); // todo: add parameters
              parametersToken.setDataType( Some(TokenType::CircleBracketBegin) );

              let mut expressionTokens: Vec<Token> = vec![
                Token::new( Some(TokenType::Word), Some(structure.name.clone()) ),
                parametersToken
              ];

              if let Some(structureParent) = structure.parent.clone()
              {
                let mut parent: RwLockWriteGuard<'_, Structure> = structureParent.write().unwrap();
                drop(structure);
                return parent.expression( &mut expressionTokens );
              }

              return Token::newEmpty( Some(TokenType::None) );
            } else 
            { // если это просто ссылка, то оставляем её
              return Token::new( Some(TokenType::Link), Some(structure.name.clone()) );
            }
            //
          }
          //
        }
      }
    }
    // если всё было плохо, то просто используем пустой результат
    Token::newEmpty( Some(TokenType::None) )
  }

  /* Принимает formatQuote типы и получает возможное значение обычной строки;
     В основном всё сводится к получению токенов в {} через Token::readTokens(),
     после чего результат проходит через expression и мы получаем обычную строку на выходе.
  */
  fn formatQuote(&mut self, tokenData: String) -> String 
  {
    let mut result:           String    = String::new(); // это строка которая будет получена в конце
    let mut expressionBuffer: String    = String::new(); // буфер для выражения между {}
    let mut expressionRead:   bool      = false;         // флаг чтения в буфер выражения

    let chars:       Vec<char> = tokenData.chars().collect(); // всех символы в строке
    let charsLength: usize     = chars.len();                 // количество всех символов в строке

    let mut i:      usize = 0; // указатель на текущий символ
    let mut c:      char;      // текущий символ

    while i < charsLength 
    { // читаем символы
      c = chars[i];
      if c == '{' 
      { // начинаем чтение выражения
        expressionRead = true;
      } else
      if c == '}' 
      { // заканчиваем чтение выражения
        expressionRead = false;
        expressionBuffer += "\n"; // это нужно чтобы успешно завершить чтение линии Tokenizer::readTokens()

        let expressionLineLink: &Arc<RwLock< Line >> = 
          unsafe{ // получаем результат выражения в виде ссылки на буферную линию
            &readTokens(expressionBuffer.as_bytes().to_vec(), false)[0]
          };

        // получаем линию на чтение;
        // получаем все токены линии
        let mut expressionBufferTokens: Vec<Token> = 
          {
            expressionLineLink.read().unwrap()
              .tokens.clone()
          };
        // отправляем все токены линии как выражение
        if let Some(expressionData) = self.expression(&mut expressionBufferTokens).getData() 
        { // записываем результат посчитанный между {}
          result += &expressionData;
        }
        // обнуляем буфер, вдруг далее ещё есть выражения между {}
        expressionBuffer = String::new();
      } else 
      { // запись символов кроме {}
        if expressionRead 
        { // если флаг чтения активен, то записываем символы выражения
          expressionBuffer.push(c);
        } else 
        { // если флаг чтения не активен, то это просто символы
          result.push(c);
        }
      }
      // продолжаем чтение символов строки
      i += 1;
    }
    // отдаём новую строку
    result
  }

  /* Получает параметры структуры вычисляя их значения;
     todo: требует пересмотра
  */
  pub fn getStructureParameters(&self, value: &mut Vec<Token>) -> Vec<Token> 
  {
    let mut result: Vec<Token> = Vec::new();

    let mut expressionBuffer: Vec<Token> = Vec::new(); // buffer of current expression
    for (l, token) in value.iter().enumerate() 
    { // read tokens
      if token.getDataType().unwrap_or_default() == TokenType::Comma || l+1 == value.len() 
      { // comma or line end
        if token.getDataType().unwrap_or_default() != TokenType::Comma 
        {
          expressionBuffer.push( token.clone() );
        }

        // todo: зачем это?
        /*
        if expressionBuffer.len() == 3 
        {
          if let Some(expressionData) = expressionBuffer[2].getData() 
          {
            expressionBuffer[0].setDataType( Some(getStructureResultType(expressionData)) );
          }
        }
        */
        result.push( expressionBuffer[0].clone() );

        expressionBuffer.clear();
      } else 
      { // push new expression token
        expressionBuffer.push( token.clone() );
      }
    }

    result
  }

  /* Получает параметры при вызове структуры в качестве метода;
     т.е. получает переданные значения через expression
  */
  fn getCallParameters(&mut self, value: &mut Vec<Token>, i: usize) -> Vec<Token> 
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
        //value.remove(i+1); // remove bracket
      }
    }

    result
  }

  /* Основная функция, которая получает результат выражения состоящего из токенов;
     Сначала она проверяет что это single токен, но если нет, 
     то в цикле перебирает возможные варианты
  */
  pub fn expression(&mut self, value: &mut Vec<Token>) -> Token 
  {
    let mut valueLength: usize = value.len(); // получаем количество токенов в выражении

    // 1 токен
    'isSingleToken: 
    { // если это будет не одиночный токен, 
      // то просто выйдем отсюда
      // todo: возможно стоит сразу проверять что тут не Figure, Square, Circle скобки
      if valueLength == 1 
      { // если это выражение с 1 токеном, то;
        match value[0].getDataType().unwrap_or_default()
        { // проверяем возможные варианты;
          TokenType::Link =>
          { // если это TokenType::Link, то;
            let data: String = value[0].getData().unwrap_or_default();                // token data
            let mut link: Vec<String> = data.split('.')
                                          .map(|s| s.to_string())
                                          .collect();
            let linkResult: Token  = self.linkExpression(None, &mut link, None);      // получаем результат от data
            let linkType:   TokenType = linkResult.getDataType().unwrap_or_default(); // предполагаем изменение dataType
            if linkType == TokenType::Word 
            { // если это TokenType::Word то теперь это будет TokenType::Link
              value[0].setDataType( Some(TokenType::Link) );
            } else 
            { // если это другие типы, то просто ставим новый dataType
              value[0].setDataType( linkResult.getDataType() );
            }
            value[0].setData( linkResult.getData() ); // ставим новый data
          }
          TokenType::Word =>
          { // если это TokenType::Word, то;
            let data:       String = value[0].getData().unwrap_or_default();      // token data
            let linkResult: Token  = self.linkExpression(None, &mut vec![data], None); // получаем результат от data
            value[0].setDataType( linkResult.getDataType() );                     // ставим новый dataType
            value[0].setData(     linkResult.getData() );                         // ставим новый data
          }
          TokenType::FormattedRawString | TokenType::FormattedString | TokenType::FormattedChar =>
          { // если это форматные варианты Char, String, RawString;
            if let Some(valueData) = value[0].getData() 
            {  // получаем data этого токена и сразу вычисляем его значение
              value[0].setData( Some(self.formatQuote(valueData)) );
            }
          }
          _ => { break 'isSingleToken; } // выходим т.к. все варианты не прошли
        }
        return value[0].clone(); // возвращаем результат в виде одного токена
      }
    }

    // если это выражение не из одного токена,
    // то следует проверять каждый токен в цикле и
    // производить соответствующие операции
    let mut i: usize = 0; // указатель на текущий токен

    while i < valueLength 
    { // проверяем на использование методов,
      // на использование ссылок на структуру,
      // на использование простого выражения в скобках
      if value[i].getDataType().unwrap_or_default() == TokenType::Word 
      { // это либо метод, либо просто слово-структура
        if i+1 < valueLength && value[i+1].getDataType().unwrap_or_default() == TokenType::CircleBracketBegin 
        { // запускает метод
          self.functionCall(value, &mut valueLength, i);
        } else 
        { // вычисляем значение для struct, array struct, array struct с одним вложением
          self.replaceStructureByName(value, &mut valueLength, i);
        }
      } else
      if value[i].getDataType().unwrap_or_default() == TokenType::Link 
      { // это ссылка на структуру
        let expressions: Vec<Token> = self.getCallParameters(value, i);
        // todo: здесь надо написать вариант в котором ссылку вызвали с параметрами
        if expressions.len() > 0 
        { // если имеются параметры
          //let data: String = value[0].getData().unwrap_or_default();
          //value[0].setDataType( Some(TokenType::String) );
          //value[0].setData(Some(
          //  self.linkExpression(&mut data.split('.').collect(), None)
          //));
        } else 
        { // без параметров
          let data: String = value[i].getData().unwrap_or_default();
          let mut link: Vec<String> = data.split('.')
                                        .map(|s| s.to_string())
                                        .collect();
          let linkResult: Token = 
            if i+1 < valueLength && value[i+1].getDataType().unwrap_or_default() == TokenType::CircleBracketBegin 
            { // если это запуск процедуры
              self.linkExpression(None, &mut link, Some(vec![]))
            } else 
            { // если это обычная ссылка
              self.linkExpression(None, &mut link, None)
            };
          value[i].setDataType( linkResult.getDataType() );
          value[i].setData(     linkResult.getData() );
        }
      } else
      if value[i].getDataType().unwrap_or_default() == TokenType::Minus 
      { // это выражение в круглых скобках, но перед ними отрицание -
        if i+1 < valueLength && value[i+1].getDataType().unwrap_or_default() == TokenType::CircleBracketBegin 
        { // считаем выражение внутри скобок
          value[i] = 
            if let Some(mut tokenTokens) = value[i+1].tokens.clone() 
            { // если получилось то оставляем его
              self.expression(&mut tokenTokens)
            } else
            { // если не получилось, то просто None
              Token::newEmpty(None)
            };
          // удаляем скобки
          value.remove(i+1); // remove UInt
          valueLength -= 1;
          // меняем отрицание
          let tokenData: String = value[i].getData().unwrap_or_default();
          if tokenData.starts_with(|c: char| c == '-') 
          { // если это было отрицательное выражение,
            // то делаем его положительным
            value[i].setData( 
              Some( tokenData.chars().skip(1).collect() ) 
            );
          } else 
          { // если это не было отрицательным выражением,
            // то делаем его отрицательным
            value[i].setData( 
              Some( format!("-{}", tokenData) )
            );
          }
        }
      } else
      if value[i].getDataType().unwrap_or_default() == TokenType::CircleBracketBegin 
      { // это просто выражение в круглых скобках
        value[i] = 
          if let Some(mut tokenTokens) = value[i].tokens.clone() 
          { // если получилось то оставляем его
            self.expression(&mut tokenTokens)
          } else
          { // если не получилось, то просто None
            Token::newEmpty(None)
          }
      }
      i += 1;
    }

    // далее идут варианты математических и логических операций

    // проверка на логические операции
    self.expressionOp(value, &mut valueLength, 
      &[TokenType::Inclusion, TokenType::Joint, TokenType::Equals, 
        TokenType::NotEquals, TokenType::GreaterThan, TokenType::LessThan,
        TokenType::GreaterThanOrEquals, TokenType::LessThanOrEquals]
    );
    
    // проверка * и /
    self.expressionOp(value, &mut valueLength, &[TokenType::Multiply, TokenType::Divide]);
    
    // проверка + и -
    self.expressionOp(value, &mut valueLength, &[TokenType::Plus, TokenType::Minus]);

    // конец чтения выражения
    if value.len() > 0 
    { // в том случае, если мы имеем всё ещё значение,
      // значит просто вернём 0 элемент, чтобы избавиться от него
      value[0].clone()
    } else 
    { // а если всё пусто, ну значит пусто
      Token::newEmpty(None)
    }
  }

  /* Получает значение операции по левому и правому выражению;
     Это зависимость для expression.
     Кроме того, может обрабатываеть отрицание при использовании TokenType::Minus
  */
  fn expressionOp(&mut self, value: &mut Vec<Token>, valueLength: &mut usize, operations: &[TokenType]) 
  {
    let mut i: usize = 0;
    let mut token: Token;
    let mut tokenType: TokenType;

    while i < *valueLength 
    { // проверка на логические операции
      if *valueLength == 1 { break; }
      if i == 0 { i += 1; continue; }

      token = value[i].clone();
      tokenType = token.getDataType().unwrap_or_default();
      if i+1 < *valueLength && matches!(tokenType, ref operations) 
      {
        value[i-1] = calculate(&tokenType, &value[i-1], &value[i+1]);
        
        value.remove(i); // remove op
        value.remove(i); // remove right value
        *valueLength -= 2;
        continue;
      }

      // value -value2
      if tokenType == TokenType::Minus && matches!(tokenType, TokenType::Int | TokenType::Float) 
      {
        value[i-1] = calculate(&TokenType::Plus, &value[i-1], &value[i]);

        value.remove(i); // remove UInt
        *valueLength -= 1;
        continue;
      }

      i += 1;
    }
  }

  /* Запускает функцию;
     Функция - это такая структура, которая возвращает значение.

     Но кроме того, запускает не стандартные методы; 
     В нестандартных методах могут быть процедуры, которые не вернут результат.
  */
  pub fn functionCall(&mut self, value: &mut Vec<Token>, valueLength: &mut usize, i: usize) -> ()
  {
    // todo: uint float ufloat ...
    if let Some(structureName) = value[i].getData() // todo: проверка на нижний регистр
    { // 
      /*
      if functionName == "int" 
      { // get expressions
        let expressions: Vec<Token> = self.getCallParameters(value, i);
        if let Some(expressionData) = expressions.get(0).and_then(|expr| expr.getData())
        { // functional
          value[i].setData    ( Some(expressionData) );
          value[i].setDataType( Some(TokenType::Int) );
        } else 
        { // error -> skip
          value[i].setData    ( None );
          value[i].setDataType( None );
        }
      } else 
      if functionName == "char" 
      { // get expressions
        let expressions: Vec<Token> = self.getCallParameters(value, i);
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
      } else 
      if functionName == "str" 
      { // get expressions
        let expressions: Vec<Token> = self.getCallParameters(value, i);
        if let Some(expressionData) = expressions.get(0).and_then(|expr| expr.getData())
        { // functional
          value[i].setData    ( Some(expressionData) );
          value[i].setDataType( Some(TokenType::String ) );
        } else
        { // error -> skip
          value[i].setData    ( None );
          value[i].setDataType( None );
        }
      } else 
      */
      /*
      if functionName == "input" 
      { // get expressions
        let expressions: Vec<Token> = self.getCallParameters(value, i);

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
      } else 
      if functionName == "exec"
      { // execute
        let expressions: Vec<Token> = self.getCallParameters(value, i);
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
            let outputString: String = String::from_utf8_lossy(&output.stdout).to_string();
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
        }
      }
      if functionName == "randUInt" 
      { // get expressions
        let expressions: Vec<Token> = self.getCallParameters(value, i);
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
      } else 
      if functionName == "len" 
      { // get expressions
        let expressions: Vec<Token> = self.getCallParameters(value, i);
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
      } else
      */
      let expressions: Vec<Token> = self.getCallParameters(value, i);
      // далее идут базовые методы;
      // эти методы ожидают аргументов
      'basicMethods: 
      { // это позволит выйти, если мы ожидаем не стандартные варианты
        if expressions.len() > 0 
        { // далее просто сверяем имя структуры в поисках базовой
          match structureName.as_str() 
          { // проверяем на сходство стандартных функций
            "type" =>
            { // todo: создать resultType() ?
              // для возвращения результата ожидаемого структурой
              value[i].setData    ( Some(expressions[0].getDataType().unwrap_or_default().to_string()) );
              value[i].setDataType( Some(TokenType::String) );
            } 
            _ => { break 'basicMethods; } // выходим, т.к. ожидается нестандартный метод
          }
          // если всё было успешно, то сдвигаем всё до 1 токена;
          // этот токен останется с полученным значением
          *valueLength -= 1;
          value.remove(i+1);
          return;
        }
      }
      // если код не завершился ранее, то далее идут custom методы;
      { // передаём параметры, они также могут быть None
        self.procedureCall( &structureName, Some(expressions) );
        // если всё было успешно, то сдвигаем всё до 1 токена;
        *valueLength -= 1;
        value.remove(i+1);
        // после чего решаем какой результат оставить
        if let Some(structureLink) = self.getStructureByName(&structureName) 
        { // по результату структуры, определяем пустой он или нет
          if let Some(result) = &structureLink.read().unwrap()
                                  .result 
          { // результат не пустой, значит оставляем его
            value[i].setData    ( result.getData() );
            value[i].setDataType( result.getDataType().clone() );
          } else 
          { // если результата структуры не было, 
            // значит это была действительно процедура
            value[i].setData    ( None );
            value[i].setDataType( None );
          }
        }
      }
      // заканчиваем чтение методов
    }
  }

  /* Запускает стандартные процедуры; 
     Процедура - это такая структура, которая не возвращает результат.

     Но кроме того, запускает не стандартные методы; 
     Из нестандартных методов, процедуры могут вернуть результат, в таком случае, их следует считать функциями.
  */
  pub fn procedureCall(&mut self, structureName: &str, expressions: Option< Vec<Token> >) -> bool 
  {
    if structureName.starts_with(|c: char| c.is_lowercase()) 
    { // если название в нижнем регистре - то это точно процедура
      let mut result: bool = true; // ожидается, что он завершится успешно
      match structureName 
      { // проверяем на сходство стандартных функций
        "println" =>
        { // println
          // expression tokens
          if let Some(mut expressionValue) = expressions 
          { // expression value
            let expressionValue: Option<String> = self.expression(&mut expressionValue).getData();
            if let Some(expressionValue) = expressionValue 
            { // functional
              formatPrint(&format!("{}\n",&expressionValue));
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
          if let Some(mut expressionValue) = expressions 
          { // expression value
            let expressionValue: Option< String > = self.expression(&mut expressionValue).getData();
            if let Some(expressionValue) = expressionValue 
            { // functional
              formatPrint(&expressionValue);
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
        /*
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
        */
        /*
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
          unsafe{ _exitCode = true; }
        }
        */
        _ =>
        { // если не было найдено совпадений среди стандартных процедур,
          // значит это нестандартный метод.
          if let Some(calledStructureLink) = self.getStructureByName(&structureName) 
          { // после получения такой нестандартной структуры по имени, 
            // мы смотрим на её параметры
            {
              if let Some(expressions) = expressions 
              {
                let calledStructure: RwLockWriteGuard<'_, Structure> = calledStructureLink.write().unwrap();
                for (l, parameter) in expressions.iter().enumerate() 
                {
                  if let Some(calledStructureStructures) = &calledStructure.structures
                  {
                    let parameterResult: Token = self.expression(&mut vec![parameter.clone()]);
                    if let Some(parameterStructure) = calledStructureStructures.get(l) 
                    {
                      let mut parameterStructure: RwLockWriteGuard<'_, Structure> = parameterStructure.write().unwrap();
                      // add new structure
                      parameterStructure.lines = 
                        vec![
                          Arc::new(
                          RwLock::new(
                            Line {
                              tokens: vec![parameterResult],
                              indent: 0,
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
            // запускаем новую структуру
            let mut lineIndexBuffer: usize = 0;
            unsafe{ readLines(calledStructureLink, &mut lineIndexBuffer, false); }
            return true;
          }
        } // конец custom метода
      }
      // всё успешно, это была стандартная процедура
      return result;
    } // если название структуры не с маленьких букв
    return false;
  }
}
