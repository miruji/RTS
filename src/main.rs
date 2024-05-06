/*
    spl init file
*/

use std::fs::File;
use std::io::{self, Read};
use std::env;

mod tokenizer;
mod parser;

fn main() -> io::Result<()> {
    use crate::tokenizer::*;
    use crate::parser::*;

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
    let mut no_run: bool = true;
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

    // read
    parse_lines( &mut read_tokens(buffer) );

    //
    Ok(())
}
