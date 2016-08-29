extern crate brdgme_markup;

use brdgme_markup::{ansi, parse};

fn main() {
    println!("{}",
             ansi(&parse(r#"
{{#b}}Egg{{/b}}
{{#table}}
  {{#row}}
    {{#cell}}Boo{{/cell}}
    {{#cell}}Bam{{/cell}}
  {{/row}}
{{/table}}
"#)
                      .unwrap(),
                  &vec!["mick".to_string(), "steve".to_string()])
                 .unwrap());
}
