/*
    spl init file
*/
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use std::fs::File;
use std::io::{self, Read};
use std::env;

mod logger;
mod tokenizer;
mod parser;

pub static mut filePath: String = String::new();
fn main() -> io::Result<()> {
    use crate::logger::*;
    use crate::tokenizer::*;
    use crate::parser::*;

    logSeparator("=> Reading arguments");

    // get args --> key-values
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

    let mut noRun:     bool = true;
    let mut debugMode: bool = false;
    for (key, values) in &argsKeys {
        let valuesLength: usize = (&values).len();
        // realtime run
        if key == "-r" {
            if valuesLength == 1 {
                unsafe{ filePath = values[0].clone() };
                // todo: check filePath file type
                noRun = false;
                log("ok",&format!("Run \"{}\"",unsafe{&*filePath}));
            // valuesLength == 0 -> in noRun if
            } else {
                log("err","Key [-r] only accepts one value");
                logExit();
            }
        } else
        // debug mode
        if key == "-d" {
            debugMode = true;
            // todo: add debug
        }
    }

    if noRun {
        log("err","Use the [-r <filename>] flag");
        logExit();
    }

    // print all flags
    if debugMode {
        log("ok","Debug mode");
    }

    logSeparator("=> Opening a file");

    // open file
    let mut file = match File::open(unsafe{&*filePath}) {
        Ok(file) => {
            log("ok",&format!("Opening the file \"{}\" was successful",unsafe{&*filePath}));
            file
        },
        Err(_) => {
            log("err",&format!("Unable to opening file \"{}\"",unsafe{&*filePath}));
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
            log("ok",&format!("Reading the file \"{}\" was successful",unsafe{&*filePath}));
        }
        Err(_) => {
            log("err",&format!("Unable to read file \"{}\"",unsafe{&*filePath}));
            logExit();
            ()
        }
    }

    logSeparator("=> AST generation");

    // read
    unsafe {
        parseLines( &mut readTokens(buffer) );
    }

    //
    Ok(())
}
