/* /logger
  Has both debug functions and for normal work
*/

use crate::{
  _debugMode,
};

use termion::color::{Bg, Fg, Rgb, Reset};
use termion::style;

// hex str -> termion::color::Rgb
fn hexToTermionColor(hex: &str) -> Option<Rgb> {
  if hex.len() != 6 { return None; }

  Some(Rgb(
    u8::from_str_radix(&hex[0..2], 16).ok()?, 
    u8::from_str_radix(&hex[2..4], 16).ok()?, 
    u8::from_str_radix(&hex[4..6], 16).ok()?
  ))
}
// devide white space, begin from the left
fn divideWhitespace(input: &str) -> (&str, &str) 
{
  let firstNonSpaceIndex: usize = input
    .find(|c: char| !c.is_whitespace())
    .unwrap_or(input.len());
  (&input[..firstNonSpaceIndex], &input[firstNonSpaceIndex..])
}
// style log
pub fn formatPrint(string: &str) -> ()
{
  print!("{}",&formatString(string));
}

static mut _result: String = String::new();

static mut _i:            usize = 0;
static mut _stringLength: usize = 0;

static mut _stringChars:   Vec<char>   = Vec::new();
static mut _string:        String      = String::new();

/*
  Formats a string, you can use flags:

  \c    clear all
  
  \b    bold
  \fg   foreground
  \bg   background

  \cb   clear bold
  \cfg  clear foreground
  \cbg  clear background
*/
fn formatString(string: &str) -> String 
{
  unsafe
  {
    _result = String::new();

    _i = 0;
    _stringChars  = string.chars().collect();
    _stringLength = _stringChars.len();

    while _i < _stringLength 
    {
      // special
      if _stringChars[_i] == '\\' && _i+1 < _stringLength 
      {
        match _stringChars[_i+1] 
        {
          'b' => 
          {
            if _i+2 < _stringLength && _stringChars[_i+2] == 'g' 
            { // bg
              _i += 5;
              _string = String::from_iter(
                _stringChars[_i.._stringLength]
                  .iter()
                  .take_while(|&&c| c != ')')
              );
              _result.push_str(&format!(
                  "{}",
                  Bg(hexToTermionColor(&_string).unwrap_or_else(|| Rgb(0, 0, 0)))
              ));
              _i += _string.len()+1;
              continue;
            } else 
            { // bold
              _result.push_str( &format!("{}",style::Bold) );
              _i += 2;
              continue;
            }
          },
          'f' => 
          {
            if _i+2 < _stringLength && _stringChars[_i+2] == 'g' 
            { // fg
                _i += 5;
                _string = String::from_iter(
                  _stringChars[_i.._stringLength]
                    .iter()
                    .take_while(|&&c| c != ')')
                );
                _result.push_str(&format!(
                    "{}",
                    Fg(hexToTermionColor(&_string).unwrap_or_else(|| Rgb(0, 0, 0)))
                ));
                _i += _string.len()+1;
                continue;
            }
          },
          'c' => 
          { // clear
            if _i+2 < _stringLength && _stringChars[_i+2] == 'b' 
            {
              if _i+3 < _stringLength && _stringChars[_i+3] == 'g' 
              { // cbg
                _i += 4;
                _result.push_str(&format!(
                  "{}",
                  Bg(Reset)
                ));
                continue;
              } else 
              { // cb
                _i += 3;
                _result.push_str(&format!(
                  "{}",
                  style::NoBold
                ));
                continue;
              }
            } else
            if _i+2 < _stringLength && _stringChars[_i+2] == 'f' 
            {
              if _i+3 < _stringLength && _stringChars[_i+3] == 'g' 
              { // cfg
                _i += 4;
                _result.push_str(&format!(
                  "{}",
                  Fg(Reset)
                ));
                continue;
              }
            } else 
            { // clear all
              _i += 2;
              _result.push_str(&format!(
                "{}",
                style::Reset
              ));
              continue;
            }
          },
          _ => 
          {
            _i += 2;
            continue;
          }
        }
      // basic
      } else 
      {
        _result.push( _stringChars[_i] );
      }
      _i += 1;
    }
    return _result.clone();
  }
}
// separator log
pub fn logSeparator(text: &str) -> ()
{
  formatPrint(&format!(
    " \\fg(#55af96)\\bx \\fg(#0095B6){}\\c\n",
    text
  ));
}
// Завершает программу и при необходимости в debug режиме
// возвращает описание выхода;
pub fn logExit(code: i32) -> !
{
  // В данном случае завершение успешно;
  if code == 0 {
    if unsafe{_debugMode}
    {
      formatPrint("   \\b┗\\fg(#1ae96b) Exit 0\\c \\fg(#f0f8ff)\\b:)\\c\n");
    }
    std::process::exit(0);
  }
  // В данном случае завершение не является успешное;
  if unsafe{_debugMode}
  { 
    formatPrint(
      &format!(
        "   \\b┗\\fg(#e91a34) Exit {}\\c \\fg(#f0f8ff)\\b:(\\c\n", 
        code
      )
    );
  }
  std::process::exit(code);
}
// basic style log
static mut _parts:       Vec<String> = Vec::new();
static mut _outputParts: Vec<String> = Vec::new();
pub fn log(textType: &str, text: &str) -> ()
{
  if textType == "syntax" 
  {
    formatPrint("\\fg(#e91a34)\\bSyntax \\c");
  } else
  // AST open +
  if textType == "parserBegin" 
  {
    let (divide1, divide2): (&str, &str) = divideWhitespace(text);
    formatPrint(&format!(
      "{}\\bg(#29352f)\\fg(#b5df90)\\b{}\\c\n",
      divide1,
      divide2
    ));
  } else
  // AST info
  if textType == "parserInfo" 
  {
    let (divide1, divide2): (&str, &str) = divideWhitespace(text);
    formatPrint(&format!(
      "{}\\bg(#29352f)\\fg(#d9d9d9)\\b{}\\c\n",
      divide1,
      divide2
    ));
  } else
  // AST token
  if textType == "parserToken" 
  {unsafe{
    _parts = text.split("|").map(|s| s.to_string()).collect();
    _outputParts = Vec::new();
    // first word no format
    if let Some(firstPart) = _parts.first() 
    {
      _outputParts.push(
        formatString(firstPart)
      );
    }
    // last word
    for part in _parts.iter().skip(1) 
    {
      _outputParts.push(
        formatString(&format!(
          "\\b\\fg(#d9d9d9){}\\c",
          part
        ))
      );
    }
    println!("{}", _outputParts.join(""));
  }} else
  // ok
  if textType == "ok" 
  {
    let (content, prefix): (&str, &str) = 
      if text.starts_with('+') 
      {
        (&text[1..], "O\\cfg \\fg(#f0f8ff)┳")
      } else
      if text.starts_with('x') 
      {
        (&text[1..], "X\\cfg \\fg(#f0f8ff)┻")
      } else 
      {
        (text, "+")
      };
    formatPrint(&format!(
      "   \\fg(#1ae96b)\\b{}\\cb\\cfg \\fg(#f0f8ff)\\b{}\\c\n",
      prefix,
      content
    ));
  } else
  // error
  if textType == "err" 
  {
    formatPrint(&format!(
      "   \\fg(#e91a34)\\b-\\cb\\cfg \\fg(#f0f8ff)\\b{}\\c\n",
      text
    ));
  } else
  // warning
  if textType == "warn" 
  {
    formatPrint(&format!(
      "   \\fg(#e98e1a)\\b?\\cb\\cfg \\fg(#f0f8ff)\\b{}\\c\n",
      text
    ));
  } else
  // warn input
  if textType == "warn-input" 
  {
    formatPrint(&format!(
      "   \\fg(#e98e1a)\\b?\\cb\\cfg \\fg(#f0f8ff)\\b{}\\c",
      text
    ));
  } else
  // note
  if textType == "note" 
  {
    formatPrint(&format!(
      "  \\fg(#f0f8ff)\\bNote:\\c \\fg(#f0f8ff){}\\c\n",
      text
    ));
  } else
  // path
  if textType == "path" 
  {unsafe{
    _parts = text.split("->").map(|s| s.to_string()).collect();
    _string = 
      _parts.join(
        &formatString("\\fg(#f0f8ff)\\b->\\c")
      );
    formatPrint(&format!(
      "\\fg(#f0f8ff)\\b->\\c \\fg(#f0f8ff){}\\c\n",
      _string
    ));
  }} else
  // line
  if textType == "line" 
  {unsafe{
    _parts = text.split("|").map(|s| s.to_string()).collect();
    _outputParts = Vec::new();
    // left
    if let Some(firstPart) = _parts.first() 
    {
      _outputParts.push(
        formatString(&format!(
          "  \\fg(#f0f8ff)\\b{} | \\c",
          firstPart.to_string()
        ))
      );
    }
    // right
    for part in _parts.iter().skip(1) 
    {
      _outputParts.push(part.to_string());
    }
    println!("{}",_outputParts.join(""));
  // basic
  }} else 
  {
    formatPrint(&format!(
      "\\fg(#f0f8ff){}\\c\n",
      text
    ));
  }
}