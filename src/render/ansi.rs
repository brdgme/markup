use render::RenderFuncs;
use std::collections::HashMap;
use ast::Node;

pub fn render_funcs() -> RenderFuncs {
    let mut funcs: RenderFuncs = HashMap::new();
    funcs.insert("style".to_string(), Box::new(style));
    funcs
}

fn style(n: &Node, content: &str) -> Result<String, String> {
    Err("not implemented".to_string())
}

pub fn render(input: &Vec<Node>) -> Result<String, String> {
    super::render(input, &render_funcs(), None)
}
