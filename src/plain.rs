use transform;
use ast::Node;
use error::MarkupError;

pub fn render(input: &[Node], players: &[String]) -> Result<String, MarkupError> {
    transform::transform(input, players)
        .map_err(From::from)
        .and_then(|nodes| render_transformed(&nodes))
}

fn render_transformed(input: &[Node]) -> Result<String, MarkupError> {
    let mut buf = String::new();
    for n in input {
        match *n {
            Node::Text(ref t) => buf.push_str(t),
            Node::Fg(_, ref children) => {
                buf.push_str(&try!(render_transformed(children)));
            }
            Node::Bg(_, ref children) => {
                buf.push_str(&try!(render_transformed(children)));
            }
            Node::Bold(ref children) => {
                buf.push_str(&try!(render_transformed(children)));
            }
            _ => return Err(MarkupError::Render(format!("unknown node {:?}", n))),
        }
    }
    Ok(buf)
}
