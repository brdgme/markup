extern crate brdgme_markup;

use brdgme_markup::ansi;
use brdgme_markup::ast::{Node, Cell, Align};

fn main() {
    println!("{}",
             ansi(&vec![
                 Node::Table(vec![
                     vec![
                         Cell{ align: Align::Right, children: vec![Node::Text("moo".to_string())] },
                         Cell{ align: Align::Left, children: vec![Node::Text(" ".to_string())] },
                         Cell{ align: Align::Left, children: vec![Node::Text("bags egg".to_string())] },
                     ],
                     vec![],
                     vec![
                         Cell{ align: Align::Center, children: vec![Node::Text("trololol\nmar".to_string())] },
                         Cell{ align: Align::Left, children: vec![Node::Text(" ".to_string())] },
                         Cell{ align: Align::Left, children: vec![Node::Text("har".to_string())] },
                     ],
                 ]),
             ],
                  &vec!["mick".to_string(), "steve".to_string()])
                 .unwrap());
}
