use std::env::args;
//use std::f32::consts::PI;
use std::io::{self, stdout, BufRead, Write};
mod error;
use error::*;

mod token;
mod token_type;

mod scanner;

use scanner::*;

mod parser;
use parser::*;

mod expr;
mod stmt;

mod callable;
mod environment;
mod native_functions;

//mod ast_printer;
//use ast_printer::*;

mod interpreter;
use interpreter::*;
mod object;

pub fn main() {
    let args: Vec<String> = args().collect();
    //println!("args: {:?}", args);
    let lox = Lox::new();

    match args.len() {
        1 => lox.run_prompt(),
        2 => lox.run_file(&args[1]).expect("Could not run file"),
        _ => {
            println!("Usage: lox-ast [script]");
            std::process::exit(64);
        }
    }
}

struct Lox {
    interpreter: Interpreter,
}

impl Lox {
    pub fn new() -> Lox {
        Lox {
            interpreter: Interpreter::new(),
        }
    }
    pub fn run_file(&self, path: &str) -> io::Result<()> {
        let buf = std::fs::read_to_string(path)?;
        if self.run(buf).is_err() {
            // Ignore: error was already reported
            std::process::exit(65);
        }
        Ok(())
    }

    pub fn run_prompt(&self) {
        let stdin = io::stdin();
        print!("> ");
        let _ = stdout().flush();

        for line in stdin.lock().lines() {
            if let Ok(line) = line {
                if line.is_empty() {
                    break;
                }
                let _ = self.run(line);
            } else {
                break;
            }
            print!("> ");
            let _ = stdout().flush();
        }
    }

    fn run(&self, source: String) -> Result<(), LoxResult> {
        if source == "@" {
            self.interpreter.print_environment();
            return Ok(());
        }
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens()?;
        let mut parser = Parser::new(tokens);
        let statements = parser.parse()?;

        if parser.success() {
            self.interpreter.interpret(&statements);
        }
        Ok(())
    }
}
