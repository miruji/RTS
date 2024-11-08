/* /main
  RTS init file
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

pub static mut _exitCode: bool = false; // todo: remove
// version
lazy_static! 
{
  pub static ref _version: String = getVersion(env!("CARGO_PKG_VERSION"));
}
// get cargo version
fn getVersion(version: &str) -> String 
{
  let mut result: String     = String::new();

  let digits:     Vec<&str>  = version.split('.').collect();
  let digitsSize: usize      = digits.len()-1;
  let mut i:      usize      = 0;

  while i < digitsSize 
  {
    let digit = digits[i];
    if digit != "0"   { result.push_str(digit); }
    if i < digitsSize { result.push('.'); }
    i += 1;
  }
  result
}
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
// 
/*
async fn fetchPackage(packageId: &str) -> Result<Value, Error> {
  let url = format!("https://realtime.su/api/packages/{}", packageId);
  let response = reqwest::get(&url).await?;
  let package = response.json::<Value>().await?;
  Ok(package)
}
// install package
async fn packageInstall(values: &Vec<String>) -> () {
  log("ok", &format!("Installing packages {:?}", values));

  for name in values {
    match fetchPackage(name).await {
      Ok(package) => {
        log("ok", &format!("Fetched package for {}: {}", name, package));

        if let Some(pkgName) = package.get("name") {
          log("ok", &format!("Package name: {}", pkgName));
        }
      }
      Err(err) => {
        log("error", &format!("Error fetching package {}: {}", name, err));
      }
    }
  }
}
*/
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
  let mut args: Vec<(String, Vec<String>)> = Vec::new();
  let input:    Vec<String>                = env::args().collect();
  if input.len() > 1 
  {
    // first argument is treated as key, others as values
    let command: String      = input[1].clone();
    let values:  Vec<String> = input.iter().skip(2).cloned().collect();
    // store key and values in args vector
    args.push((command.clone(), values.clone()));
  } else { help() }
  
  // read key
  let mut runFile: bool = false;
  let mut  buffer: Vec<u8> = Vec::new();

  let valuesLength: usize = (args[0].1).len();

  if !args.is_empty() 
  {
    let key: &str = args[0].0.as_str();
    match key
    {
      "version" => 
      { // get version
        log("ok", &format!("RTS v{}", *_version));
        logExit(0);
      }
      "help" => help(),
      "package" =>
      { // package
        packageApi(&args[0].1,valuesLength).await;
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
          _argc = valuesLength;
          _argv = args[0].1.clone();
          _filePath = args[0].1[0].clone();
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
        // add endl if it doesn't exist
        if !buffer.ends_with(&[b'\n']) 
        {
          buffer.push(b'\n');
        }
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

  // read
  unsafe 
  {
    parseLines( readTokens(buffer, _debugMode) );
  }

  // duration
  if unsafe{_debugMode} 
  {
    let endTime:  Instant  = Instant::now();
    let duration: Duration = endTime-startTime;
    log("ok",&format!("All duration [{:?}]",duration));
  }
  // ** to release test, use hyperfine/perf

  //
  Ok(())
}
