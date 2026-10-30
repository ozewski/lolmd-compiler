use std::thread::current;

use crate::{lexer::Lexer, traits::SyntaxAnalyzer};

pub struct Parser<'a> {
    lexer: &'a mut Lexer,
    current_token: String,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer) -> Self {
        let current_token = lexer.next();

        Self {
            lexer,
            current_token,
        }
    }

    pub fn next_token(&mut self) -> String {
        self.current_token = self.lexer.next();
        self.current_token.clone()
    }

    pub fn parse(&mut self) {
        self.parse_lolcode();
    }

    fn error(&self, message: String) {
        eprintln!("Syntax error: {}", message);
        std::process::exit(1);
    }

    // a few of the rules from the BNF were not in the trait.
    // these are those extra rules

    fn parse_optional_variable_define(&mut self) {

    }

    fn parse_comments(&mut self) {

    }

    fn parse_inner_body(&mut self) {

    }

    fn parse_list_content(&mut self) {

    }
}

impl SyntaxAnalyzer for Parser<'_> {
    fn parse_lolcode(&mut self) {
        if self.current_token == "#hai" {
            self.next_token();
            self.parse_comment();
            self.parse_head();
            self.parse_optional_variable_define();
            self.parse_body();
            if self.current_token == "#kbye" {
                self.next_token();
                // all we need is return here maybe? since there shouldn't be any next token
            } else {
                self.error(
                    format!("Expected '#KBYE', found '{}'", self.current_token)
                );
            }
        } else {
            self.error(
                format!("Expected '#HAI', found '{}'", self.current_token)
            );
        }
    }

    fn parse_head(&mut self) {
        if self.current_token == "#MAEK" {
            self.next_token();
            if self.current_token == "HEAD" {
                self.next_token();
                self.parse_title();
                if self.current_token == "#MKAY" {
                    self.next_token();
                } else {
                    self.error(
                        format!("Expected '#MKAY', found '{}'", self.current_token)
                    );
                }
            } else {
                self.error(
                    format!("Expected 'HEAD', found '{}'", self.current_token)
                );
            }
        } else {
            // head is optional
        }
    }

    fn parse_title(&mut self) {
        if self.current_token == "#GIMMEH" {
            self.next_token();
            if self.current_token == "TITLE" {
                self.next_token();
                self.parse_text();
                if self.current_token == "#OIC" {
                    self.next_token();
                } else {
                    self.error(
                        format!("Expected '#OIC', found '{}'", self.current_token)
                    );
                }
            } else {
                self.error(
                    format!("Expected 'TITLE', found '{}'", self.current_token)
                );
            }
        } else {
            self.error(
                format!("Expected '#GIMMEH', found '{}'", self.current_token)
            );
        }
    }

    fn parse_comment(&mut self) {
        if self.current_token == "#OBTW" {
            self.next_token();
            self.parse_text();
            if self.current_token == "#TLDR" {
                self.next_token();
            } else {
                self.error(
                    format!("Expected '#TLDR', found '{}'", self.current_token)
                );
            }
        }
    }

    fn parse_body(&mut self) {

    }

    fn parse_paragraph(&mut self) {

    }

    fn parse_inner_paragraph(&mut self) {

    }

    fn parse_inner_text(&mut self) {

    }

    fn parse_variable_define(&mut self) {

    }

    fn parse_variable_use(&mut self) {

    }

    fn parse_bold(&mut self) {

    }

    fn parse_list(&mut self) {

    }

    fn parse_list_items(&mut self) {

    }

    fn parse_inner_list(&mut self) {

    }

    fn parse_link(&mut self) {

    }

    fn parse_newline(&mut self) {

    }

    fn parse_text(&mut self) {

    }

}