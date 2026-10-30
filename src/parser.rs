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
}

impl SyntaxAnalyzer for Parser<'_> {
    fn parse_lolcode(&mut self) {

    }

    fn parse_head(&mut self) {

    }

    fn parse_title(&mut self) {

    }

    fn parse_comment(&mut self) {

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