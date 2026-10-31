mod traits;
mod lexer;
mod parser;
mod scope;
mod codegen;
mod ast;

use std::{env, fs, path::Path, process};

use crate::lexer::Lexer;
use crate::parser::Parser;

fn main() {
    // Exactly one argument: the input file
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: lolcompiler <input.lol>");
        process::exit(1);
    }
    let path = &args[1];

    // The assignment requires a .lol extension
    let has_lol_ext = Path::new(path)
        .extension()
        .map_or(false, |e| e.eq_ignore_ascii_case("lol"));
    if !has_lol_ext {
        eprintln!("Usage error: input file must have a .lol extension");
        process::exit(1);
    }

    let source = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Could not read '{}': {}", path, e);
            process::exit(1);
        }
    };

    // Phase 1: lexical analysis (exits on a lexical error)
    let mut lexer = Lexer::new(&source);
    lexer.tokenize();

    // Phase 2: syntax analysis (exits on a syntax error)
    let mut parser = Parser::new(&mut lexer);
    parser.parse();

    println!("OK: '{}' lexed and parsed successfully", path);
}