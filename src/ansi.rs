use transform;
use ast::Node;
use brdgme_color::Style;
use error::MarkupError;

pub fn render(input: &Vec<Node>, players: &Vec<String>) -> Result<String, MarkupError> {
    let default_style = Style::default();
    transform::transform(input, players)
        .map_err(|err| From::from(err))
        .and_then(|nodes| render_styled(&nodes, default_style))
        .map(|output| {
            format!(
                "{}{}",
                default_style.ansi(),
                output,
            )
        })
}

fn render_styled(input: &Vec<Node>, last_style: Style) -> Result<String, MarkupError> {
    let mut buf = String::new();
    for n in input {
        match n {
            &Node::Text(ref t) => buf.push_str(t),
            &Node::Fg(ref color, ref children) => {
                let new_style = Style { fg: color, ..last_style };
                buf.push_str(&new_style.ansi());
                buf.push_str(&try!(render_styled(children, new_style)));
                buf.push_str(&last_style.ansi());
            }
            &Node::Bg(ref color, ref children) => {
                let new_style = Style { bg: color, ..last_style };
                buf.push_str(&new_style.ansi());
                buf.push_str(&try!(render_styled(children, new_style)));
                buf.push_str(&last_style.ansi());
            }
            &Node::Bold(ref children) => {
                let new_style = Style { bold: true, ..last_style };
                buf.push_str(&new_style.ansi());
                buf.push_str(&try!(render_styled(children, new_style)));
                buf.push_str(&last_style.ansi());
            }
            _ => return Err(MarkupError::Render(format!("unknown node {:?}", n))),
        }
    }
    Ok(buf)
}
