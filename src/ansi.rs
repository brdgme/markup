use transform;
use ast::Node;
use brdgme_color::Style;

pub fn render(input: &[Node], players: &[String]) -> String {
    let default_style = Style::default();
    format!("{}{}",
            default_style.ansi(),
            render_styled(&transform::transform(input, players), default_style))
}

fn render_styled(input: &[Node], last_style: Style) -> String {
    let mut buf = String::new();
    for n in input {
        match *n {
            Node::Text(ref t) => buf.push_str(t),
            Node::Fg(ref color, ref children) => {
                let new_style = Style { fg: color, ..last_style };
                buf.push_str(&new_style.ansi());
                buf.push_str(&render_styled(children, new_style));
                buf.push_str(&last_style.ansi());
            }
            Node::Bg(ref color, ref children) => {
                let new_style = Style { bg: color, ..last_style };
                buf.push_str(&new_style.ansi());
                buf.push_str(&render_styled(children, new_style));
                buf.push_str(&last_style.ansi());
            }
            Node::Bold(ref children) => {
                let new_style = Style { bold: true, ..last_style };
                buf.push_str(&new_style.ansi());
                buf.push_str(&render_styled(children, new_style));
                buf.push_str(&last_style.ansi());
            }
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

