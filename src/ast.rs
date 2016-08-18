#[derive(PartialEq, Debug, Clone)]
pub enum Node {
    Text(String),
    Tag(String, Vec<String>, Vec<Node>),
}
