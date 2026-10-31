use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::ast::Node;

/// Translates scope-resolved nodes into an HTML string.
/// Exits if a VarDef or VarUse is still present, since the scope
/// analyzer should have removed or replaced every one of them.
pub fn generate_html(nodes: &[Node]) -> String {
    let mut html = String::new();

    for (i, node) in nodes.iter().enumerate() {
        match node {
            Node::DocOpen => html.push_str("<html>"),
            Node::DocClose => html.push_str("</html>"),
            Node::HeadOpen => html.push_str("<head>"),
            Node::HeadClose => html.push_str("</head>"),
            Node::TitleOpen => html.push_str("<title>"),
            Node::TitleClose => html.push_str("</title>"),
            Node::CommentOpen => html.push_str("<!--"),
            Node::CommentClose => html.push_str("-->"),
            Node::ParagraphOpen => html.push_str("<p>"),
            Node::ParagraphClose => html.push_str("</p>"),
            Node::ListOpen => html.push_str("<ul>"),
            Node::ListClose => html.push_str("</ul>"),
            Node::ItemOpen => html.push_str("<li>"),
            Node::ItemClose => html.push_str("</li>"),
            Node::BoldOpen => html.push_str("<b>"),
            Node::BoldClose => html.push_str("</b>"),
            Node::Newline => html.push_str("<br>"),

            // The address is the Text node right after LinkOpen. It is used
            // here for href, and the Text arm below prints it as the visible text.
            Node::LinkOpen => match nodes.get(i + 1) {
                Some(Node::Text(address)) => {
                    html.push_str(&format!("<a href=\"{}\">", address));
                }
                _ => {
                    eprintln!("Code generation error: link has no address");
                    std::process::exit(1);
                }
            },
            Node::LinkClose => html.push_str("</a>"),

            Node::Text(t) => html.push_str(t),

            // These must be gone after the scope pass
            Node::VarUse(name) => {
                eprintln!(
                    "Code generation error: unresolved variable use '{}' reached code generation",
                    name
                );
                std::process::exit(1);
            }
            Node::VarDef { name, .. } => {
                eprintln!(
                    "Code generation error: variable definition '{}' reached code generation",
                    name
                );
                std::process::exit(1);
            }
        }
        // Placeholder spacing: one fragment per line, so adjacent nodes
        // never run together. Real formatting comes in a later step.
        html.push('\n');
    }

    html
}

/// Same directory and file name as the input, with an .html extension.
pub fn output_path(input: &Path) -> PathBuf {
    input.with_extension("html")
}

/// Generates the HTML and writes it to `path`. The HTML is fully built
/// before the file is touched, so an error never leaves a partial file.
pub fn write_html(nodes: &[Node], path: &Path) {
    let html = generate_html(nodes);
    if let Err(e) = fs::write(path, html) {
        eprintln!("Code generation error: could not write '{}': {}", path.display(), e);
        std::process::exit(1);
    }
}

/// Opens the file in the default browser (Windows or macOS per the
/// assignment, with a Linux fallback).
pub fn open_in_browser(path: &Path) {
    let result = if cfg!(target_os = "windows") {
        // the empty "" is the window title argument that `start` expects
        Command::new("cmd").args(["/C", "start", ""]).arg(path).spawn()
    } else if cfg!(target_os = "macos") {
        Command::new("open").arg(path).spawn()
    } else {
        Command::new("xdg-open").arg(path).spawn()
    };

    // The file is already written, so a missing browser is a warning only.
    if let Err(e) = result {
        eprintln!("Warning: could not open browser: {}", e);
    }
}
