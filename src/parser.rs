use crate::{ast::Node, lexer::Lexer, traits::{Compiler, SyntaxAnalyzer}};

pub struct Parser {
    lexer: Lexer,
    current_token: String,
    nodes: Vec<Node>,
}

impl Compiler for Parser {
    fn compile(&mut self, source: &str) {
        self.lexer = Lexer::new(source);
        self.lexer.tokenize();
        self.nodes.clear();
        let first = self.lexer.next();
        self.set_current_token(first);

        self.parse();
    }

    fn next_token(&mut self) -> String {
        self.current_token = self.lexer.next();
        self.current_token.clone()
    }

    fn parse(&mut self) { self.parse_lolcode(); }
    fn current_token(&self) -> String { self.current_token.clone() }
    fn set_current_token(&mut self, tok: String) { self.current_token = tok; }
}


impl Parser {
    pub fn new() -> Self {
        Self {
            lexer: Lexer::new(""),
            current_token: String::new(),
            nodes: Vec::new(),
        }
    }

    pub fn take_nodes(&mut self) -> Vec<Node> {
        std::mem::take(&mut self.nodes)
    }

    fn error(&self, message: String) -> ! {
        eprintln!("Syntax error: {}", message);
        std::process::exit(1);
    }

    fn expected(&self, expected: &str) -> ! {
        self.error(format!(
            "Expected '{}', found '{}'",
            expected, self.current_token
        ));
    }

    fn is(&self, kw: &str) -> bool {
        // case-insensitive checker
        self.current_token().eq_ignore_ascii_case(kw)
    }

    fn read_text(&mut self) -> String {
        if !self.lexer.is_text(&self.current_token) {
            self.error(format!("Expected text, found '{}'", self.current_token));
        }
        let mut words: Vec<String> = Vec::new();
        while self.lexer.is_text(&self.current_token) {
            words.push(self.current_token.clone());
            self.next_token();
        }
        words.join(" ")
    }

    // a few of the rules from the BNF were not in the trait.
    // these are those extra rules

    fn parse_optional_variable_define(&mut self) {
        if self.is("#ihaz") {
            self.parse_variable_define();
        }
    }

    fn parse_comments(&mut self) {
        if self.is("#obtw") {
            self.parse_comment();
            self.parse_comments();
        }
    }

    fn parse_inner_body(&mut self) {
        if self.is("#maek paragraf") {
            self.parse_paragraph();
        } else if self.is("#maek list") {
            self.parse_list();
        } else if self.is("#obtw") {
            self.parse_comment();
        } else if self.is("#gimmeh bold") {
            self.parse_bold();
        } else if self.is("#gimmeh linx") {
            self.parse_link();
        } else if self.is("#gimmeh var") {
            self.parse_variable_use();
        } else if self.is("#newline") {
            self.parse_newline();
        } else if self.lexer.is_text(&self.current_token) {
            self.parse_text();
        } else {
            self.error(format!("Unexpected token '{}'", self.current_token));
        }
    }

    fn parse_list_content(&mut self) {
        if self.is("#obtw") {
            self.parse_comment();
        } else if self.is("#gimmeh bold") {
            self.parse_bold();
        } else if self.is("#gimmeh linx") {
            self.parse_link();
        } else if self.is("#gimmeh var") {
            self.parse_variable_use();
        } else if self.is("#newline") {
            self.parse_newline();
        } else if self.lexer.is_text(&self.current_token) {
            self.parse_text();
        } else {
            self.error(format!("Unexpected token '{}'", self.current_token));
        }
    }

    fn parse_list_item(&mut self) {
        if self.is("#gimmeh item") {
            self.next_token();
            self.nodes.push(Node::ItemOpen);
            self.parse_optional_variable_define();
            self.parse_inner_list();
            if self.is("#oic") {
                self.next_token();
                self.nodes.push(Node::ItemClose);
            } else {
                self.expected("#OIC");
            }
        } else {
            self.expected("#GIMMEH ITEM");
        }
    }
}

impl SyntaxAnalyzer for Parser {
    fn parse_lolcode(&mut self) {
        if self.is("#hai") {
            self.next_token();
            self.nodes.push(Node::DocOpen);
            self.parse_comments();
            self.parse_head();
            self.parse_optional_variable_define();
            self.parse_body();
            if self.is("#kbye") {
                self.next_token();
                self.nodes.push(Node::DocClose);
                if self.current_token.is_empty() {
                    return;
                } else {
                    self.error(format!("Unexpected '{}' after '#KBYE'", self.current_token));
                }
            } else {
                self.expected("#KBYE");
            }
        } else {
            self.expected("#HAI");
        }
    }

    fn parse_head(&mut self) {
        if self.is("#maek head"){
            self.next_token();
            self.nodes.push(Node::HeadOpen);
            self.parse_title();
            if self.is("#mkay") {
                self.next_token();
                self.nodes.push(Node::HeadClose);
            } else {
                self.expected("#MKAY");
            }
        } else {
            // head is optional
        }
    }

    fn parse_title(&mut self) {
        if self.is("#gimmeh title") {
            self.next_token();
            self.nodes.push(Node::TitleOpen);
            self.parse_text();
            if self.is("#oic") {
                self.next_token();
                self.nodes.push(Node::TitleClose);
            } else {
                self.expected("#OIC");
            }
        } else {
            self.expected("#GIMMEH TITLE");
        }
    }

    fn parse_comment(&mut self) {
        if self.is("#obtw") {
            self.next_token();
            self.nodes.push(Node::CommentOpen);
            self.parse_text();
            if self.is("#tldr") {
                self.next_token();
                self.nodes.push(Node::CommentClose);
            } else {
                self.expected("#TLDR");
            }
        } else {
            self.expected("#OBTW");
        }
    }

    fn parse_body(&mut self) {
        if self.is("#maek paragraf")
            || self.is("#maek list")
            || self.is("#obtw")
            || self.is("#gimmeh bold")
            || self.is("#gimmeh linx")
            || self.is("#gimmeh var")
            || self.is("#newline")
            || self.lexer.is_text(&self.current_token)
        {
            self.parse_inner_body();
            self.parse_body();
        }
    }

    fn parse_paragraph(&mut self) {
        if self.is("#maek paragraf") {
            self.next_token();
            self.nodes.push(Node::ParagraphOpen);
            self.parse_optional_variable_define();
            self.parse_inner_paragraph();
            if self.is("#mkay") {
                self.next_token();
                self.nodes.push(Node::ParagraphClose);
            } else {
                self.expected("#MKAY");
            }
        } else {
            self.expected("#MAEK PARAGRAF");
        }
    }

    fn parse_inner_paragraph(&mut self) {
        if self.is("#maek list")
            || self.is("#obtw")
            || self.is("#gimmeh bold")
            || self.is("#gimmeh linx")
            || self.is("#gimmeh var")
            || self.is("#newline")
            || self.lexer.is_text(&self.current_token)
            // same as the body, EXCEPT minus #MAEK PARAGRAF (paragraphs can't nest)
        {
            self.parse_inner_text();
            self.parse_inner_paragraph();
        }
    }

    fn parse_inner_text(&mut self) {
        if self.is("#maek list") {
            self.parse_list();
        } else if self.is("#obtw") {
            self.parse_comment();
        } else if self.is("#gimmeh bold") {
            self.parse_bold();
        } else if self.is("#gimmeh linx") {
            self.parse_link();
        } else if self.is("#gimmeh var") {
            self.parse_variable_use();
        } else if self.is("#newline") {
            self.parse_newline();
        } else if self.lexer.is_text(&self.current_token) {
            self.parse_text();
        } else {
            self.error(format!("Unexpected token '{}'", self.current_token));
        }
    }

    fn parse_variable_define(&mut self) {
        if self.is("#ihaz") {
            self.next_token();
            if self.lexer.is_varname(&self.current_token) {
                let var_name = self.current_token.clone();
                self.next_token();
                if self.is("#itiz") {
                    self.next_token();
                    let var_value = self.read_text();
                    if self.is("#mkay") {
                        self.next_token();
                        self.nodes.push(Node::VarDef { name: var_name, value: var_value });
                    } else {
                        self.expected("#MKAY");
                    }
                } else {
                    self.expected("#ITIZ");
                }
            } else {
                self.error(format!("Expected variable name, found '{}'", self.current_token));
            }
        } else {
            self.expected("#IHAZ");
        }
    }

    fn parse_variable_use(&mut self) {
        if self.is("#gimmeh var") {
            self.next_token();
            if self.lexer.is_varname(&self.current_token) {
                let varname = self.current_token.clone();
                self.next_token();
                if self.is("#oic") {
                    self.next_token();
                    self.nodes.push(Node::VarUse(varname));
                } else {
                    self.expected("#OIC");
                }
            } else {
                self.error(format!("Expected variable, found '{}'", self.current_token));
            }
        } else {
            self.expected("#GIMMEH VAR");
        }
    }

    fn parse_bold(&mut self) {
        if self.is("#gimmeh bold") {
            self.next_token();
            self.nodes.push(Node::BoldOpen);
            self.parse_text();
            if self.is("#oic") {
                self.next_token();
                self.nodes.push(Node::BoldClose);
            } else {
                self.expected("#OIC");
            }
        } else {
            self.expected("#GIMMEH BOLD");
        }
    }

    fn parse_list(&mut self) {
        if self.is("#maek list") {
            self.next_token();
            self.nodes.push(Node::ListOpen);
            self.parse_list_item(); // at least one list item is required
            self.parse_list_items();
            if self.is("#mkay") {
                self.next_token();
                self.nodes.push(Node::ListClose);
            } else {
                self.expected("#MKAY");
            }
        } else {
            self.expected("#MAEK LIST");
        }
    }

    fn parse_list_items(&mut self) {
        if self.is("#gimmeh item") {
            self.parse_list_item();
            self.parse_list_items();
        }
    }

    fn parse_inner_list(&mut self) {
        if self.is("#obtw")
            || self.is("#gimmeh bold")
            || self.is("#gimmeh linx")
            || self.is("#gimmeh var")
            || self.is("#newline")
            || self.lexer.is_text(&self.current_token)
        {
            self.parse_list_content();
            self.parse_inner_list();
        }
    }

    fn parse_link(&mut self) {
        if self.is("#gimmeh linx") {
            self.nodes.push(Node::LinkOpen);
            self.next_token();
            if self.lexer.is_text(&self.current_token) {
                self.nodes.push(Node::Text(self.current_token.clone()));
                self.next_token();
                if self.is("#oic") {
                    self.next_token();
                    self.nodes.push(Node::LinkClose);
                } else {
                    self.expected("#OIC");
                }
            } else {
                self.error(format!("Expected link address, found '{}'", self.current_token));
            }
        } else {
            self.expected("#GIMMEH LINX");
        }
    }

    fn parse_newline(&mut self) {
        if self.is("#newline") {
            self.next_token();
            self.nodes.push(Node::Newline);
        } else {
            self.expected("#NEWLINE");
        }
    }

    fn parse_text(&mut self) {
        let text = self.read_text();
        self.nodes.push(Node::Text(text));
    }
}
