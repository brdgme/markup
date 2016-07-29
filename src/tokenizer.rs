use combine::*;

#[derive(PartialEq, Debug)]
pub enum Token {
    Text(String),
    TagSingle(String, Vec<String>),
    TagBlockOpen(String, Vec<String>),
    TagBlockClose(String),
}

pub fn markup_tokens<I>(input: I) -> ParseResult<Vec<Token>, I>
    where I: Stream<Item = char>
{
    many(choice([text, tag])).parse_state(input)
}

pub fn text<I>(input: I) -> ParseResult<Token, I>
    where I: Stream<Item = char>
{
    try(many1(choice([not_brace, single_brace])).map(|s| Token::Text(s))).parse_state(input)
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

pub fn tag<I>(input: I) -> ParseResult<Token, I>
    where I: Stream<Item = char>
{
    try(parser(tag_parts).map(|(prefix, tag, args)| match prefix {
            Some('#') => Token::TagBlockOpen(tag, args),
            Some('/') => Token::TagBlockClose(tag),
            _ => Token::TagSingle(tag, args),
        }))
        .parse_state(input)
}

pub fn tag_parts<I>(input: I) -> ParseResult<(Option<char>, String, Vec<String>), I>
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

pub fn tag_word<I>(input: I) -> ParseResult<String, I>
    where I: Stream<Item = char>
{
    many1(satisfy(|c: char| c != '}' && !c.is_whitespace()))
        .skip(spaces())
        .parse_state(input)
}

pub fn tag_argument<I>(input: I) -> ParseResult<String, I>
    where I: Stream<Item = char>
{
    parser(tag_word).parse_state(input)
}

pub fn tag_arguments<I>(input: I) -> ParseResult<Vec<String>, I>
    where I: Stream<Item = char>
{
    many(parser(tag_argument)).parse_state(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use combine::{parser, Parser};

    #[test]
    fn markup_tokens_works() {
        assert_eq!((vec![Token::Text("\nThis is ".to_string()),
                         Token::TagBlockOpen("b".to_string(), vec![]),
                         Token::Text("totally ".to_string()),
                         Token::TagBlockOpen("fg".to_string(), vec!["green".to_string()]),
                         Token::Text("some markup".to_string()),
                         Token::TagBlockClose("fg".to_string()),
                         Token::Text(" yeah".to_string()),
                         Token::TagBlockClose("b".to_string()),
                         Token::Text(".\n\n".to_string()),
                         Token::TagSingle("player".to_string(), vec!["5".to_string()]),
                         Token::Text(" is the best.\n".to_string()),
                ],
                    ""),
                   parser(markup_tokens)
                       .parse(r"
This is {{#b}}totally {{#fg green}}some markup{{/fg}} yeah{{/b}}.

{{player 5}} is the best.
")
                       .unwrap());
    }

    #[test]
    fn tag_parts_works() {
        assert_eq!(((Some('#'),
                     "fartypoo".to_string(),
                     vec!["bacon".to_string(), "egg".to_string()]),
                    " cheese"),
                   parser(tag_parts).parse("{{#fartypoo bacon egg}} cheese").unwrap());
        assert_eq!(((Some('/'),
                     "fartypoo".to_string(),
                     vec!["bacon".to_string(), "egg".to_string()]),
                    " cheese"),
                   parser(tag_parts).parse("{{/fartypoo bacon egg}} cheese").unwrap());
        assert_eq!(((None, "fartypoo".to_string(), vec!["bacon".to_string(), "egg".to_string()]),
                    " cheese"),
                   parser(tag_parts).parse("{{fartypoo bacon egg}} cheese").unwrap());
    }

    #[test]
    fn tag_argument_works() {
        parser(tag_argument).parse("egggbacon    ").unwrap();
    }

    #[test]
    fn text_works() {
        assert_eq!((Token::Text("egggbacon  {cheese}  ".to_string()), "{{blah}}"),
                   parser(text).parse("egggbacon  {cheese}  {{blah}}").unwrap());
    }
}
