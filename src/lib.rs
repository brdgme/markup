#![feature(plugin)]
#![plugin(peg_syntax_ext)]

extern crate brdgme_color;

pub mod ast;
mod error;
mod transform;
mod ansi;
mod html;
peg_file! parser("parser.peg");

use ast::Node;
use parser::ParseResult;
use error::MarkupError;

pub fn parse(input: &str) -> ParseResult<Vec<Node>> {
    parser::markup(input)
}

pub fn html(input: &Vec<Node>, players: &Vec<String>) -> Result<String, MarkupError> {
    html::render(input, players)
}

pub fn ansi(input: &Vec<Node>, players: &Vec<String>) -> Result<String, MarkupError> {
    ansi::render(input, players)
}

#[cfg(test)]
mod tests {
    use super::{parse, parser, html, ansi};
    use ast::Node;

    #[test]
    fn text_works() {
        assert_eq!(parser::text("blah blah blah"),
                   Ok(Node::Text("blah blah blah".to_string())));
        assert_eq!(parser::text("{blah blah{ blah"),
                   Ok(Node::Text("{blah blah{ blah".to_string())));
    }

    #[test]
    fn markup_works() {
        assert_eq!(parse(r"This is some text {{player 5}}"),
                   Ok(vec![
                       Node::Text("This is some text ".to_string()),
                       Node::Player(5),
                   ]));
        assert_eq!(parse(r"Testing blocks {{#b}}for {{#b}}superbold {{player 2}}{{/b}}{{/b}}!"),
                   Ok(vec![
                       Node::Text("Testing blocks ".to_string()),
                       Node::Bold(vec![
                           Node::Text("for ".to_string()),
                           Node::Bold(vec![
                               Node::Text("superbold ".to_string()),
                               Node::Player(2),
                           ]),
                       ]),
                       Node::Text("!".to_string()),
                   ]));
        assert_eq!(parse("blah blah {blah"),
                   Ok(vec![Node::Text("blah blah {blah".to_string())]));
    }

    #[test]
    fn ansi_works() {
        ansi(&parse("Here is {{#b}}something{{/b}} for {{player 0}} and {{player 1}}").unwrap(),
             &vec!["mick".to_string(), "steve".to_string()])
            .unwrap();
    }

    #[test]
    fn html_works() {
        html(&parse("Here is {{#b}}something{{/b}} for {{player 0}} and {{player 1}}").unwrap(),
             &vec!["mick".to_string(), "steve".to_string()])
            .unwrap();
    }
}
