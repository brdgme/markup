use combine::{many, many1, Parser};
use combine::char::{letter, string, digit};
use combine::combinator::{choice, parser, try, none_of};
use combine::primitives::{Stream, ParseResult};

use std::str::FromStr;

use brdgme_color::*;

use ast::{Node, Row, Cell, Align};

pub fn parse<I>(input: I) -> ParseResult<Vec<Node>, I>
    where I: Stream<Item = char>
{
    many(choice([bold, fg, bg, c, player, canvas, table, text, align, indent])).parse_stream(input)
}

fn bold<I>(input: I) -> ParseResult<Node, I>
    where I: Stream<Item = char>
{
    (try(string("{{b}}")), parser(parse), string("{{/b}}"))
        .map(|(_, children, _)| Node::Bold(children))
        .parse_stream(input)
}

fn parse_u8<I>(input: I) -> ParseResult<u8, I>
    where I: Stream<Item = char>
{
    many1(digit()).and_then(|s: String| s.parse::<u8>()).parse_stream(input)
}

fn parse_usize<I>(input: I) -> ParseResult<usize, I>
    where I: Stream<Item = char>
{
    many1(digit()).and_then(|s: String| s.parse::<usize>()).parse_stream(input)
}

fn fg<I>(input: I) -> ParseResult<Node, I>
    where I: Stream<Item = char>
{
    (try(string("{{fg ")),
     parser(parse_u8),
     string(" "),
     parser(parse_u8),
     string(" "),
     parser(parse_u8),
     string("}}"),
     parser(parse),
     string("{{/fg}}"))
            .map(|(_, r, _, g, _, b, _, children, _)| {
                     Node::Fg(Color { r: r, g: g, b: b }, children)
                 })
            .parse_stream(input)
}

fn bg<I>(input: I) -> ParseResult<Node, I>
    where I: Stream<Item = char>
{
    (try(string("{{bg ")),
     parser(parse_u8),
     string(" "),
     parser(parse_u8),
     string(" "),
     parser(parse_u8),
     string("}}"),
     parser(parse),
     string("{{/bg}}"))
            .map(|(_, r, _, g, _, b, _, children, _)| {
                     Node::Bg(Color { r: r, g: g, b: b }, children)
                 })
            .parse_stream(input)
}

/// Backwards compatibility with Go brdgme. Magenta is handled manually as it doesn't exist in this
/// version of brdgme.
fn c<I>(input: I) -> ParseResult<Node, I>
    where I: Stream<Item = char>
{
    (try(string("{{c ")),
     many1::<String, _>(letter()),
     string("}}"),
     parser(parse),
     string("{{/c}}"))
            .map(|(_, col, _, children, _)| {
                Node::Fg(match col.as_ref() {
                                 "magenta" => Some(&PURPLE),
                                 _ => named(&col),
                             }
                             .unwrap_or(&BLACK)
                             .to_owned(),
                         children)
            })
            .parse_stream(input)
}

fn player<I>(input: I) -> ParseResult<Node, I>
    where I: Stream<Item = char>
{
    (try(string("{{player ")), parser(parse_usize), string("}}"))
        .map(|(_, p, _)| Node::Player(p))
        .parse_stream(input)
}

fn canvas<I>(input: I) -> ParseResult<Node, I>
    where I: Stream<Item = char>
{
    (try(string("{{canvas}}")), many(parser(layer)), string("{{/canvas}}"))
        .map(|(_, layers, _)| Node::Canvas(layers))
        .parse_stream(input)
}

fn layer<I>(input: I) -> ParseResult<(usize, usize, Vec<Node>), I>
    where I: Stream<Item = char>
{
    (try(string("{{layer ")),
     parser(parse_usize),
     string(" "),
     parser(parse_usize),
     string("}}"),
     parser(parse),
     string("{{/layer}}"))
            .map(|(_, x, _, y, _, children, _)| (x, y, children))
            .parse_stream(input)
}

fn table<I>(input: I) -> ParseResult<Node, I>
    where I: Stream<Item = char>
{
    (try(string("{{table}}")), many(parser(row)), string("{{/table}}"))
        .map(|(_, rows, _)| Node::Table(rows))
        .parse_stream(input)
}

fn row<I>(input: I) -> ParseResult<Row, I>
    where I: Stream<Item = char>
{
    (try(string("{{row}}")), many(parser(cell)), string("{{/row}}"))
        .map(|(_, cells, _)| cells)
        .parse_stream(input)
}

fn cell<I>(input: I) -> ParseResult<Cell, I>
    where I: Stream<Item = char>
{
    (try(string("{{cell ")), parser(align_arg), string("}}"), parser(parse), string("{{/cell}}"))
        .map(|(_, al, _, children, _)| (al, children))
        .parse_stream(input)
}

fn align<I>(input: I) -> ParseResult<Node, I>
    where I: Stream<Item = char>
{
    (try(string("{{align ")),
     parser(align_arg),
     string(" "),
     parser(parse_usize),
     string("}}"),
     parser(parse),
     string("{{/align}}"))
            .map(|(_, al, _, width, _, children, _)| Node::Align(al, width, children))
            .parse_stream(input)
}

fn indent<I>(input: I) -> ParseResult<Node, I>
    where I: Stream<Item = char>
{
    (try(string("{{indent ")),
     parser(parse_usize),
     string("}}"),
     parser(parse),
     string("{{/indent}}"))
            .map(|(_, width, _, children, _)| Node::Indent(width, children))
            .parse_stream(input)
}

fn align_arg<I>(input: I) -> ParseResult<Align, I>
    where I: Stream<Item = char>
{
    choice([string("left"), string("center"), string("right")])
        .map(|s| Align::from_str(s).unwrap())
        .parse_stream(input)
}

fn text<I>(input: I) -> ParseResult<Node, I>
    where I: Stream<Item = char>
{
    many1(none_of("{".chars())).map(Node::Text).parse_stream(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_works() {
        println!("{:?}",
                 parse("{{canvas}}{{layer 5 10}}{{table}}{{row}}{{cell center}}{{fg 255 160 0}}{{bg 25 118 210}}{{b}}{{c magenta}}moo{{/c}}{{/b}}{{/bg}}{{/fg}}{{/cell}}{{/row}}{{/table}}{{/layer}}{{/canvas}} And something else {{align right 10}}rrrr{{/align}}"));
    }
}
