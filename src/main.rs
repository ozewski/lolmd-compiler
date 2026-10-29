
mod traits;
mod lexer;
mod parser;
mod scope;
mod codegen;

use crate::lexer::Lexer;

/// Tokenize a snippet and print every token until the lexer runs dry.
fn show_tokens(label: &str, source: &str) {
    println!("--- {} ---", label);
    let mut lexer = Lexer::new(source);
    lexer.tokenize();

    loop {
        let tok = lexer.next();
        if tok.is_empty() {
            break; // empty string means end of input
        }
        println!("  [{}]", tok);
    }
    println!();
}

fn main() {
    // 1) Mixed-case keywords should come out lowercase
    show_tokens("case normalization", "#HaI #KBYE");

    // 2) Text keeps its capitalization, even words that look like keywords
    show_tokens("text preserved", "The Bold move, Simpsons!");

    // 3) A small realistic snippet
    show_tokens(
        "mini document",
        "#HAI\n#MAEK PARAGRAF\nHello World #GIMMEH BOLD Homer #OIC\n#MKAY\n#KBYE",
    );

    // 4) Whitespace handling: tabs, blank lines, extra spaces
    show_tokens("whitespace", "#hai\n\n\t  #newline   \n  #kbye");

    // 5) Error cases exit the process, so run them one at a time.
    //    Uncomment one, run, and check the message.
    // show_tokens("bad tag", "#HAI #HEY #KBYE");
    // show_tokens("bad character", "#HAI Hello@there #KBYE");
}

