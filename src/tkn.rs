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
named!(pub tag_arg<&str>,
       chain!(
           opt!(multispace) ~
               arg: alt!(delimited!(char!('"'), map_res!(take_until!("\""), from_utf8), char!('"')) | tag_word) ,
           || { arg }
       ));
named!(pub tag<Token>, delimited!(tag!("{{"), chain!(
    pre: opt!(one_of!("#/")) ~
        opt!(multispace) ~
        tag_name: tag_word ~
        args: many0!(tag_arg) ~
        opt!(multispace) ,
    || {
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
    fn tag_works() {
        assert_eq!(tag(b"{{bacon}}"), Done(&b""[..], Token::TagSingle("bacon", vec![])));
        assert_eq!(tag(b"{{bacon cheese}}"), Done(&b""[..], Token::TagSingle("bacon", vec!["cheese"])));
    }
}
