use crate::traits::LexicalAnalyzer;

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    current_build: String,
    tokens: Vec<String>,
    pub articles: Vec<String>,
    pub verbs: Vec<String>,
    pub nouns: Vec<String>,
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
        self.articles.iter().any(|a| a == word)
            || self.nouns.iter().any(|n| n == word)
            || self.verbs.iter().any(|v| v == word)
    }
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let mut lexer = Self {
            input: input.chars().collect(),
            position: 0,
            current_build: String::new(),
            tokens: Vec::new(),
            articles: vec!["a".into(), "teh".into()],
            verbs: vec!["lovez".into(), "hatez".into(), "ates".into()],
            nouns: vec!["dawg".into(), "kat".into(), "rat".into()],
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
                "A lexical error was encountered. '{}' is not a recognized token.",
                candidate_token
            );
            std::process::exit(1);
        }
    }

    pub fn is_a_article(&self, word: &str) -> bool {
        self.articles.iter().any(|a| a == word)
    }

    pub fn is_a_noun(&self, word: &str) -> bool {
        self.nouns.iter().any(|n| n == word)
    }

    pub fn is_a_verb(&self, word: &str) -> bool {
        self.verbs.iter().any(|v| v == word)
    }
}