extern crate brdgme_color;

pub mod ast;
mod error;
mod transform;
mod ansi;
mod html;

use ast::Node;
use error::MarkupError;

pub fn html(input: &[Node], players: &[String]) -> Result<String, MarkupError> {
    html::render(input, players)
}

pub fn ansi(input: &[Node], players: &[String]) -> Result<String, MarkupError> {
    ansi::render(input, players)
}

#[cfg(test)]
mod tests {
    use super::{html, ansi};
    use ast::Node as N;

    #[test]
    fn ansi_works() {
        ansi(&[N::Text("Here is ".to_string()),
               N::Bold(vec![
                N::Text("something".to_string()),
            ]),
               N::Text(" for ".to_string()),
               N::Player(0),
               N::Text(" and ".to_string()),
               N::Player(1)],
             &vec!["mick".to_string(), "steve".to_string()])
            .unwrap();
    }

    #[test]
    fn html_works() {
        html(&[N::Text("Here is ".to_string()),
               N::Bold(vec![
                   N::Text("something".to_string()),
               ]),
               N::Text(" for ".to_string()),
               N::Player(0),
               N::Text(" and ".to_string()),
               N::Player(1)],
             &vec!["mick".to_string(), "steve".to_string()])
            .unwrap();
    }
}
