use render::{transform, RenderFuncs};
use std::collections::HashMap;
use ast::Node;
use brdgme_color::Style;

pub fn render_funcs() -> RenderFuncs {
    let mut funcs: RenderFuncs = HashMap::new();
    funcs.insert("style".to_string(), Box::new(style));
    funcs
}

fn style(_n: &Node, _content: &str) -> Result<String, String> {
    Err("not implemented".to_string())
}

pub fn render(input: &Vec<Node>, players: Vec<String>) -> Result<String, String> {
    let default_style = Style::default();
    transform::transform(input, &transform::default_transforms(players))
        .and_then(|nodes| render_styled(&nodes, default_style))
        .map(|output| {
            format!(
                "{}{}",
                default_style.ansi(),
                output,
            )
        })
}

fn render_styled(input: &Vec<Node>, last_style: Style) -> Result<String, String> {
    let mut buf = String::new();
    for n in input {
        match n {
            &Node::Text(ref t) => buf.push_str(t),
            &Node::Tag(ref name, _, ref children) => {
                match name.as_str() {
                    "fg" => {
                        let new_style =
                            Style { fg: try!(super::parse_color_node(n)), ..last_style };
                        buf.push_str(&new_style.ansi());
                        buf.push_str(&try!(render_styled(children, new_style)));
                        buf.push_str(&last_style.ansi());
                    }
                    "bg" => {
                        let new_style =
                            Style { bg: try!(super::parse_color_node(n)), ..last_style };
                        buf.push_str(&new_style.ansi());
                        buf.push_str(&try!(render_styled(children, new_style)));
                        buf.push_str(&last_style.ansi());
                    }
                    "b" => {
                        let new_style = Style { bold: true, ..last_style };
                        buf.push_str(&new_style.ansi());
                        buf.push_str(&try!(render_styled(children, new_style)));
                        buf.push_str(&last_style.ansi());
                    }
                    _ => return Err(format!("unknown tag {}", name)),
                }
            }
        }
    }
    Ok(buf)
}
