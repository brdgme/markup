extern crate brdgme_color;
extern crate brdgme_markup;

use brdgme_markup::ansi;
use brdgme_markup::ast::{Node, Cell, Align};
use brdgme_color::*;

fn main() {
    println!("{}",
             ansi(&vec![
                 Node::Bg(YELLOW, vec![
                     Node::Align(Align::Right, 80, vec![
                         Node::Bg(GREEN, vec![
                             Node::Table(vec![
                                 vec![
                                     Cell{ align: Align::Right, children: vec![
                                         Node::Text("moo".to_string())] },
                                     Cell{ align: Align::Left, children: vec![
                                         Node::Text(" ".to_string())] },
                                     Cell{ align: Align::Left, children: vec![
                                         Node::Bg(RED, vec![
                                             Node::Text("bags egg".to_string()),
                                         ]),
                                     ] }, 
                                 ],
                                 vec![],
                                 vec![
                                     Cell{ align: Align::Center, children: vec![
                                         Node::Bg(BLUE, vec![
                                             Node::Text("trololol\nmar".to_string())
                                         ]),
                                     ] },
                                     Cell{ align: Align::Left, children: vec![
                                         Node::Text(" ".to_string())] },
                                     Cell{ align: Align::Left, children: vec![
                                         Node::Text("har".to_string())] },
                                 ],
                             ]),
                         ]),
                     ]),
                 ]),
             ],
                  &vec!["mick".to_string(), "steve".to_string()])
             .unwrap());
}
