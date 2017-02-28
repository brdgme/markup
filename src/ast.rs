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
    Action(String, Vec<Node>),
    Player(usize),
    Table(Vec<Row>),
    Align(Align, usize, Vec<Node>),
    Text(String),
    Group(Vec<Node>),
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

pub type Row = Vec<Cell>;

pub type Cell = (Align, Vec<Node>);

