use ast::Node;
use std::collections::HashMap;

pub mod html;

type RenderFunc = Fn(&Node, &str) -> Result<String, String>;
type RenderFuncs = HashMap<String, Box<RenderFunc>>;
type TextFunc = Fn(&str) -> Result<String, String>;

pub fn render(input: &Vec<Node>,
              funcs: &RenderFuncs,
              text_func: Option<&TextFunc>)
              -> Result<String, String> {
    let mut output = String::new();
    for n in input {
        match n {
            &Node::Text(ref t) => {
                output = format!("{}{}",
                                 output,
                                 if let Some(ref f) = text_func {
                                     try!(f(t)).to_string()
                                 } else {
                                     t.to_string()
                                 });
            }
            &Node::Tag(ref name, _, ref children) => {
                let mut child_render = String::new();
                if children.len() > 0 {
                    child_render = try!(render(children, funcs, text_func));
                }
                if let Some(ref f) = funcs.get(name) {
                    output = format!("{}{}", output, try!(f(n, &child_render)));
                } else {
                    return Err("no render func".to_string());
                }
            }
        }
    }
    Ok(output)
}
