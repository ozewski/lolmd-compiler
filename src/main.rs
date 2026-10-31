mod traits;
mod lexer;
mod parser;
mod scope;
mod codegen;
mod ast;

use std::{env, fs, path::Path, process};

use crate::codegen::{open_in_browser, output_path, write_html};
use crate::parser::Parser;
use crate::scope::ScopeAnalyzer;
use crate::traits::Compiler;

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

    let mut parser = Parser::new();
    parser.compile(&source);
    let nodes = parser.take_nodes();

    let scoper = ScopeAnalyzer::new();
    let resolved = scoper.resolve(nodes);

    let path = &output_path(Path::new(path));

    write_html(&resolved, path);
    open_in_browser(path);

    println!("Compilation successful. Output written to '{}'.", path.display());
}