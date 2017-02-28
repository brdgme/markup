use brdgme_color::Color;
use std::str::FromStr;

#[derive(PartialEq, Debug, Clone)]
pub enum Align {
    Left,
    Center,
    Right,
}

impl FromStr for Align {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "left" => Ok(Align::Left),
            "center" => Ok(Align::Center),
            "right" => Ok(Align::Right),
            _ => Err(format!("invalid align {}, must be one of left, center, right", s)),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum Node {
    Fg(Color, Vec<Node>),
    Bg(Color, Vec<Node>),
    Bold(Vec<Node>),
    Text(String),
    Player(usize),
    Table(Vec<Row>),
    Align(Align, usize, Vec<Node>),
    Indent(usize, Vec<Node>),
    Canvas(Vec<(usize, usize, Vec<Node>)>),
}

impl Node {
    pub fn text<T>(t: T) -> Node
        where T: Into<String>
    {
        Node::Text(t.into())
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum TNode {
    Fg(Color, Vec<TNode>),
    Bg(Color, Vec<TNode>),
    Bold(Vec<TNode>),
    Text(String),
}

impl TNode {
    pub fn text<T>(t: T) -> TNode
        where T: Into<String>
    {
        TNode::Text(t.into())
    }
}

pub type Row = Vec<Cell>;

pub type Cell = (Align, Vec<Node>);

