use ::tokenizer::{tokenize, Token};

#[derive(PartialEq, Debug)]
pub enum Node {
    Text(String),
    Tag(String, Vec<String>, Vec<Node>),
}

pub fn parse(input: &str) -> Result<Vec<Node>, String> {
    tokenize(input)
        .map_err(|e| format!("{}", e))
        .map(|tokens| {
            tokens.into_iter()
                .map(|t| match t {
                    Token::Text(s) => Node::Text(s),
                    Token::TagSingle(n, args) => Node::Tag(n, args, vec![]),
                    Token::TagBlockOpen(n, args) => Node::Tag(n, args, vec![]),
                    Token::TagBlockClose(n) => Node::Tag(n, vec![], vec![]),
                })
                .collect()
        })
}

fn nodify(tokens: &Vec<Token>) -> Result<Vec<Node>, String> {
    let mut i: usize = 0;
    let mut nodes: Vec<Node> = vec![];
    while i < tokens.len() {
        match tokens[i] {
            Token::Text(s) => nodes.push(Node::Text(s)),
            Token::TagSingle(t, a) => nodes.push(Node::Tag(t, a, vec![])),
            Token::TagBlockOpen(_, a) => {
                let (n, new_index) = try!(nodify_tag(tokens, i));
                nodes.push(n);
                i = new_index;
            }
            Token::TagBlockClose(t) => {
                return Err(format!("unexpected close tag: {}", t));
            }
        }
        i += 1;
    }
    Err("shit".to_string())
}

fn nodify_tag(tokens: &Vec<Token>, start: usize) -> Result<(Node, usize), String> {
    Err("not implemented".to_string())
}

#[cfg(test)]
mod tests {
    extern crate env_logger;
    use super::*;

    #[test]
    fn parse_works() {
        let _ = env_logger::init();
        info!("{:?}", parse("This is {{broken really bad}}").unwrap());
    }
}
