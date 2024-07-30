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
                let exprs = parser.parse().unwrap();

                for expr in exprs {
                    println!("{}", expr);
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
