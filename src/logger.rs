/*
    Logger
*/

use termion::color::{Bg, Fg, Rgb};
use termion::style;

// hex str -> termion::color::Rgb
fn hexToTermionColor(hex: &str) -> Option<Rgb> {
    if hex.len() == 6 {
        Some(Rgb(
          u8::from_str_radix(&hex[0..2], 16).ok()?, 
          u8::from_str_radix(&hex[2..4], 16).ok()?, 
          u8::from_str_radix(&hex[4..6], 16).ok()?
        ))
    } else {
        None
    }
}
// devide white space, begin from the left
fn divideWhitespace(input: &str) -> (&str, &str) {
    let firstNonSpaceIndex = input.find(|c: char| !c.is_whitespace()).unwrap_or(input.len());
    (&input[..firstNonSpaceIndex], &input[firstNonSpaceIndex..])
}
// style log
fn logWithStyle(string: &str) {
  print!("{}",&formatPrint(string));
}

static mut _result: String = String::new();

static mut _i:            usize = 0;
static mut _stringLength: usize = 0;

static mut _stringChars:   Vec<char>   = Vec::new();
static mut _string:        String      = String::new();
static mut _bracketColor:  Option<Rgb> = None;

pub fn formatPrint(string: &str) -> String {
    unsafe{
      _result = String::new();

      _i = 0;
      _stringChars  = string.chars().collect();
      _stringLength = _stringChars.len();

      _string       = String::new(); // bracket string
      _bracketColor = None;

      while _i < _stringLength {
          // special
          if _stringChars[_i] == '\\' && _i+1 < _stringLength {
              match _stringChars[_i+1] {
                  //
                  'b' => {
                      if _i+2 < _stringLength && _stringChars[_i+2] == 'g' {
                          _i += 5;
                          _string = String::new();
                          for j in _i.._stringLength {
                              if _stringChars[j] == ')' {
                                  break;
                              }
                              _string.push(_stringChars[j]);
                          }
                          _bracketColor = hexToTermionColor(&_string);
                          _result.push_str(&format!(
                              "{}",
                              Bg(_bracketColor.unwrap_or_else(|| Rgb(0, 0, 0)))
                          ));
                          _i += _string.len()+1;
                          continue;
                      } else {
                          _result.push_str( &format!("{}",style::Bold) );
                          _i += 2;
                          continue;
                      }
                  },
                  'c' => {
                      _i += 2;
                      _result.push_str( &format!("{}",style::Reset) );
                      continue;
                  },
                  'f' => {
                      if _i+2 < _stringLength && _stringChars[_i+2] == 'g' {
                          _i += 5;
                          _string = String::new();
                          for j in _i.._stringLength {
                              if _stringChars[j] == ')' {
                                  break;
                              }
                              _string.push(_stringChars[j]);
                          }
                          _bracketColor = hexToTermionColor(&_string);
                          _result.push_str(&format!(
                              "{}",
                              Fg(_bracketColor.unwrap_or_else(|| Rgb(0, 0, 0)))
                          ));
                          _i += _string.len()+1;
                          continue;
                      }
                  },
                  _ => {
                      _i += 2;
                      continue;
                  }
              }
          // basic
          } else {
              _result.push( _stringChars[_i] );
          }
          _i += 1;
      }
      return _result.clone();
    }
}
// separator log
pub fn logSeparator(text: &str) {
    logWithStyle(&format!("\\fg(#4d8af9)\\b{}\\c\n",text));
}
// exit log
pub fn logExit() {
  logWithStyle("\\fg(#f94d4d)\\bExit 1\\c \\fg(#f0f8ff)\\b:(\\c\n");
    std::process::exit(1);
}
// basic style log
static mut _parts:       Vec<String> = Vec::new();
static mut _outputParts: Vec<String> = Vec::new();
pub fn log(textType: &str, text: &str) {
  // syntax error
  if textType == "syntax" {
    logWithStyle("\\fg(#e91a34)\\bSyntax \\c");
  } else
  // AST open +
  if textType == "parserBegin" {
    let (divide1, divide2): (&str, &str) = divideWhitespace(text);
    logWithStyle(&format!(
      "{}\\bg(#29352f)\\fg(#b5df90)\\b{}\\c\n",
      divide1,
      divide2
    ));
  } else
  // AST header
  if textType == "parserHeader" {
    logWithStyle(&format!(
      "\\fg(#90df91)\\b{}\\c\n",
      text
    ));
  } else
  // AST info
  if textType == "parserInfo" {
    let (divide1, divide2): (&str, &str) = divideWhitespace(text);
    logWithStyle(&format!(
      "{}\\bg(#29352f)\\fg(#d9d9d9)\\b{}\\c\n",
      divide1,
      divide2
    ));
  } else
  // AST token
  if textType == "parserToken" {
  unsafe{
    _parts = text.split("|").map(|s| s.to_string()).collect();
    _outputParts = Vec::new();
    // first word no format
    if let Some(firstPart) = _parts.first() {
      _outputParts.push(
        formatPrint(firstPart)
      );
    }
    // last word
    for part in _parts.iter().skip(1) {
      _outputParts.push(
        formatPrint(&format!(
          "\\b\\fg(#d9d9d9){}\\c",
          part
        ))
      );
    }
    println!("{}", _outputParts.join(""));
  }} else
  // ok
  if textType == "ok" {
    logWithStyle(&format!(
      "  \\fg(#55af96)\\b+\\c \\fg(#f0f8ff)\\b{}\\c\n",
      text
    ));
  } else
  // error
  if textType == "err" {
    logWithStyle(&format!(
      "  \\fg(#e91a34)\\b-\\c \\fg(#f0f8ff)\\b{}\\c\n",
      text
    ));
  } else
  // note
  if textType == "note" {
    logWithStyle(&format!(
      "  \\fg(#f0f8ff)\\bNote:\\c \\fg(#f0f8ff){}\\c\n",
      text
    ));
  } else
  // path
  if textType == "path" {
  unsafe{
    _parts = text.split("->").map(|s| s.to_string()).collect();
    _string = 
      _parts.join(
        &formatPrint("\\fg(#f0f8ff)\\b->\\c")
    );
    logWithStyle(&format!(
      "\\fg(#f0f8ff)\\b->\\c \\fg(#f0f8ff){}\\c\n",
      _string
    ));
  }} else
  // line
  if textType == "line" {
  unsafe{
    if let Some(textColor) = hexToTermionColor("d9d9d9") {
      _parts = text.split("|").map(|s| s.to_string()).collect();
      _outputParts = Vec::new();
      // left
      if let Some(firstPart) = _parts.first() {
        _outputParts.push(
          formatPrint(&format!(
            "  \\fg(#f0f8ff)\\b{} | \\c",
            firstPart.to_string()
          ))
        );
      }
      // right
      for part in _parts.iter().skip(1) {
        _outputParts.push(part.to_string());
      }
      println!("{}",_outputParts.join(""));
    }
  // basic
  }} else {
    logWithStyle(&format!(
      "\\fg(#f0f8ff){}\\c\n",
      text
    ));
  }
}