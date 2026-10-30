use crate::traits::LexicalAnalyzer;

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    current_build: String,
    tokens: Vec<String>,
    keywords: Vec<String>,
}

impl LexicalAnalyzer for Lexer {
    fn get_char(&mut self) -> char {
        if self.position < self.input.len() {
            let c = self.input[self.position];
            self.position += 1;
            c
        } else {
            eprintln!(
                "Unexpected error: attempted to read past the end of the input.",
            );
            std::process::exit(1);
        }
    }

    fn add_char(&mut self, c: char) {
        self.current_build.push(c);
    }

    fn lookup(&self, word: &str) -> bool {
        let lower = word.to_lowercase();
        self.keywords.iter().any(|k| k == &lower) 
            || lower.chars().all(|c| Lexer::is_allowed(c))
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
                "#oic".into(),
                "#mkay".into(),
                "#newline".into(),
                "#ihaz".into(),
                "#itiz".into(),
                // maek and gimmeh are two-word tokens
                "#maek head".into(), 
                "#maek paragraf".into(), 
                "#maek list".into(),
                "#gimmeh title".into(), 
                "#gimmeh bold".into(), 
                "#gimmeh item".into(),
                "#gimmeh linx".into(), 
                "#gimmeh var".into(),

                // with the two-word versions, the single-word versions should never be accepted
                // "#maek".into(),
                // "#gimmeh".into(),
            ],
        };

        lexer
    }

    pub fn finish_token(&mut self) {
        if self.current_build.is_empty() { return; }
        let mut word = std::mem::take(&mut self.current_build);
        let mut lowered = word.to_lowercase();

        if word.starts_with('#') {
            if lowered == "#maek" || lowered == "#gimmeh" {
                // these are two-word keywords, so we need to look ahead
                self.read_word();
                word = format!("{} {}", word, std::mem::take(&mut self.current_build));
                lowered = word.to_lowercase();
            }

            if !self.lookup(&lowered) {
                eprintln!(
                    "Lexical error: '{}' is not a recognized tag.",
                    word
                );
                std::process::exit(1);
            }
        } else {
            if !self.lookup(&word) {
                eprintln!(
                    "Lexical error: '{}' is not a recognized token.",
                    word
                );
                std::process::exit(1);
            }
        }

        self.tokens.push(word);
    }

    fn read_word(&mut self)  {
        while self.position < self.input.len() {
            let c = self.get_char();
            if c.is_whitespace() {
                if !self.current_build.is_empty() {
                    break; // word finished
                }
            } else {
                self.add_char(c);
            }
        }
    }

    pub fn tokenize(&mut self) {
        loop {
            self.read_word();
            if self.current_build.is_empty() {
                break; // no more input
            }
            self.finish_token();
        }
        self.tokens.reverse();
    }

    pub fn next(&mut self) -> String {
        let candidate_token = self.tokens.pop().unwrap_or_default();
        // at this point, everything has already been validated by the lexer, so we can just return it
        candidate_token
    }

    pub fn is_allowed(c: char) -> bool {
        c.is_ascii_alphanumeric() 
        || c.is_whitespace()
        || c == ',' 
        || c == '.' 
        || c == '?' 
        || c == '!' 
        || c == '%' 
        || c == '=' 
        || c == '/'
        || c == '"'
        || c == ':'
    }

    pub fn is_text(&self, token: &str) -> bool {
        !token.is_empty() && !token.starts_with('#')
    }

    pub fn is_varname(&self, token: &str) -> bool {
        !token.is_empty() && token.chars().all(|c| c.is_ascii_alphabetic())
    }
}