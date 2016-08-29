extern crate brdgme_markup;

use brdgme_markup::ansi;
use brdgme_markup::ast::{Node, Cell, Align};

fn main() {
    println!("{}",
             ansi(&vec![
                 Node::Table(vec![
                     vec![Cell{ align: Align::Left, children: vec![Node::Text("moo".to_string())] }],
                 ]),
             ],
                  &vec!["mick".to_string(), "steve".to_string()])
                 .unwrap());
}
