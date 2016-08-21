#![feature(plugin)]
#![plugin(peg_syntax_ext)]

extern crate brdgme_color;

pub mod ast;
pub mod render;
mod error;
peg_file! parser("parser.peg");

use ast::Node;
use parser::ParseResult;
use error::MarkupError;

pub fn parse(input: &str) -> ParseResult<Vec<Node>> {
    parser::markup(input)
}

pub fn html(input: &str, players: Vec<String>) -> Result<String, MarkupError> {
    render::html::render(input, players)
}

#[cfg(test)]
mod tests {
    use super::{parse, parser, html};
    use ast::Node;

    #[test]
    fn tag_name_works() {
        assert_eq!(parser::tag_name("farty"), Ok("farty".to_string()));
    }

    #[test]
    fn tag_works() {
        assert_eq!(parser::tag("{{farty smeg}}"),
                   Ok(Node::Tag("farty".to_string(), vec!["smeg".to_string()], vec![])));
    }

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
                       Node::Tag("player".to_string(), vec!["5".to_string()], vec![]),
                   ]));
        assert_eq!(parse(r"Testing blocks {{#b}}for {{#b}}superbold {{player 2}}{{/b}}{{/b}}!"),
                   Ok(vec![
                       Node::Text("Testing blocks ".to_string()),
                       Node::Tag("b".to_string(), vec![], vec![
                           Node::Text("for ".to_string()),
                           Node::Tag("b".to_string(), vec![], vec![
                               Node::Text("superbold ".to_string()),
                               Node::Tag("player".to_string(), vec!["2".to_string()], vec![]),
                           ]),
                       ]),
                       Node::Text("!".to_string()),
                   ]));
        assert_eq!(parse("blah blah {blah"),
                   Ok(vec![Node::Text("blah blah {blah".to_string())]));
    }

    #[test]
    fn html_works() {
        println!("{}",
                 html("Here is {{#b}}something{{/b}} for {{player 0}} and {{player 1}}",
                      vec!["mick".to_string(), "steve".to_string()])
                     .unwrap())
    }
}
