/* /main
  RTS init file

  Несколько моментов о производительности в коде:
  - match быстрее if; matches! быстрее простой проверки if
    при множества значениях на одну проверку;
  - Использование ссылок на данные быстрее их клонирования;
  - Использование Arc+RwLock позволяет нескольким потокам 
    управлять чем-то без клонирование его самого;
  - На RwLock следует вовремя использовать drop(),
    не создавать переменные на них, а также использовать в
    замкнутых временных блоках.
  - Следует избегать флагов mut;
  - Следует указывать типы везде, где это возможно;
  - Объявление данных следует выносить за циклы.
*/
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

#[macro_use]
extern crate lazy_static;

use std::{
  time::{Instant,Duration},
  env,
  io::{self, Read},
  fs::File
};

use crate::logger::*;

mod logger;
mod tokenizer;
mod parser;
mod packageApi;
// other globals
pub static mut _filePath: String = String::new(); // run file path
pub static mut _debugMode: bool = false;          // debug flag
// input & output
pub static mut _argc: usize       = 0;            // arhuments count
pub static mut _argv: Vec<String> = Vec::new();   // arguments vector

pub static mut _exitCode: i32 = 0;      // Значение которое вернёт программа при завершении;
pub static mut _exit:     bool = false; // Завершилась ли программа ?
// version
pub static _version: &str = "231206.0";
// help
fn help() -> ()
{
  // todo: description
  log("ok","version");
  log("ok","<empty>");
  log("ok","help");
  log("ok","drun");
  log("ok","drun <filename>");
  log("ok","drun \"<script>\"");
  log("ok","run");
  log("ok","run <filename>");
  log("ok","run \"<script>\"");
  log("ok","package <empty>");
  log("ok","package help");
  log("ok","package local");
  log("ok","package local-delete");
  logExit(0);
}
// main
#[tokio::main]
async fn main() -> io::Result<()> 
{
  let startTime: Instant = Instant::now();

  //
  use crate::tokenizer::*;
  use crate::parser::*;
  use crate::packageApi::packageApi;

  // args to key-values
  let mut args: (String, Vec<String>) = (String::new(), Vec::new());
  let input:    Vec<String> = env::args().collect();
  if input.len() > 1 
  {
    // first argument is treated as key, others as values
    let command: String      = input[1].clone();
    let values:  Vec<String> = input.iter().skip(2).cloned().collect();
    // store key and values in args vector
    args = (command.clone(), values.clone());
  } else { help() }
  
  // read key
  let mut runFile: bool = false;
  let mut buffer:  Vec<u8> = Vec::new();

  let valuesLength: usize = (args.1).len();

  if !args.0.is_empty() 
  {
    let key: &str = args.0.as_str();
    match key
    {
      "version" => 
      { // get version
        log("ok", &format!("RTS v{}", _version));
        logExit(0);
      }
      "help" => help(),
      "package" =>
      { // package
        packageApi(&args.1,valuesLength).await;
        logExit(0);
      },
      _ if (key == "run" || key == "drun") && valuesLength >= 1 =>
      { // run

        // debug ?
        if key == "drun" { unsafe {_debugMode = true;} }

        // todo: if not file
        // run file
        unsafe
        {
          _argc = valuesLength-1;
          _argv = (args.1)[1..].to_vec();
          _filePath = args.1[0].clone();
        }

        // todo: check filePath file type
        if unsafe{_debugMode} {
          log("ok",&format!("Run [{}]",unsafe{&*_filePath}));
        }
        runFile = true;

        // run script
        /*
        let combinedString: String = values.concat().replace("\\n", "\n"); // todo: \\n ?
        buffer = combinedString.clone().into_bytes();
        // todo: argc & argv

        // todo: check filePath file type
        if unsafe{_debugMode} 
        {
          log("ok",&format!("Run [{}]",combinedString));
        }
        */

        // run package
        // todo: run package
      }
      _ => {
        log("err","Use [rts help] to get help");
        logExit(1)
      }
    }
  }

  if unsafe{_debugMode} 
  {
    logSeparator("Arguments");
    log("ok","Debug mode");
  }

  // run file
  if runFile 
  {
    if unsafe{_debugMode} 
    {
      logSeparator("File");
    }
    // open file
    let mut file: File = match File::open(unsafe{&*_filePath}) 
    {
      Ok(file) => 
      {
        if unsafe{_debugMode} 
        {
          log("ok",&format!("Opening the file [{}] was successful",unsafe{&*_filePath}));
        }
        file
      },
      Err(_) => 
      {
        log("err",&format!("Unable to opening file [{}]",unsafe{&*_filePath}));
        logExit(1)
      }
    };
    // read file into buffer
    match file.read_to_end(&mut buffer) 
    {
      Ok(_) => 
      {
        if unsafe{_debugMode} 
        {
          log("ok",&format!("Reading the file [{}] was successful",unsafe{&*_filePath}));
        }
      }
      Err(_) => 
      {
        log("err",&format!("Unable to read file [{}]",unsafe{&*_filePath}));
        logExit(1)
      }
    }
  }

  // проверяем что в конце был \n, если нет, то добавляем его
  if let Some(&lastByte) = buffer.last() 
  {
    if lastByte != b'\n' 
    {
      buffer.push(b'\n');
    }
  }

  // Начинаем чтение кода
  parseLines( readTokens(buffer, unsafe{_debugMode}) );
  
  if unsafe{_debugMode} 
  { // Замеры всего прошедшего времени работы
    let endTime:  Instant  = Instant::now();
    let duration: Duration = endTime-startTime;
    log("ok",&format!("All duration [{:?}]",duration));
  }
  // ** Для дополнительных тестов можно использовать hyperfine/perf

  // Возвращаем код завершения
  logExit(unsafe{_exitCode});
}
