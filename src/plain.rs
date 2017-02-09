use transform;
use ast::Node;

pub fn render(input: &[Node], players: &[String]) -> String {
    render_transformed(&transform::transform(input, players))
}

fn render_transformed(input: &[Node]) -> String {
    let mut buf = String::new();
    for n in input {
        match *n {
            Node::Text(ref t) => buf.push_str(t),
            Node::Fg(_, ref children) |
            Node::Bg(_, ref children) |
            Node::Bold(ref children) => {
                buf.push_str(&render_transformed(children));
            }
            Node::Action(_, _) |
            Node::Player(_) |
            Node::Table(_) |
            Node::Align(_, _, _) |
            Node::Group(_) |
            Node::Indent(_, _) => panic!("found untransformed node"),
        }
    }
    buf
}
