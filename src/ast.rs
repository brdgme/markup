#[derive(PartialEq, Debug)]
pub enum Node {
    Text(String),
    Tag(String, Vec<String>, Vec<Node>),
}
