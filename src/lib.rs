extern crate brdgme_color;

pub mod ast;
mod transform;
mod ansi;
mod html;
mod plain;

use ast::Node;

pub fn html(input: &[Node], players: &[String]) -> String {
    html::render(input, players)
}

pub fn ansi(input: &[Node], players: &[String]) -> String {
    ansi::render(input, players)
}

pub fn plain(input: &[Node], players: &[String]) -> String {
    plain::render(input, players)
}

#[cfg(test)]
mod tests {
    use super::{html, ansi};
    use ast::Node as N;

    #[test]
    fn ansi_works() {
        ansi(&[N::text("Here is "),
               N::Bold(vec![N::text("something")]),
               N::text(" for "),
               N::Player(0),
               N::text(" and "),
               N::Player(1)],
             &vec!["mick".to_string(), "steve".to_string()]);
    }

    #[test]
    fn html_works() {
        html(&[N::text("Here is "),
               N::Bold(vec![N::text("something")]),
               N::text(" for "),
               N::Player(0),
               N::text(" and "),
               N::Player(1)],
             &vec!["mick".to_string(), "steve".to_string()]);
    }
}
