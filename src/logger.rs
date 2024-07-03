/*
    Logger
*/

use termion::color::{Bg, Fg, Rgb};
use termion::style;

// hex str -> termion::color::Rgb
fn hexToTermionColor(hex: &str) -> Option<Rgb> {
    if hex.len() == 6 {
        let r: u8 = u8::from_str_radix(&hex[0..2], 16).ok()?;
        let g: u8 = u8::from_str_radix(&hex[2..4], 16).ok()?;
        let b: u8 = u8::from_str_radix(&hex[4..6], 16).ok()?;
        Some(Rgb(r, g, b))
    } else {
        None
    }
}
// devide white space, begin from the left
fn divideWhitespace(input: &str) -> Vec<&str> {
    let first_non_space_index = input.find(|c: char| !c.is_whitespace()).unwrap_or(input.len());
    vec![&input[..first_non_space_index], &input[first_non_space_index..]]
}
// style log
fn logWithStyle(string: &str) {
	print!("{}",&formatPrint(string));
}
pub fn formatPrint(string: &str) -> String {
    let mut result = String::new();
    let mut i = 0;
    let string: Vec<char> = string.chars().collect();
    let stringLength      = string.len();

    let mut bracketString: String;
    let mut bracketColor: Option<Rgb>;

    while i < stringLength {
        // special
        if string[i] == '\\' && i+1 < stringLength {
            match string[i+1] {
                //
                'b' => {
                    if i+2 < stringLength && string[i+2] == 'g' {
                        i += 5;
                        bracketString = String::new();
                        for j in i..stringLength {
                            if string[j] == ')' {
                                break;
                            }
                            bracketString.push(string[j]);
                        }
                        bracketColor = hexToTermionColor(&bracketString);
                        result.push_str(&format!(
                            "{}",
                            Bg(bracketColor.unwrap_or_else(|| Rgb(0, 0, 0)))
                        ));
                        i += bracketString.len()+1;
                        continue;
                    } else {
                        result.push_str( &format!("{}",style::Bold) );
                        i += 2;
                        continue;
                    }
                },
                'c' => {
                    i += 2;
                    result.push_str( &format!("{}",style::Reset) );
                    continue;
                },
                'f' => {
                    if i+2 < stringLength && string[i+2] == 'g' {
                        i += 5;
                        bracketString = String::new();
                        for j in i..stringLength {
                            if string[j] == ')' {
                                break;
                            }
                            bracketString.push(string[j]);
                        }
                        bracketColor = hexToTermionColor(&bracketString);
                        result.push_str(&format!(
                            "{}",
                            Fg(bracketColor.unwrap_or_else(|| Rgb(0, 0, 0)))
                        ));
                        i += bracketString.len()+1;
                        continue;
                    }
                },
                _ => {
                    i += 2;
                    continue;
                }
            }
        // basic
        } else {
            result.push( string[i] );
        }
        i += 1;
    }
    return result;
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
pub fn log(textType: &str, text: &str) {
	// syntax error
	if textType == "syntax" {
		logWithStyle("\\fg(#e91a34)\\bSyntax \\c");
	} else
	// AST open +
	if textType == "parserBegin" {
		let divide: Vec<&str> = divideWhitespace(text);
		logWithStyle(&format!(
			"{}\\bg(#29352f)\\fg(#b5df90)\\b{}\\c\n",
			divide[0],
			divide[1]
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
		let divide: Vec<&str> = divideWhitespace(text);
		logWithStyle(&format!(
			"{}\\bg(#29352f)\\fg(#d9d9d9)\\b{}\\c\n",
			divide[0],
			divide[1]
		));
	} else
	// AST token
	if textType == "parserToken" {
		let parts: Vec<&str> = text.split("|").collect();
		let mut outputParts: Vec<String> = Vec::new();
		// first word no format
		if let Some(firstPart) = parts.first() {
			outputParts.push(
				formatPrint(firstPart)
			);
		}
		// last word
		for part in parts.iter().skip(1) {
			outputParts.push(
				formatPrint(&format!(
					"\\b\\fg(#d9d9d9){}\\c",
					part
				))
			);
		}
		println!("{}", outputParts.join(""));
	} else
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
		let parts: Vec<&str> = text.split("->").collect();
		let outputParts: String = 
			parts.join(
				&formatPrint("\\fg(#f0f8ff)\\b->\\c")
		);
		logWithStyle(&format!(
			"\\fg(#f0f8ff)\\b->\\c \\fg(#f0f8ff){}\\c\n",
			outputParts
		));
	} else
	// line
	if textType == "line" {
		if let Some(textColor) = hexToTermionColor("d9d9d9") {
			let parts: Vec<&str> = text.split("|").collect();
			let mut outputParts: Vec<String> = Vec::new();
			// left
			if let Some(firstPart) = parts.first() {
				outputParts.push(
					formatPrint(&format!(
						"  \\fg(#f0f8ff)\\b{} | \\c",
						firstPart.to_string()
					))
				);
			}
			// right
			for part in parts.iter().skip(1) {
				outputParts.push(part.to_string());
			}
			println!("{}",outputParts.join(""));
		}
	// basic
	} else {
		logWithStyle(&format!(
			"\\fg(#f0f8ff){}\\c\n",
			text
		));
	}
}