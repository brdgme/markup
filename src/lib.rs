extern crate brdgme_color;

mod ast;
mod transform;
mod ansi;
mod html;
mod plain;

pub use transform::{transform, from_lines, to_lines};
pub use ast::{Node, TNode, Align, Row};

pub fn html(input: &[TNode]) -> String {
    html::render(input)
}

pub fn ansi(input: &[TNode]) -> String {
    ansi::render(input)
}

pub fn plain(input: &[TNode]) -> String {
    plain::render(input)
}

pub fn to_string(input: &[Node]) -> String {
    input.iter()
        .map(|n| match *n {
                 Node::Text(ref t) => t.to_owned(),
                 Node::Bold(ref children) => format!("{{{{b}}}}{}{{{{/b}}}}", to_string(children)),
                 Node::Fg(c, ref children) => {
                     format!("{{{{fg {} {} {}}}}}{}{{{{/fg}}}}",
                             c.r,
                             c.g,
                             c.b,
                             to_string(children))
                 }
                 Node::Bg(c, ref children) => {
                     format!("{{{{bg {} {} {}}}}}{}{{{{/bg}}}}",
                             c.r,
                             c.g,
                             c.b,
                             to_string(children))
                 }
                 Node::Player(p) => format!("{{{{p {}}}}}", p),
                 Node::Table(ref rows) => {
                     format!("{{{{table}}}}{}{{{{/table}}}}",
                             rows.iter()
                                 .map(|r| {
            format!("{{{{row}}}}{}{{{{/row}}}}",
                    r.iter()
                        .map(|&(ref align, ref children)| {
                                 format!("{{{{cell {}}}}}{}{{{{/cell}}}}",
                                         align.to_string(),
                                         to_string(children))
                             })
                        .collect::<Vec<String>>()
                        .join(""))
        })
                                 .collect::<Vec<String>>()
                                 .join(""))
                 }
                 Node::Align(ref al, width, ref children) => {
                     format!("{{{{align {} {}}}}}{}{{{{/align}}}}",
                             al.to_string(),
                             width,
                             to_string(children))
                 }
                 Node::Indent(width, ref children) => {
                     format!("{{{{indent {}}}}}{}{{{{/indent}}}}",
                             width,
                             to_string(children))
                 }
                 Node::Canvas(ref layers) => {
                     format!("{{{{canvas}}}}{}{{{{/canvas}}}}",
                             layers.iter()
                                 .map(|&(x, y, ref children)| {
                                          format!("{{{{layer {} {}}}}}{}{{{{/layer}}}}",
                                                  x,
                                                  y,
                                                  to_string(children))
                                      })
                                 .collect::<Vec<String>>()
                                 .join(""))
                 }
             })
        .collect::<Vec<String>>()
        .join("")
}

#[cfg(test)]
mod tests {
    use super::*;
    use brdgme_color::*;
    use ast::{Node as N, Align as A};

    #[test]
    fn ansi_works() {
        ansi(&transform(&[N::text("Here is "),
                          N::Bold(vec![N::text("something")]),
                          N::text(" for "),
                          N::Player(0),
                          N::text(" and "),
                          N::Player(1)],
                        &vec!["mick".to_string(), "steve".to_string()]));
    }

    #[test]
    fn html_works() {
        html(&transform(&[N::text("Here is "),
                          N::Bold(vec![N::text("something")]),
                          N::text(" for "),
                          N::Player(0),
                          N::text(" and "),
                          N::Player(1)],
                        &vec!["mick".to_string(), "steve".to_string()]));
    }

    #[test]
    fn to_string_works() {
        println!("{}",
                 to_string(&[N::Canvas(vec![(5,
                                             10,
                                             vec![
                     N::Table(vec![vec![(A::Center,
                                                 vec![N::Fg(AMBER,
                                   vec![N::Bg(BLUE, vec![N::Bold(vec![N::text("moo")])])])])]])
             ])])]));
    }
}
