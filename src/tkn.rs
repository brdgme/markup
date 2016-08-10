use nom::multispace;
use std::str::from_utf8;

#[derive(PartialEq, Debug)]
pub enum Token<'a> {
    Text(&'a str),
    TagSingle(&'a str, Vec<&'a str>),
    TagBlockOpen(&'a str, Vec<&'a str>),
    TagBlockClose(&'a str),
}

named!(pub tag_word<&str>, map_res!(take_until_either!(" }"), from_utf8));
named!(pub tag_arg<&str>, alt!(
    delimited!(char!('"'), map_res!(take_until!("\""), from_utf8), char!('"')) |
    tag_word
));
named!(pub tag_args<Vec<&str> >, separated_list!(multispace, tag_arg));
named!(pub tag<Token>, delimited!(tag!("{{"), chain!(
    pre: opt!(one_of!("#/")) ~
        opt!(multispace) ~
        tag_name: tag_word ~
        opt!(multispace) ~
        opt_args: opt!(tag_args) ~
        opt!(multispace) ,
    || {
        let args = match opt_args.clone() {
            Some(a) => a,
            None => vec![],
        };
        match pre {
            Some('#') => Token::TagBlockOpen(tag_name, args),
            Some('/') => Token::TagBlockClose(tag_name),
            _ => Token::TagSingle(tag_name, args),
        }
    }
), tag!("}}")));

#[cfg(test)]
mod tests {
    use super::*;
    use nom::IResult::*;

    #[test]
    fn tag_arg_works() {
        assert_eq!(tag_arg(b"cheese}"), Done(&b"}"[..], "cheese"));
        assert_eq!(tag_arg(b"cheese "), Done(&b" "[..], "cheese"));
        assert_eq!(tag_arg(b"\"cheese bacon tomato\""), Done(&b""[..], "cheese bacon tomato"));
    }

    #[test]
    fn tag_args_works() {
        assert_eq!(tag_args(b"cheese bacon tomato}}"), Done(&b"}}"[..], vec!["cheese", "bacon", "tomato"]));
    }

    #[test]
    fn tag_works() {
        assert_eq!(tag(b"{{bacon}}"), Done(&b""[..], Token::TagSingle("bacon", vec![])));
        assert_eq!(tag(b"{{bacon cheese}}"), Done(&b""[..], Token::TagSingle("bacon", vec!["cheese"])));
    }
}
