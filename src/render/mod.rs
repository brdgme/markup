use ast::Node;
use std::collections::HashMap;
use brdgme_color::Color;

pub mod transform;
pub mod html;
pub mod ansi;

type RenderFunc = Fn(&Node, &str) -> Result<String, String>;
type RenderFuncs = HashMap<String, Box<RenderFunc>>;
type TextFunc = Fn(&str) -> Result<String, String>;

pub fn render(input: &Vec<Node>,
              funcs: &RenderFuncs,
              text_func: Option<&TextFunc>)
              -> Result<String, String> {
    let mut output: Vec<String> = vec![];
    for n in input {
        match n {
            &Node::Text(ref t) => {
                output.push(if let Some(ref f) = text_func {
                    try!(f(t)).to_string()
                } else {
                    t.to_string()
                })
            }
            &Node::Tag(ref name, _, ref children) => {
                let mut child_render = String::new();
                if children.len() > 0 {
                    child_render = try!(render(children, funcs, text_func));
                }
                if let Some(ref f) = funcs.get(name) {
                    output.push(try!(f(n, &child_render)));
                } else {
                    return Err("no render func".to_string());
                }
            }
        }
    }
    Ok(output.join(""))
}

pub fn parse_color_node(n: &Node) -> Result<Color, String> {
    match n {
        &Node::Tag(ref name, ref args, _) => {
            if args.len() != 1 {
                return Err(format!("{} requires one argument", name));
            }
            use std::str::FromStr;
            Color::from_str(&args[0])
        }
        _ => Err("color node must be a tag node".to_string()),
    }
}
