use crate::traits::LexicalAnalyzer;

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    current_build: String,
    tokens: Vec<String>,
    pub keywords: Vec<String>,
}

impl LexicalAnalyzer for Lexer {
    fn get_char(&mut self) -> char {
        if self.position < self.input.len() {
            let c = self.input[self.position];
            self.position += 1;
            c
        } else {
            '\0'
        }
    }

    fn add_char(&mut self, c: char) {
        self.current_build.push(c);
    }

    fn lookup(&self, word: &str) -> bool {
        self.keywords.iter().any(|k| k == word)
    }
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let lexer = Self {
            input: input.chars().collect(),
            position: 0,
            current_build: String::new(),
            tokens: Vec::new(),
            keywords: vec![
                "#hai".into(), 
                "#kbye".into(),
                "#obtw".into(),
                "#tldr".into(),
                "#maek".into(),
                "#oic".into(),
                "#gimmeh".into(),
                "#mkay".into(),
                "#newline".into(),
                "#ihaz".into(),
                "#itiz".into(),
                "head".into(),
                "title".into(),
                "paragraf".into(),
                "bold".into(),
                "list".into(),
                "item".into(),
                "linx".into(),
                "var".into()
            ],
        };

        lexer
    }

    pub fn tokenize(&mut self) {
        loop {
            let c = self.get_char();

            if c == '\0' {
                break;
            }

            if c.is_whitespace() {
                if !self.current_build.is_empty() {
                    self.tokens
                        .push(std::mem::take(&mut self.current_build));
                }
            } else {
                self.add_char(c);
            }
        }

        if !self.current_build.is_empty() {
            self.tokens
                .push(std::mem::take(&mut self.current_build));
        }

        self.tokens.reverse();
    }

    pub fn next(&mut self) -> String {
        let candidate_token = self.tokens.pop().unwrap_or_default();

        if self.lookup(&candidate_token) {
            candidate_token
        } else if self.tokens.is_empty() {
            "".to_string()
        } else {
            eprintln!(
                "Lexical error: '{}' is not a recognized token.",
                candidate_token
            );
            std::process::exit(1);
        }
    }
}