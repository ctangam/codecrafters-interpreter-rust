use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::exit;

use parser::Parser;
pub mod token;
pub mod expr;
pub mod parser;
pub mod lexer;
pub mod printer;
pub mod eval;
pub mod stmt;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });

            if !file_contents.is_empty() {
                let (tokens, code) = lexer::scan(file_contents);
                for token in tokens {
                    println!("{}", token);
                }
                if code != 0 {
                    exit(code);
                }
            } else {
                println!("EOF  null");
            }
        }
        "parse" => {
            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });
            if !file_contents.is_empty() {
                let (tokens, code) = lexer::scan(file_contents);
                if code != 0 {
                    exit(code);
                }
                let mut parser = Parser::new(tokens);
                let exprs = parser.parse();

                match exprs {
                    Ok(exprs) => {
                        for expr in exprs {
                            println!("{}", expr);
                        }
                    }
                    Err(errors) => {
                        for error in errors {
                            writeln!(io::stderr(), "{}", error).unwrap();
                        }
                        exit(65);
                    }
                }
            }
        }
        "evaluate" => {
            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });
            if !file_contents.is_empty() {
                let (tokens, code) = lexer::scan(file_contents);
                if code != 0 {
                    exit(code);
                }
                let mut parser = Parser::new(tokens);
                let exprs = parser.parse();

                match exprs {
                    Ok(exprs) => {
                        let interpreter = eval::Interpreter::new();
                        let values = interpreter.interpret(exprs);
                        match values {
                            Ok(values) => {
                                for value in values {
                                    println!("{}", value);
                                }
                            }
                            Err(errors) => {
                                for error in errors {
                                    writeln!(io::stderr(), "{}", error).unwrap();
                                }
                                exit(70);
                            }
                        }
                    }
                    Err(errors) => {
                        for error in errors {
                            writeln!(io::stderr(), "{}", error).unwrap();
                        }
                        exit(65);
                    }
                }

            }
        }
        "run" => {
            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });
            if !file_contents.is_empty() {
                let (tokens, code) = lexer::scan(file_contents);
                if code != 0 {
                    exit(code);
                }
                let mut parser = Parser::new(tokens);
                let exprs = parser.parse2();

                match exprs {
                    Ok(stmts) => {
                        let interpreter = eval::Interpreter::new();
                        let values = interpreter.execute(stmts);
                        match values {
                            Ok(_) => (),
                            Err(errors) => {
                                for error in errors {
                                    writeln!(io::stderr(), "{}", error).unwrap();
                                }
                                exit(70);
                            }
                        }
                    }
                    Err(errors) => {
                        for error in errors {
                            writeln!(io::stderr(), "{}", error).unwrap();
                        }
                        exit(65);
                    }
                }

            }
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}

pub trait Walkable<V, T> {
    fn walk(&self, visitor: &V) -> T;
}
