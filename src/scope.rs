use crate::ast::Node;

pub struct ScopeAnalyzer {
    defs: Vec<(String, String)>, // live definitions, most recent last
    marks: Vec<usize>,           // defs.len() at the start of each open block
    output: Vec<Node>,           // resolved nodes
}

/* 
Rough explanation of algorithm:
1. When we see a block open, we push the current length of defs onto marks.
2. When we see a block close, we pop the last mark and truncate defs to that length.
    - This clears from the stack all the variables that were defined in that block (which is now closed).
3. When we see a variable definition, we add it to defs.
4. When we see a variable use, we search defs from the end to find the most recent definition.
    - If we didn't find a definition, we report an error and exit.
*/

impl ScopeAnalyzer {
    pub fn new() -> Self {
        Self { defs: Vec::new(), marks: Vec::new(), output: Vec::new() }
    }

    pub fn resolve(mut self, nodes: Vec<Node>) -> Vec<Node> {
        for node in nodes {
            match node {
                Node::DocOpen | Node::ParagraphOpen | Node::ItemOpen => {
                    self.marks.push(self.defs.len());
                    self.output.push(node);
                }
                Node::DocClose | Node::ParagraphClose | Node::ItemClose => {
                    let mark = self.marks.pop().unwrap();
                    self.defs.truncate(mark);
                    self.output.push(node);
                }
                Node::VarDef { name, value } => {
                    self.defs.push((name, value));
                }
                Node::VarUse(name) => {
                    match self.defs.iter().rev().find(|(n, _)| *n == name) {
                        Some((_, value)) => self.output.push(Node::Text(value.clone())),
                        None => {
                            eprintln!(
                                "Static semantic error: variable '{}' used before it is defined",
                                name
                            );
                            std::process::exit(1);
                        }
                    }
                }
                other => self.output.push(other),
            }
        }
        self.output
    }
}
