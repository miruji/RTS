/*
    spl init file
*/
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

#[macro_use]
extern crate lazy_static;

use std::fs::File;
use std::io::{self, Read};
use std::env;

mod logger;
mod tokenizer;
mod parser;

pub static _version: &str = "0.2.0";

pub static mut _filePath: String = String::new();

pub static mut _argc: usize       = 0;
pub static mut _argv: Vec<String> = Vec::new();

pub static mut _debugMode: bool = false;

fn main() -> io::Result<()> {
    use crate::logger::*;
    use crate::tokenizer::*;
    use crate::parser::*;

    // get args -> key-values
    let mut argsKeys: Vec<(String, Vec<String>)> = Vec::new();
    {
        let args: Vec<String> = env::args().collect();
        let mut keyValues: Vec<String> = Vec::new();
        let mut readKey: String = String::new();
        for arg in args.iter().skip(1) {
            //if (arg.len() >= 2 && &arg[0..2] == "--") ||
            if arg.len() >= 1 && &arg[0..1] == "-" {
                // --
                if !readKey.is_empty() {
                    argsKeys.push((readKey.clone(), keyValues.clone()));
                    keyValues.clear();
                }
                readKey = arg.clone();
            } else {
                // read key
                if !readKey.is_empty() {
                    keyValues.push(arg.clone());
                }
            }
        }
        if !readKey.is_empty() {
            argsKeys.push((readKey.clone(), keyValues.clone()));
            keyValues.clear();
        }
    }

    // debug mode on ?
    for (key, values) in &argsKeys {
        // version
        if key == "-v" {
            // todo: version save file ?
            println!("spl v{}",unsafe{_version});
            std::process::exit(0);
        }
        // debug mode
        if key == "-d" {
            // todo: debug sectors
            // e: ast, structs, interpritation
            unsafe{_debugMode = true;}
            break;
        }
    }
    if unsafe{_debugMode} {
        logSeparator("=> Reading arguments");
        log("ok","Debug mode");
    }

    // read args
    let mut noRun: bool = true;

    for (key, values) in &argsKeys {
        let valuesLength: usize = (&values).len();
        // realtime run
        if key == "-r" {
            unsafe{
                _argc = valuesLength-1;
                _argv = values.clone();
                _argv.remove(0); // remove file name
                _filePath = values[0].clone();
            }
            // todo: check filePath file type
            noRun = false;
            if unsafe{_debugMode} {
                log("ok",&format!("Run \"{}\"",unsafe{&*_filePath}));
            }
        }
    }

    if noRun {
        log("err","Use the [-r <filename>] flag");
        logExit();
    }

    if unsafe{_debugMode} {
        logSeparator("=> Opening a file");
    }

    // open file
    let mut file = match File::open(unsafe{&*_filePath}) {
        Ok(file) => {
            if unsafe{_debugMode} {
                log("ok",&format!("Opening the file \"{}\" was successful",unsafe{&*_filePath}));
            }
            file
        },
        Err(_) => {
            log("err",&format!("Unable to opening file \"{}\"",unsafe{&*_filePath}));
            logExit();
            std::process::exit(1)
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
            if unsafe{_debugMode} {
                log("ok",&format!("Reading the file \"{}\" was successful",unsafe{&*_filePath}));
            }
        }
        Err(_) => {
            log("err",&format!("Unable to read file \"{}\"",unsafe{&*_filePath}));
            logExit();
            ()
        }
    }

    if unsafe{_debugMode} {
        logSeparator("=> AST generation");
    }

    // read
    unsafe {
        parseLines( readTokens(buffer) );
    }

    //
    Ok(())
}
