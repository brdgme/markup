use render::RenderFuncs;
use std::collections::HashMap;
use ast::Node;

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
    match n {
        &Node::Tag(_, ref args, _) => {
            if args.len() != 1 {
                return Err("fg requires one argument".to_string());
            }
            return Ok(format!(r#"<span style="color:{};">{}</span>"#, args[0], content));
        }
        _ => return Err("expected tag node".to_string()),
    }
}

fn bg(n: &Node, content: &str) -> Result<String, String> {
    match n {
        &Node::Tag(_, ref args, _) => {
            if args.len() != 1 {
                return Err("bg requires one argument".to_string());
            }
            return Ok(format!(r#"<span style="background-color:{};">{}</span>"#,
                              args[0],
                              content));
        }
        _ => return Err("expected tag node".to_string()),
    }
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

pub fn render(input: &Vec<Node>) -> Result<String, String> {
    super::render(input, &render_funcs(), Some(&render_text))
}
