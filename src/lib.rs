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

#[cfg(test)]
mod tests {
    use super::{html, ansi, transform};
    use ast::Node as N;

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
}
