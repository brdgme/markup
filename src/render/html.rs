use std::collections::HashMap;

use error::MarkupError;
use render::{transform, RenderFuncs};
use ast::Node;
use parser::markup;

pub fn render_funcs() -> RenderFuncs {
    let mut funcs: RenderFuncs = HashMap::new();
    funcs.insert("fg".to_string(), Box::new(fg));
    funcs.insert("bg".to_string(), Box::new(bg));
    funcs.insert("b".to_string(), Box::new(b));
    funcs
}

pub fn render_text(t: &str) -> Result<String, String> {
    Ok(escape(t))
}

fn fg(n: &Node, content: &str) -> Result<String, String> {
    return Ok(format!(r#"<span style="color:{};">{}</span>"#,
                      try!(super::parse_color_node(n)),
                      content));
}

fn bg(n: &Node, content: &str) -> Result<String, String> {
    return Ok(format!(r#"<span style="background-color:{};">{}</span>"#,
                      try!(super::parse_color_node(n)),
                      content));
}

fn b(n: &Node, content: &str) -> Result<String, String> {
    match n {
        &Node::Tag(_, _, _) => {
            return Ok(format!(r#"<b>{}</b>"#, content));
        }
        _ => return Err("expected tag node".to_string()),
    }
}

fn escape(input: &str) -> String {
    input.replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
}

pub fn render_nodes(input: &Vec<Node>, players: Vec<String>) -> Result<String, String> {
    transform::transform(input, &transform::default_transforms(players))
        .and_then(|nodes| super::render(&nodes, &render_funcs(), Some(&render_text)))
}

pub fn render(input: &str, players: Vec<String>) -> Result<String, MarkupError> {
    render_nodes(&try!(markup(input)), players)
        .map_err(|err| From::from(err))
        .and_then(|content| {
            Ok(format!(r#"<div style="background-color:#ffffff;color:#000000;white-space:pre-wrap;font-family:monospace;">{}</div>"#,
                       content))
        })

}
