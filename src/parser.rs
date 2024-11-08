/* /parser
  предоставляет механизмы для парсинга токенов;
  что позволяет запускать получившиеся структуры.
*/

pub mod value;
pub mod uf64;
pub mod structure;

use crate::{
  logger::*,
  _argc, _argv, _debugMode, _exitCode,
  parser::structure::*,
  tokenizer::{token::*, line::*}
};

use std::{
  time::{Instant, Duration},
  sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard},
  ptr::addr_of_mut
};

// проверяет что переданный dataType 
// является математическим оператором
fn isMathOperator(dataType: TokenType) -> bool 
{
  matches!(dataType, 
    TokenType::Equals         | // =
    TokenType::UnaryPlus      | // ++
    TokenType::PlusEquals     | // +=
    TokenType::UnaryMinus     | // --
    TokenType::MinusEquals    | // -=
    TokenType::UnaryMultiply  | // **
    TokenType::MultiplyEquals | // *=
    TokenType::UnaryDivide    | // //
    TokenType::DivideEquals   | // /=
    TokenType::UnaryModulo    | // %%
    TokenType::ModuloEquals   | // %=
    TokenType::UnaryExponent  | // ^^
    TokenType::ExponentEquals   // ^=
  )
}

// эта функция должна искать return для структур
// e:  = value
// это явно показывает, что это не просто валяющееся значение
unsafe fn searchReturn(lineLink: Arc<RwLock<Line>>, structureLink: Arc<RwLock<Structure>>) -> bool 
{
  let mut lineTokens: Vec<Token> = 
  {
    lineLink.read().unwrap() // читаемая линия, на которой мы сейчас находимся
      .tokens.clone()        // токены линии на которой мы сейчас находимся
  };

  if lineTokens[0].getDataType().unwrap_or_default() == TokenType::Equals 
  { // если нашли TokenType::Equals, значит это return, сразу удаляем его,
    // чтобы он нам не мешался потом
    lineTokens.remove(0);

    // редактируемый родитель, поскольку мы собираемся присвоить значение его result
    let mut structure:  RwLockWriteGuard<'_, Structure> = structureLink.write().unwrap();

    // собственно, структура ожидает какой-то тип в результате, 
    // либо это может быть TokenType:None. Но мы просто будем менять data

    // используем expression, чтобы получить результат выражения
    let newResultData: Option<String> = structure.expression(&mut lineTokens).getData();
    if let Some(structureResult) = &mut structure.result 
    { // присваиваем новую data результату
      structureResult.setData( newResultData );
    }

    // всё успешно, это был результат
    return true;
  }
  // это был не результат, идём дальше
  return false;
}
// эта функция ищет структуры
// это может быть либо:
// - вложенная структура (типо array/vector/list, как удобно)
// - линейное выражение (типо a = 10)
// - условный блок (типо if/elif/else)
unsafe fn searchStructure(lineLink: Arc<RwLock<Line>>, parentLink: Arc<RwLock<Structure>>, lineIndex: *mut usize) -> bool 
{
  // todo: line можно вынести, чтобы потом не было .read().unwrap();
  //       для этого надо сразу забрать все нужные значения здесь.
  let line:             RwLockReadGuard<'_, Line> = lineLink.read().unwrap(); // сама линия
  let lineTokens:       &Vec<Token>               = &line.tokens;             // ссылка на токены линии
  let lineTokensLength: usize                     = lineTokens.len();         // размер токенов линии

  let firstTokenType:  TokenType                = lineTokens[0].getDataType().unwrap_or_default(); // тип первого токена в строке
  let lineLines:       Vec< Arc<RwLock<Line>> > = line.lines.clone().unwrap_or(vec![]);            // вложенные линии

  if firstTokenType == TokenType::Word
  { // если мы видим TokenType::Word в начале строки, 
    // это значит, что это либо структура, либо линейная запись
    if lineLines.len() > 0
    { // если в линии есть вложение, то это структура
      if let Some(newStructureName) = lineTokens[0].getData() 
      { // получаем имя структуры
        let mut newStructureResultType: Option<TokenType>    = None; // результат структуры
        let mut parameters:             Option< Vec<Token> > = None; // параметры структуры
        if lineTokensLength > 1 && lineTokens[1].getDataType().unwrap_or_default() == TokenType::CircleBracketBegin 
        { // если токенов > 1 и 1 токен это TokenType::CircleBracketBegin 
          // значит это вариант параметры + возможно результат

          if let Some(mut lineTokens) = lineTokens[1].tokens.clone() 
          { // берём вложенные токены в TokenType::CircleBracketBegin 
            // получаем параметры из этих токенов, давая доступ к родительским структурам
            parameters = Some( 
              parentLink.read().unwrap() // читаем родительскую структуру
                .getStructureParameters(&mut lineTokens) 
            );
          }
          // если > 3 (т.е name () -> result)
          // то значит это результат структуры 
          if lineTokensLength > 3 && 
             lineTokens[2].getDataType().unwrap_or_default() == TokenType::Pointer && 
             lineTokens[3].getDataType().unwrap_or_default() == TokenType::Word 
          { // в таком случае просто читаем тип результата структуры
            if let Some(lineTokenData) = lineTokens[3].getData() 
            {
              newStructureResultType = Some( getStructureResultType(lineTokenData) );
            }
          } // если результата не было, то просто пропускаем
        } else 
        { // в этом случае это вариант только с результатом структуры
          if lineTokensLength > 2 && 
             lineTokens[1].getDataType().unwrap_or_default() == TokenType::Pointer && 
             lineTokens[2].getDataType().unwrap_or_default() == TokenType::Word 
          { // в таком случае просто читаем тип результата структуры
            if let Some(lineTokenData) = lineTokens[2].getData() 
            {
              newStructureResultType = Some( getStructureResultType(lineTokenData) );
            }
          } // если результата не было, то просто пропускаем
        } // если параметров и результата не было, то просто пропускаем
        // создаём новую структуру
        let mut newStructure: Structure = 
          Structure::new(
            newStructureName,
            lineLines,
            Some(parentLink.clone())
          );
        // ставим модификаторы на структуру;
        // параметры структуры, если они были
        if let Some(parameters) = &parameters 
        { 
          for parameter in parameters 
          {
            newStructure.pushStructure(
              Structure::new(
                parameter.getData().unwrap_or_default(),
                vec![], // todo: add option, pls 
                None,
              )
            );
          }
        }
        // результат структуры, если он есть
        newStructure.result = Some( Token::newEmpty(newStructureResultType.clone()) );
        // создаём ссылку на новую структуру
        let newStructure: Arc<RwLock<Structure>> =
          Arc::new(RwLock::new(
            newStructure
          ));
        { // читаем структуру, чтобы найти результаты
          readLines(newStructure.clone(), true);
        }
        // получаем редактируемую структуру родителя
        let mut parent: RwLockWriteGuard<'_, Structure> = parentLink.write().unwrap();
        if let Some(ref mut parentStructures) = parent.structures 
        { // если уже есть структуры в родителе,
          // то просто push делаем
          parentStructures.push(
            newStructure
          );
        } else 
        { // если не было ещё структур в родителе
          // то создаём новый вектор
          parent.structures = 
            Some(vec![
              newStructure
            ]);
        }
        return true;
      }
    } else 
    { // если это не структура, значит это линейная запись
      let mut opType: TokenType = TokenType::None; // готовим место для проверки оператора
      let mut opPos:  usize     = 0;               // это будет место, где находится оператор
      for (i, lineToken) in lineTokens.iter().enumerate()
      { // читаем линию, и ищем чтобы TokenType в opType совпал с математическим
        // после чего выходим отсюда и остаётся позиция найденного оператора в opPos
        opType = lineToken.getDataType().unwrap_or_default().clone();
        if isMathOperator(opType.clone()) 
        {
          opPos = i+1;
          break;
        }
      }
      // позиция оператора не может быть 0, т.к. по 0 у нас TokenType::Word
      // поэтому мы проверяем позицию > 1 и количество токенов в строке > 1
      if lineTokensLength > 1 && opPos > 1
      { // теперь мы точно уверенны, что это линейная запись с математической операцией
        if let Some(structureName) = lineTokens[0].getData() 
        { // получаем имя первого токена, чтобы знать с кем мы работаем

          // это левая часть линейной записи
          let leftValue:  Option< Vec<Token> > = Some( lineTokens[1..opPos-1].to_vec() );
          // это правая (рабочая) запись линейной части
          let rightValue: Option< Vec<Token> > = Some( lineTokens[opPos..(lineTokensLength)].to_vec() );
          // получаем родительскую структуру
          let mut structure: RwLockWriteGuard<'_, Structure> = parentLink.write().unwrap();
          // ищем в родительской структуре, есть ли там похожая на structureName
          if let Some(parentLink) = structure.getStructureByName(&structureName) 
          { // если мы нашли такую, то значит работаем уже с существующей структурой
            structure.structureOp(
              parentLink, 
              opType, 
              leftValue.unwrap_or(vec![]).clone(),
              rightValue.unwrap_or(vec![]).clone()
            );
          } else 
          { // если мы не нашли похожую, то создаём новую и работаем с правой частью выражения
            let tokens: Vec<Token> = 
              vec![ // значение будет состоять из вычисленной правой части выражения
                structure.expression(&mut rightValue.unwrap_or(vec![]).clone()) // один токен
              ];
            // закидываем новую структуру в родительскую структуру
            structure.pushStructure(
              Structure::new(
                structureName,
                vec![ Arc::new(RwLock::new( 
                  Line {
                    tokens: tokens,
                    indent: 0,
                    lines:  None,
                    parent: None
                  }
                )) ],
                None
              )
            );
          }
          return true;
        }
      }
      //
    }
  } else 
  // в том случае, если это не структура и не линейная запись, 
  // мы видим TokenType::Question в начале строки и есть вложения у этой линии, 
  // то это условное вложение
  if firstTokenType == TokenType::Question && lineLines.len() > 0
  { // условное вложение запускает код внутри себя, в том случае если её условное выражение = true;
    // если условное выражение = false, то условное вложение не запускается, 
    // но может продолжить запускать блоки ниже, если такие там есть.
    // в этом моменте мы точно уверены что нашли первое условное вложение
    let mut conditions: Vec< Arc<RwLock<Line>> > = Vec::new();
    let mut saveNewLineIndex: usize = 0;  // сдвиг вниз на сколько условных блоков мы увидели
    { // теперь мы ищем все условные вложения ниже
      let lines: Vec< Arc<RwLock<Line>> > = 
      { 
        parentLink.read().unwrap() // родительская структура
          .lines.clone()           // родительские линии
      };
      let linesLength: usize = lines.len(); // количество линий родительской структуры
      { // смотрим линии внизу
        let mut i: usize = *lineIndex;
        while i < linesLength 
        { // если line index < lines length, то читаем вниз линии,
          // и если там первый токен не имеет TokenType::Question,
          // или количество токенов == 0, то только в этом случае break;
          // это будет означать, что мы нашли все возможные условные блоки.
          let lineBottomLink: Arc<RwLock<Line>> = lines[i].clone(); // ссылка на нижнюю линию
          { // берём нижнюю линию на чтение
            let bottomLine: RwLockReadGuard<'_, Line> = lineBottomLink.read().unwrap();
            // выходим если пустая линия
            if bottomLine.tokens.len() == 0 { break; } else
            // выходим если в начале линии нет TokenType::Question
            if bottomLine.tokens[0].getDataType().unwrap_or_default() != TokenType::Question { break; }
          }
          // если мы не вышли, значит это условный блок;
          // значит мы его добавляем
          conditions.push(lineBottomLink);
          i += 1;
        }
      }
      // в данном месте мы точно уверенны 
      // что conditions.len() > 1 из-за первого блока
      saveNewLineIndex = conditions.len()-1;
    }
    // после нахождения всех возможных условных блоков,
    // начинаем читать их условия и выполнять
    let mut conditionTruth: bool = false; // заранее создаём true/false ячейку
    for conditionLink in &mut conditions 
    { // итак, мы читает ссылки на условия в цикле;
      // после чего мы берём само условие на чтение
      let condition: RwLockReadGuard<'_, Line> = conditionLink.read().unwrap();
      if condition.tokens.len() > 1 
      { // если условие больше чем просто один токен TokenType::Question,
        // то значит там обычное if/elif условие
        { // проверяем верность условия;
          let mut conditionTokens: Vec<Token> = condition.tokens.clone(); // todo: no clone ? fix its please
          // удаляем TokenType::Question токен
          conditionTokens.remove(0);
          // и проверяем
          conditionTruth = 
          { // получаем string ответ от expression, true/false
            let expressionResult: Option<String> = 
              parentLink.read().unwrap()                     // для этого берём родительскую линию;
                .expression(&mut conditionTokens).getData(); // и её токены.
            // итоговый boolean результат
            if let Some(expressionResult) = expressionResult { expressionResult == "true" } 
            else                                             { false }
          };
        }
        // если условие верно
        if conditionTruth 
        { // создаём новую временную структуру условного блока
          let structure: Arc<RwLock<Structure>> =
            Arc::new(
            RwLock::new(
              Structure::new(
                String::from("if-elif"),
                condition.lines.clone().unwrap_or(vec![]),
                Some(parentLink.clone())
              )
            ));
          // после создания, читаем эту структуру
          readLines(structure, false);
          break; // end
        }
      } else
      // в случае если в токенах условия просто TokenType::Question,
      // значит это else блок
      if !conditionTruth 
      { // создаём новую временную структуру условного блока
        let structure: Arc<RwLock<Structure>> =
          Arc::new(
          RwLock::new(
            Structure::new(
              String::from("else"),
              condition.lines.clone().unwrap_or(vec![]),
              Some(parentLink.clone())
            )
          ));
        // после создания, читаем эту структуру
        readLines(structure, false);
        break; // end
      }
    }

    // и только после прочтения всех блоков, 
    // мы можем сдвигать указатель ниже
    *lineIndex += saveNewLineIndex;
    return true;
  }
  return false;
}

lazy_static! 
{ // основная структура, в которой вкладываются остальные;
  // в эту структуру будут переданы стартовые параметры
  static ref _main: Arc<RwLock<Structure>> = Arc::new(
    RwLock::new(
      Structure::new(
        String::from("main"),
        Vec::new(),
        None
      )
    )
  );
}

// это основная функция для парсинга строк;
// она разделена на подготовительную часть,
// и часть запуска readLine()
pub unsafe fn parseLines(tokenizerLinesLinks: Vec< Arc<RwLock<Line>> >) -> ()
{ // начинается подготовка к запуску

  if unsafe{_debugMode} 
  {
    logSeparator("Preparation");
  }

  // присваиваем в главную структуру argc & argv
  // todo: переписать закоментированные части, чтобы оно работало
  {
    let mut main: RwLockWriteGuard<'_, Structure> = _main.write().unwrap();
    main.lines = tokenizerLinesLinks.clone();
    // argc
    /*
    main.pushStructure(
      MemoryCell::new(
        String::from("argc"),
        MemoryCellMode::LockedFinal,
        TokenType::UInt,
        Token::newNesting(
          Some( vec![
            Token::new( Some(TokenType::UInt), Some(_argc.to_string()) )
          ] )
        )
      )
    );
    */
    // argv
    let mut argv: Vec<Token> = Vec::new();
    for a in &_argv 
    {
      argv.push(
        Token::new( Some(TokenType::String), Some(String::from(a)) )
      );
    }
    /*
    main.pushStructure(
      Structure::new(
        String::from("argv"),
        //MemoryCellMode::LockedFinal,
        //TokenType::Array,
        Some(argv),
        None
      )
    );
    */
  }

  if unsafe{_debugMode} 
  {
    log("ok",&format!("argc [{}]",_argc));
    if _argc > 0 
    {
      log("ok",&format!("argv {:?}",_argv));
    }
  }

  // подготовка закончена, читаем линии
  let startTime: Instant = Instant::now(); // получаем текущее время для замера debug
  if unsafe{_debugMode} 
  {
    logSeparator("Interpretation");
  }
  // передаём ссылку на структуру, указатель текущей линии и количество линий
  readLines(_main.clone(), false);
  // далее идут замеры
  if unsafe{_debugMode} 
  {
    let endTime:  Instant  = Instant::now();    // получаем текущее время
    let duration: Duration = endTime-startTime; // получаем сколько всего прошло
    logSeparator("End");
    log("ok",&format!("Parser duration [{:?}]",duration));
  }
}
// эта функция занимается чтением блоков по ссылке на них;
// также необходимо передать переменную указателя чтения линии,
// передать сколько всего линий вложено.
// todo: исправить переполнение стека
pub unsafe fn readLines(structureLink: Arc<RwLock<Structure>>, structuresRead: bool) -> ()
{ // получаем сколько линий вложено в структуру
  let (lineIndex, linesLength): (*mut usize, usize) = 
  {
    let structure = structureLink.read().unwrap(); // Читаем структуру
    (
      unsafe { &structure.lineIndex as *const usize as *mut usize }, 
      structure.lines.len()
    )
  };

  // выполнение программы происходит до тех пор, 
  // пока не будет всё прочитано, либо 
  // пока не будет вызван _exitCode на true
  while _exitCode == false && *lineIndex < linesLength 
  { // если мы читаем строки, то создаём сразу ссылку на текущую линию
    let lineLink: Arc<RwLock<Line>> = 
    { // получаем её через чтение текущей структуры;
      // берём линию по индексу линии
      structureLink.read().unwrap()
        .lines[*lineIndex].clone()
    };
    // после чего проверяем, если линия пустая на токены, 
    // то не читаем и идём дальше
    if lineLink.read().unwrap()
        .tokens.len() == 0 
    {
      *lineIndex += 1;
      continue;
    }
    // если всё хорошо, то начинаем читать через специальные функции;
    // ищем структуры
    if !searchStructure(lineLink.clone(), structureLink.clone(), lineIndex) 
    { // ищем return
      if !searchReturn(lineLink.clone(), structureLink.clone()) 
      { // ищем выражения
        if !structuresRead 
        { // читаем выражение в режиме изменения структуры
          structureLink.read().unwrap()
            .expression(
              &mut lineLink.read().unwrap()
                .tokens.clone()
            ); 
          // клонируем токены, для сохранения возможности повторного запуска
        }
      }
    }
    // идём дальше
    *lineIndex += 1;
  }
}