use error::MarkupError;
use transform;
use ast::Node;
use brdgme_color::Color;

fn fg(color: &Color, content: &str) -> String {
    return format!(r#"<span style="color:{};">{}</span>"#, color, content);
}

fn bg(color: &Color, content: &str) -> String {
    return format!(r#"<span style="background-color:{};">{}</span>"#,
                   color,
                   content);
}

fn b(content: &str) -> String {
    return format!("<b>{}</b>", content);
}

fn escape(input: &str) -> String {
    input.replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
}

pub fn render(input: &Vec<Node>, players: &Vec<&str>) -> Result<String, MarkupError> {
    transform::transform(input, players)
        .map_err(|err| From::from(err))
        .and_then(|nodes| render_nodes(&nodes))
        .map(|output| {
            format!(
                "<div style=\"\
background-color:#ffffff;\
color:#000000;\
white-space:pre-wrap;\
font-family:monospace;\
\">{}</div>",
                output,
            )
        })
}

fn render_nodes(input: &Vec<Node>) -> Result<String, MarkupError> {
    let mut buf = String::new();
    for n in input {
        match n {
            &Node::Text(ref t) => buf.push_str(&escape(t)),
            &Node::Fg(ref color, ref children) => {
                buf.push_str(&fg(color, &try!(render_nodes(children))))
            }
            &Node::Bg(ref color, ref children) => {
                buf.push_str(&bg(color, &try!(render_nodes(children))))
            }
            &Node::Bold(ref children) => buf.push_str(&b(&try!(render_nodes(children)))),
            _ => return Err(MarkupError::Render(format!("unknown node {:?}", n))),
        }
    }
    Ok(buf)
}
