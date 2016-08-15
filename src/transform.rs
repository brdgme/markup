use ast::Node;
use std::collections::HashMap;

pub fn transform<F>(nodes: Vec<Node>, funcs: &HashMap<String, F>) -> Vec<Node>
    where F: Fn(Node) -> Vec<Node>
{
    let mut transformed: Vec<Node> = vec![];
    for n in nodes {
        match n.clone() {
            Node::Tag(tag_name, args, children) => {
                if let Some(f) = funcs.get(&tag_name) {
                    transformed.extend(f(n));
                } else {
                    transformed.push(Node::Tag(tag_name, args, transform(children, funcs)));
                }
            }
            _ => transformed.push(n),
        }
    }
    transformed
}
