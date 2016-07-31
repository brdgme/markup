use combine::*;

#[derive(PartialEq, Debug)]
pub enum Token<'a> {
    Text(&'a str),
    TagSingle(&'a str, Vec<&'a str>),
    TagBlockOpen(&'a str, Vec<&'a str>),
    TagBlockClose(&'a str),
}

pub fn markup_tokens<'a, I>(input: &'a I) -> ParseResult<Vec<Token>, I>
    where I: Stream<Item = char>
{
    many(choice([text, tag])).parse_state(input)
}

pub fn text<'a, I>(input: &'a I) -> ParseResult<Token, I>
    where I: Stream<Item = char>
{
    try(many1(choice([not_brace, single_brace])).map(Token::Text)).parse_state(input)
}

pub fn not_brace<I>(input: I) -> ParseResult<char, I>
    where I: Stream<Item = char>
{
    try(satisfy(|c| c != '{')).parse_state(input)
}

pub fn single_brace<I>(input: I) -> ParseResult<char, I>
    where I: Stream<Item = char>
{
    try(char('{').skip(not_followed_by(char('{')))).parse_state(input)
}

pub fn tag<'a, I>(input: &'a I) -> ParseResult<Token, I>
    where I: Stream<Item = char>
{
    try(parser(tag_parts).map(|(prefix, tag, args)| match prefix {
            Some('#') => Token::TagBlockOpen(tag, args),
            Some('/') => Token::TagBlockClose(tag),
            _ => Token::TagSingle(tag, args),
        }))
        .parse_state(input)
}

pub fn tag_parts<'a, I>(input: I) -> ParseResult<(Option<char>, &'a str, Vec<&'a str>), I>
    where I: Stream<Item = char>
{
    between(string("{{"),
            string("}}"),
            (optional(parser(tag_prefix)), parser(tag_word), parser(tag_arguments)))
        .parse_state(input)
}

pub fn tag_prefix<I>(input: I) -> ParseResult<char, I>
    where I: Stream<Item = char>
{
    satisfy(|c| c == '#' || c == '/').parse_state(input)
}

pub fn tag_word<'a, I>(input: I) -> ParseResult<&'a str, I>
    where I: 'a + Stream<Item = char>
{
    many1(satisfy(|c: char| c != '}' && !c.is_whitespace()))
        .skip(spaces())
        .parse_state(input)
}

pub fn tag_argument<'a, I>(input: I) -> ParseResult<&'a str, I>
    where I: 'a + Stream<Item = char>
{
    parser(tag_word).parse_state(input)
}

pub fn tag_arguments<'a, I>(input: I) -> ParseResult<Vec<&'a str>, I>
    where I: 'a + Stream<Item = char>
{
    many(parser(tag_argument)).parse_state(input)
}

pub fn tokenize<'a, I>(input: &'a I) -> Result<Vec<Token>, ParseError<I>>
    where I: Stream<Item = char>
{
    parser(markup_tokens).parse(input).map(|(t, _)| t)
}

#[cfg(test)]
mod tests {
    use super::*;
    use combine::{parser, Parser};

    #[test]
    fn tokenize_works() {
        assert_eq!(vec![Token::Text("\nThis is "),
                         Token::TagBlockOpen("b", vec![]),
                         Token::Text("totally "),
                         Token::TagBlockOpen("fg", vec!["green"]),
                         Token::Text("some markup"),
                         Token::TagBlockClose("fg"),
                         Token::Text(" yeah"),
                         Token::TagBlockClose("b"),
                         Token::Text(".\n\n"),
                         Token::TagSingle("player", vec!["5"]),
                         Token::Text(" is the best.\n"),
                ],
                   tokenize(r"
This is {{#b}}totally {{#fg green}}some markup{{/fg}} yeah{{/b}}.

{{player 5}} is the best.
")
                       .unwrap());
    }

    #[test]
    fn tag_parts_works() {
        assert_eq!(((Some('#'), "fartypoo", vec!["bacon", "egg"]), " cheese"),
                   parser(tag_parts).parse("{{#fartypoo bacon egg}} cheese").unwrap());
        assert_eq!(((Some('/'), "fartypoo", vec!["bacon", "egg"]), " cheese"),
                   parser(tag_parts).parse("{{/fartypoo bacon egg}} cheese").unwrap());
        assert_eq!(((None, "fartypoo", vec!["bacon", "egg"]), " cheese"),
                   parser(tag_parts).parse("{{fartypoo bacon egg}} cheese").unwrap());
    }

    #[test]
    fn tag_argument_works() {
        parser(tag_argument).parse("egggbacon    ").unwrap();
    }

    #[test]
    fn text_works() {
        assert_eq!((Token::Text("egggbacon  {cheese}  "), "{{blah}}"),
                   parser(text).parse("egggbacon  {cheese}  {{blah}}").unwrap());
    }
}
