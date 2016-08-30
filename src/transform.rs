use ast::{Node, Row, Align};
use brdgme_color::player_color;
use std::cmp;
use std::iter;

pub fn transform(input: &Vec<Node>, players: &Vec<String>) -> Result<Vec<Node>, String> {
    let mut remaining: Vec<Node> = input.clone();
    remaining.reverse();
    let mut ret: Vec<Node> = vec![];
    while let Some(n) = remaining.pop() {
        match n {
            Node::Player(p) => {
                let mut t = try!(player(p, players));
                t.reverse();
                remaining.extend(t);
            }
            Node::Table(rows) => {
                let mut t = try!(table(&rows, players));
                t.reverse();
                remaining.extend(t);
            }
            Node::Align(a, w, c) => {
                let mut t = try!(align(a, w, &c, &players));
                t.reverse();
                remaining.extend(t);
            }
            Node::Fg(c, children) => ret.push(Node::Fg(c, try!(transform(&children, players)))),
            Node::Bg(c, children) => ret.push(Node::Bg(c, try!(transform(&children, players)))),
            Node::Bold(children) => ret.push(Node::Bold(try!(transform(&children, players)))),
            Node::Action(a, children) => {
                ret.push(Node::Action(a, try!(transform(&children, players))))
            }
            Node::Text(_) => ret.push(n),
        }
    }
    Ok(ret)
}

fn player(p: usize, players: &Vec<String>) -> Result<Vec<Node>, String> {
    let p_len = players.len();
    if p >= p_len {
        return Err(format!(
            "invalid player index {}, there are only {} players",
            p,
            p_len,
        ));
    }
    Ok(vec![
        Node::Bold(vec![
            Node::Fg(player_color(p), vec![
                Node::Text(format!("• {}", players[p])),
            ]),
        ]),
    ])
}

fn table(rows: &Vec<Row>, players: &Vec<String>) -> Result<Vec<Node>, String> {
    // Transform individual cells and calculate row heights and column widths.
    let mut transformed: Vec<Vec<Vec<Vec<Node>>>> = vec![];
    let mut widths: Vec<usize> = vec![];
    let mut heights: Vec<usize> = vec![];
    let mut cols: usize = 0;
    for r in rows {
        cols = cmp::max(cols, r.len());
        let mut row: Vec<Vec<Vec<Node>>> = vec![];
        let mut row_height: usize = 1;
        for (i, c) in r.iter().enumerate() {
            let cell_lines = try!(to_lines(&c.children, players));
            row_height = cmp::max(row_height, cell_lines.len());
            let width = cell_lines.iter().fold(0, |width, l| cmp::max(width, len(l)));
            if i >= widths.len() {
                widths.push(width);
            } else {
                widths[i] = cmp::max(widths[i], width);
            }
            row.push(cell_lines);
        }
        heights.push(row_height);
        transformed.push(row);
    }
    // Second pass, output, padding and aligning where required.
    let mut output: Vec<Node> = vec![];
    for (ri, r) in rows.iter().enumerate() {
        for line_i in 0..heights[ri] {
            if ri > 0 || line_i > 0 {
                output.push(Node::Text("\n".to_string()));
            }
            for ci in 0..cols {
                if let Some(c) = r.get(ci) {
                    output.push(if transformed[ri][ci].len() > line_i {
                        Node::Align(c.align.to_owned(),
                                    widths[ci],
                                    transformed[ri][ci][line_i].to_owned())
                    } else {
                        Node::Align(Align::Left, widths[ci], vec![])
                    });
                } else {
                    output.push(Node::Align(Align::Left, widths[ci], vec![]));
                }
            }
        }
    }
    Ok(output)
}

fn align(a: Align,
         width: usize,
         children: &Vec<Node>,
         players: &Vec<String>)
         -> Result<Vec<Node>, String> {
    let mut aligned: Vec<Node> = vec![];
    for l in try!(to_lines(children, players)) {
        if aligned.len() > 0 {
            aligned.push(Node::Text("\n".to_string()));
        }
        let l_len = len(&l);
        let diff = cmp::max(width, l_len) - l_len;
        match a {
            Align::Left => {
                aligned.extend(l);
                if diff > 0 {
                    aligned.push(Node::Text(iter::repeat(" ").take(diff).collect()));
                }
            }
            Align::Center => {
                let before = diff / 2;
                let after = (diff + 1) / 2;
                if before > 0 {
                    aligned.push(Node::Text(iter::repeat(" ").take(before).collect()));
                }
                aligned.extend(l);
                if after > 0 {
                    aligned.push(Node::Text(iter::repeat(" ").take(after).collect()));
                }
            }
            Align::Right => {
                if diff > 0 {
                    aligned.push(Node::Text(iter::repeat(" ").take(diff).collect()));
                }
                aligned.extend(l);
            }
        }
    }
    Ok(aligned)
}

fn len(nodes: &Vec<Node>) -> usize {
    nodes.iter().fold(0, |sum, n| {
        sum +
        match n {
            &Node::Text(ref text) => text.len(),
            _ => len(&n.children()),
        }
    })
}

/// `to_lines` splits text nodes into multiple text nodes, duplicating parent
/// nodes as necessary.
fn to_lines(nodes: &Vec<Node>, players: &Vec<String>) -> Result<Vec<Vec<Node>>, String> {
    let mut lines: Vec<Vec<Node>> = vec![];
    let transformed = try!(transform(nodes, players));
    let mut line: Vec<Node> = vec![];
    for n in transformed {
        let n_lines: Vec<Vec<Node>> = match n {
            Node::Fg(color, children) => {
                try!(to_lines(&children, players))
                    .iter()
                    .map(|l| vec![Node::Fg(color, l.to_owned())])
                    .collect()
            }
            Node::Bg(color, children) => {
                try!(to_lines(&children, players))
                    .iter()
                    .map(|l| vec![Node::Bg(color, l.to_owned())])
                    .collect()
            }
            Node::Bold(children) => {
                try!(to_lines(&children, players))
                    .iter()
                    .map(|l| vec![Node::Bold(l.to_owned())])
                    .collect()
            }
            Node::Action(action, children) => {
                try!(to_lines(&children, players))
                    .iter()
                    .map(|l| vec![Node::Action(action.to_owned(), l.to_owned())])
                    .collect()
            }
            Node::Text(text) => text.split("\n").map(|l| vec![Node::Text(l.to_owned())]).collect(),
            _ => return Err(format!("invalid node to reduce to lines {:?}", n)),
        };
        let n_lines_len = n_lines.len();
        if n_lines_len > 0 {
            line.extend(n_lines[0].to_owned());
            if n_lines_len > 1 {
                lines.push(line);
                for i in 1..(n_lines_len - 1) {
                    lines.push(n_lines[i].to_owned());
                }
                line = n_lines[n_lines_len - 1].to_owned();
            }
        }
    }
    lines.push(line);
    Ok(lines)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ast::{Node, Align};
    use parser;

    #[test]
    fn it_works() {
        assert_eq!(transform(&vec![Node::Player(1)],
                             &vec!["mick".to_string(), "steve".to_string()]),
                   Ok(parser::markup("{{#b}}{{#fg #d32f2f}}• steve{{/fg}}{{/b}}").unwrap()));

    }

    #[test]
    fn align_works() {
        assert_eq!(transform(&vec![Node::Align(Align::Left,
                                               10,
                                               vec![Node::Text("abc".to_string())])],
                             &vec![]),
                   Ok(vec![
                       Node::Text("abc".to_string()),
                       Node::Text("       ".to_string()),
                   ]));
        assert_eq!(transform(&vec![Node::Align(Align::Center,
                                               10,
                                               vec![Node::Text("abc".to_string())])],
                             &vec![]),
                   Ok(vec![
                       Node::Text("   ".to_string()),
                       Node::Text("abc".to_string()),
                       Node::Text("    ".to_string()),
                   ]));
        assert_eq!(transform(&vec![Node::Align(Align::Right,
                                               10,
                                               vec![Node::Text("abc".to_string())])],
                             &vec![]),
                   Ok(vec![
                       Node::Text("       ".to_string()),
                       Node::Text("abc".to_string()),
                   ]));
    }
}
