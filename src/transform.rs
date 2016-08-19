use ast::Node;
use std::collections::HashMap;
use brdgme_color;

type TransformFunc = Fn(&Node) -> Result<Vec<Node>, String>;
type TransformFuncs = HashMap<String, Box<TransformFunc>>;

pub fn transform(input: &Vec<Node>, funcs: TransformFuncs) -> Result<Vec<Node>, String> {
    let mut remaining: Vec<Node> = input.clone();
    remaining.reverse();
    let mut ret: Vec<Node> = vec![];
    while let Some(n) = remaining.pop() {
        match n {
            Node::Text(_) => ret.push(n),
            Node::Tag(ref name, _, _) => {
                if let Some(f) = funcs.get(name) {
                    remaining.extend(try!(f(&n)));
                } else {
                    ret.push(n.clone());
                }
            }
        }
    }
    Ok(ret)
}

pub fn default_transform(players: Vec<String>) -> TransformFuncs {
    let mut funcs: TransformFuncs = HashMap::new();
    funcs.insert("player".to_string(),
                 Box::new(move |n| player_transform(n, players.clone())));
    funcs
}

fn player_transform(node: &Node, players: Vec<String>) -> Result<Vec<Node>, String> {
    match node {
        &Node::Text(_) => return Err("expected player tag, got text node".to_string()),
        &Node::Tag(_, ref tags, ref children) => {
            if children.len() > 0 {
                return Err("player tag should not have children".to_string());
            }
            if tags.len() != 1 {
                return Err("player tag should have exactly one argument".to_string());
            }
            if let Ok(pnum) = tags[0].parse::<usize>() {
                if pnum > players.len() {
                    return Err(format!("got player number {} but there are only {} players",
                                        pnum,
                                        players.len()));
                }
                return Ok(vec![
                    Node::Tag("b".to_string(), vec![], vec![
                        Node::Tag("fg".to_string(), vec![brdgme_color::player_color(pnum).hex()], vec![
                            Node::Text(players[pnum].clone()),
                        ]),
                    ]),
                ]);
            } else {
                return Err("player tag argument must be a positive integer".to_string());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ast::Node;
    use parser;

    #[test]
    fn it_works() {
        assert_eq!(transform(&vec![Node::Tag("player".to_string(), vec!["1".to_string()], vec![])],
                             default_transform(vec!["mick".to_string(), "steve".to_string()])),
                   Ok(parser::markup("{{#b}}{{#fg #d32f2f}}steve{{/fg}}{{/b}}").unwrap()));
    }
}
