/*
    log
*/

use termion::color::{Bg, Fg, Rgb};
use termion::style;

// hex str -> termion::color::Rgb
fn hexToTermionColor(hex: &str) -> Option<Rgb> {
    if hex.len() != 6 {
        return None;
    }

    let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
    let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
    let b = u8::from_str_radix(&hex[4..6], 16).ok()?;

    Some(Rgb(r, g, b))
}
//
fn divideWhitespace(input: &str) -> Vec<&str> {
    let mut iter = input.chars();
    let mut firstNonSpaceIndex = 0;
    // search first no white
    while let Some(c) = iter.next() {
        if !c.is_whitespace() {
            break;
        }
        firstNonSpaceIndex += 1;
    }
    //
    vec![&input[..firstNonSpaceIndex], &input[firstNonSpaceIndex..]]
}
// start log
pub fn logSeparator(text: &str) {
	if let Some(textColor) = hexToTermionColor("4d8af9") {
		println!(
			"{}{}{}{}",
			style::Bold,
			Fg(textColor),
			text,
			style::Reset,
		);
	}
}
// exit log
pub fn logExit() {
	if let Some(textColor) = hexToTermionColor("f94d4d") {
		println!(
			"{}{}Exit 1{} {}:({}",
			style::Bold,
			Fg(textColor),
			style::Reset,
			style::Bold,
			style::Reset,
		);
	}
	std::process::exit(1);
}
// basic style log
pub fn log(textType: &str, text: &str) {
	if textType == "info" {
		if let Some(bgColor) = hexToTermionColor("1a1a1a") {
		if let Some(textColor) = hexToTermionColor("f0f8ff") {
			let devide = divideWhitespace(text);
			println!(
				"{}{}{}{}Info{}: {}{}",
				devide[0],
				Bg(bgColor),
				Fg(textColor),
				style::Bold,
				style::Reset,
				devide[1],
				style::Reset
			);
		} }
	} else
	if textType == "fatal" {
		if let Some(textColor) = hexToTermionColor("e91a34") {
			println!(
				"{}{}Fatal{}: {}{}{}",
				style::Bold,
				Fg(textColor),
				style::Reset,
				style::Bold,
				text,
				style::Reset
			);
		}
	} else
	if textType == "warning" {
		if let Some(textColor) = hexToTermionColor("ffd589") {
			println!(
				"{}{}Warning{}: {}{}{}",
				style::Bold,
				Fg(textColor),
				style::Reset,
				style::Bold,
				text,
				style::Reset
			);
		}
	} else
	//
	if textType == "syntax" {
		if let Some(textColor) = hexToTermionColor("e91a34") {
			println!(
				"{}{}Syntax{}: {}{}{}",
				style::Bold,
				Fg(textColor),
				style::Reset,
				style::Bold,
				text,
				style::Reset
			);
		}
	} else
	if textType == "parserBegin" { // open +
		if let Some(bgColor) = hexToTermionColor("29352f") {
		if let Some(textColor) = hexToTermionColor("c6df90") {
			let devide = divideWhitespace(text);
			println!(
				"{}{}{}{}{}{}",
				devide[0],
				Bg(bgColor),
				Fg(textColor),
				style::Bold,
				devide[1],
				style::Reset
			);
		} }
	} else
	if textType == "parserHeader" { // header
		if let Some(textColor) = hexToTermionColor("c6df90") {
			println!(
				"{}{}{}{}",
				Fg(textColor),
				style::Bold,
				text,
				style::Reset
			);
		}
	} else
	if textType == "parserEnd" { // end -
		if let Some(bgColor) = hexToTermionColor("29352f") {
		if let Some(textColor) = hexToTermionColor("fb9950") {
			let devide = divideWhitespace(text);
			println!(
				"{}{}{}{}{}{}",
				devide[0],
				Bg(bgColor),
				Fg(textColor),
				style::Bold,
				devide[1],
				style::Reset
			);
		} }
	} else
	if textType == "parserInfo" { // info
    	if let Some(bgColor) = hexToTermionColor("29352f") {
		if let Some(textColor) = hexToTermionColor("d9d9d9") {
			println!(
				"{}{}{}{}{}",
				Bg(bgColor),
				Fg(textColor),
				style::Bold,
				text,
				style::Reset
			);
		} }
	} else
	if textType == "parserToken" {
		if let Some(textColor) = hexToTermionColor("d9d9d9") {
			let parts: Vec<&str> = text.split("|").collect();
			let mut outputParts: Vec<String> = Vec::new();

			// first word no format
			if let Some(firstPart) = parts.first() {
				outputParts.push(firstPart.to_string());
			}
			//
			for part in parts.iter().skip(1) {
				outputParts.push(format!(
					"{}{}{}{}",
					style::Bold,
					Fg(textColor),
					part,
					style::Reset
				));
			}

			println!("{}", outputParts.join(""));
		}
	} else
	//
	if textType == "ok" { // ok
		if let Some(firstTextColor)  = hexToTermionColor("55af96") {
		if let Some(secondTextColor) = hexToTermionColor("f0f8ff") {
			println!(
				"{}{}+{} {}{}{}{}",
				style::Bold,
				Fg(firstTextColor),
				style::Reset,
				style::Bold,
				Fg(secondTextColor),
				text,
				style::Reset
			);
		} }
	} else
	if textType == "err" { // error
		if let Some(firstTextColor)  = hexToTermionColor("e91a34") {
		if let Some(secondTextColor) = hexToTermionColor("f0f8ff") {
			println!(
				"{}{}-{} {}{}{}{}",
				style::Bold,
				Fg(firstTextColor),
				style::Reset,
				style::Bold,
				Fg(secondTextColor),
				text,
				style::Reset
			);
		} }
	} else
	if textType == "note" {
		if let Some(textColor) = hexToTermionColor("f0f8ff") {
			println!(
				"{}{}Note:{} {}",
				style::Bold,
				Fg(textColor),
				style::Reset,
				text,
			);
		}
	} else
	if textType == "path" {
		if let Some(textColor) = hexToTermionColor("f0f8ff") {
		    let parts: Vec<&str> = text.split("->").collect();
		    let outputParts: String = parts.join(
		    	&format!(
		    		"{}{}->{}",
		    		style::Bold,
		    		Fg(textColor),
		    		style::Reset
	    		)
	    	);
		    let outputParts = 
		    	&format!(
		    		"  {}{}-> {}{}",
		    		style::Bold,
		    		Fg(textColor),
		    		style::Reset,
		    		outputParts
    			);
			println!(
				"{}",
				outputParts,
			);
		}
	} else {
		if let Some(textColor) = hexToTermionColor("f0f8ff") {
			println!(
				"{}{}{}",
				Fg(textColor),
				text,
				style::Reset
			);
		}
	}
}