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

pub fn render(input: &[Node], players: &[String]) -> String {
    format!("<div style=\"\
background-color:#ffffff;\
color:#000000;\
white-space:pre-wrap;\
font-family:monospace;\
\">{}</div>",
            render_nodes(&transform::transform(input, players)))
}

fn render_nodes(input: &[Node]) -> String {
    let mut buf = String::new();
    for n in input {
        match *n {
            Node::Text(ref t) => buf.push_str(&escape(t)),
            Node::Fg(ref color, ref children) => buf.push_str(&fg(color, &render_nodes(children))),
            Node::Bg(ref color, ref children) => buf.push_str(&bg(color, &render_nodes(children))),
            Node::Bold(ref children) => buf.push_str(&b(&render_nodes(children))),
            Node::Action(_, _) |
            Node::Player(_) |
            Node::Table(_) |
            Node::Align(_, _, _) |
            Node::Group(_) |
            Node::Indent(_, _) |
            Node::Canvas(_) => panic!("found untransformed node"),
        }
    }
    buf
}

