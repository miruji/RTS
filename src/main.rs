/*
    RTS init file
*/
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

#[macro_use]
extern crate lazy_static;

use std::fs::File;
use std::io::{self, Read};
use std::env;
use std::time::Instant;

mod logger;
mod tokenizer;
mod parser;
// other globals
pub static mut _filePath: String = String::new(); // todo: ?
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
// main
fn main() -> io::Result<()> 
{
  let startTime: Instant = Instant::now();

  //
  use crate::logger::*;
  use crate::tokenizer::*;
  use crate::parser::*;

  // args to key-values
  let mut args: Vec<(String, Vec<String>)> = Vec::new(); // Vector< key-values >
  {
    let          input: Vec<String>    = env::args().collect(); // input argv
    let mut     values: Vec<String>    = Vec::new();            // values
    let mut readBuffer: Option<String> = None;                  // buffer

    for arg in input.iter().skip(1) // skip first arg
    { 
        if arg.starts_with('-') 
        { // read key
            if let Some(key) = readBuffer.take() 
            { // use `take` to get the current key
                args.push((key, values.clone()));
                values.clear();
            }
            readBuffer = Some(arg.clone());
        } else if let Some(_) = readBuffer 
        { // read values
            values.push(arg.clone());
        }
    }

    if let Some(key) = readBuffer 
    { // set last key
        args.push((key, values));
    }
  }

  // debug mode on ?
  for (key, _) in &args 
  { // read keys
      match key.as_str() 
      {
          "-v" => 
          { // version
              log("ok",&format!("RTS v{}",*_version));
              logExit();
          }
          "-d" => 
          { // debug mode
              unsafe { _debugMode = true; }
          }
          _ => {}
      }
  }
  if unsafe{_debugMode} 
  {
      logSeparator("Arguments");
      log("ok","Debug mode");
  }

  // read args
  let mut   noRun: bool = true;
  let mut runFile: bool = false;
  let mut  buffer: Vec<u8> = Vec::new();

  for (key, values) in &args 
  {
    let valuesLength: usize = (&values).len();
    match key.as_str() 
    {
      "-rf" => 
      { // run file
          unsafe
          {
              _argc = valuesLength-1;
              _argv = values.clone();
              _argv.remove(0); // remove file name
              _filePath = values[0].clone();
          }

          // todo: check filePath file type
          noRun = false;
          if unsafe{_debugMode} {
              log("ok",&format!("Run [{}]",unsafe{&*_filePath}));
          }
          runFile = true;
      } 
      "-rs" => 
      { // run script
          let combinedString: String = values.concat().replace("\\n", "\n"); // todo: \\n ?
          buffer = combinedString.clone().into_bytes();
          // todo: argc & argv

          // todo: check filePath file type
          noRun = false;
          if unsafe{_debugMode} 
          {
              log("ok",&format!("Run [{}]",combinedString));
          }
      }
      _ => {}
    }
  }
  
  if noRun 
  {
      log("err","Use the [-rf <filename>] or [-rs \"<script>\"] flag");
      logExit();
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
              logExit();
              std::process::exit(1)
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
              logExit();
              ()
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
      let endTime  = Instant::now();
      let duration = endTime-startTime;
      log("ok",&format!("All duration [{:?}]",duration));
  }
  // ** to release test, use hyperfine/perf

  //
  Ok(())
}
