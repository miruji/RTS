/*
    spl init file
*/

use std::fs::File;
use std::io::{self, Read};
use std::env;

mod tokenizer;

fn main() -> io::Result<()> {
    use crate::tokenizer::token::token::*;
    use crate::tokenizer::tokenizer::*;

    // get args --> key-values
    let mut args_keys: Vec<(String, Vec<String>)> = Vec::new();
    {
        let args: Vec<String> = env::args().collect();
        let mut key_values: Vec<String> = Vec::new();
        let mut read_key: String = String::new();
        for arg in args.iter().skip(1) {
            if (arg.len() >= 2 && &arg[0..2] == "--") ||
               (arg.len() >= 1 && &arg[0..1] == "-") {
                // --
                if !read_key.is_empty() {
                    args_keys.push((read_key.clone(), key_values.clone()));
                    key_values.clear();
                }
                read_key = arg.clone();
            } else {
                // read key
                if !read_key.is_empty() {
                    key_values.push(arg.clone());
                }
            }
        }
        if !read_key.is_empty() {
            args_keys.push((read_key.clone(), key_values.clone()));
            key_values.clear();
        }
    }

    let mut file_path: String = String::new();
    let mut no_run = true;
    for (key, values) in &args_keys {
        if key == "-rr" {
            if (&values).len() == 1 {
                file_path = values[0].clone();
                no_run = false;
            } else {
                println!("[LOG][WARNING] Key [-rr] only accepts one value");
            }
        } else if key == "--debug" {
            println!("[LOG][INFO] Started with debug mode");
        }
    }

    if no_run {
        eprintln!("[LOG][WARNING] Use the -rr <filename> flag");
        std::process::exit(1);
    }

    // open file
    let mut file = match File::open(&file_path) {
        Ok(file) => file,
        Err(_) => {
            eprintln!("[LOG][FATAL] Unable to opening file [{}]", &file_path);
            std::process::exit(1);
        }
    };
    // read file into buffer
    let mut buffer = Vec::new();
    match file.read_to_end(&mut buffer) {
        Ok(_) => {
            // add endl if it doesn't exist
            if !buffer.ends_with(&[b'\n']) {
                buffer.push(b'\n');
            }
        }
        Err(_) => {
            eprintln!("[FATAL] Unable to read file [{}]", &file_path);
            std::process::exit(1);
        }
    }

    // read tokens
    let mut tokens: Vec<Token> = Vec::new();

    let buffer_length = buffer.len();
    let mut index = 0;
    while index < buffer_length {
        let c = buffer[index] as char;

        // delete comment
        if c == '#' {
            delete_comment(&mut index, &buffer, &buffer_length);
        } else
        // get endline + end of file
        if c == '\n' {
            tokens.push( Token { data_type: TokenType::Endline, data: "".to_string()} );
            index += 1;
        } else
        // get number
        if c.is_digit(10) {
            tokens.push( get_number(&mut index, &buffer, &buffer_length) );
        } else
        // get word
        if c.is_alphabetic() {
            tokens.push( get_word(&mut index, &buffer, &buffer_length) );
        } else
        // get quotes ' " `
        if c == '\'' || c == '"' || c == '`' {
            let token = get_quotes(buffer[index], &mut index, &buffer);
            if token.data_type != TokenType::None {
                tokens.push(token);
            } else {
                index += 1;
            }
        } else
        // get single and double chars
        if get_single_char(c) {
            let token = get_operator(&mut index, &buffer);
            if token.data_type != TokenType::None {
                tokens.push(token);
            } else {
                index += 1;
            }
            // skip
        } else {
            index += 1;
        }
    }

    // output tokens
    println!("[LOG][INFO] Tokens:");
    for token in &tokens {
        if !token.data.is_empty() {
            println!("  [{}] [{}]", token.data_type.to_string(), token.data);
        } else {
            println!("  [{}]", token.data_type.to_string());
        }
    }

    Ok(())
}
