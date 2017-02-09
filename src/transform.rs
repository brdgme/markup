use ast::{Node, Row, Align};
use brdgme_color::player_color;
use std::cmp;
use std::iter;

pub fn transform(input: &[Node], players: &[String]) -> Vec<Node> {
    let mut remaining: Vec<Node> = input.to_owned();
    remaining.reverse();
    let mut ret: Vec<Node> = vec![];
    while let Some(n) = remaining.pop() {
        match n {
            Node::Player(p) => {
                let mut t = player(p, players);
                t.reverse();
                remaining.extend(t);
            }
            Node::Table(rows) => {
                let mut t = table(&rows, players);
                t.reverse();
                remaining.extend(t);
            }
            Node::Align(a, w, c) => {
                let mut t = align(a, w, &c, players);
                t.reverse();
                remaining.extend(t);
            }
            Node::Indent(n, c) => {
                let mut t = indent(n, &c, players);
                t.reverse();
                remaining.extend(t);
            }
            Node::Fg(c, children) => ret.push(Node::Fg(c, transform(&children, players))),
            Node::Bg(c, children) => ret.push(Node::Bg(c, transform(&children, players))),
            Node::Bold(children) => ret.push(Node::Bold(transform(&children, players))),
            Node::Action(a, children) => ret.push(Node::Action(a, transform(&children, players))),
            Node::Text(_) => ret.push(n),
            Node::Group(children) => {
                remaining.extend(children.into_iter().rev().collect::<Vec<Node>>());
            }
        }
    }
    ret
}

fn player(p: usize, players: &[String]) -> Vec<Node> {
    let p_len = players.len();
    let p_name = if p < p_len {
        players[p].to_string()
    } else {
        format!("Player {}", p)
    };
    vec![Node::Bold(vec![Node::Fg(player_color(p).to_owned(),
                                  vec![Node::text(format!("• {}", p_name))])])]
}

fn table(rows: &[Row], players: &[String]) -> Vec<Node> {
    // Transform individual cells and calculate row heights and column widths.
    let mut transformed: Vec<Vec<Vec<Vec<Node>>>> = vec![];
    let mut widths: Vec<usize> = vec![];
    let mut heights: Vec<usize> = vec![];
    for r in rows {
        let mut row: Vec<Vec<Vec<Node>>> = vec![];
        let mut row_height: usize = 1;
        for (i, &(_, ref children)) in r.iter().enumerate() {
            let cell_lines = to_lines(children, players);
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
                output.push(Node::text("\n"));
            }
            for (ci, w) in widths.iter().enumerate() {
                if let Some(&(ref align, _)) = r.get(ci) {
                    output.push(if transformed[ri][ci].len() > line_i {
                        Node::Align(align.to_owned(), *w, transformed[ri][ci][line_i].to_owned())
                    } else {
                        Node::Align(Align::Left, widths[ci], vec![])
                    });
                } else {
                    output.push(Node::Align(Align::Left, widths[ci], vec![]));
                }
            }
        }
    }
    output
}

fn align(a: Align, width: usize, children: &[Node], players: &[String]) -> Vec<Node> {
    let mut aligned: Vec<Node> = vec![];
    for l in to_lines(children, players) {
        if !aligned.is_empty() {
            aligned.push(Node::text("\n"));
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
    aligned
}

fn indent(n: usize, children: &[Node], players: &[String]) -> Vec<Node> {
    from_lines(&to_lines(children, players)
        .iter()
        .map(|l| {
            let mut new_l = vec![Node::Text(iter::repeat(" ").take(n).collect())];
            new_l.extend(l.clone());
            new_l
        })
        .collect::<Vec<Vec<Node>>>())
}

fn len(nodes: &[Node]) -> usize {
    nodes.iter().fold(0, |sum, n| {
        sum +
        match *n {
            Node::Text(ref text) => text.chars().count(),
            _ => len(&n.children()),
        }
    })
}

/// `to_lines` splits text nodes into multiple text nodes, duplicating parent
/// nodes as necessary.
pub fn to_lines(nodes: &[Node], players: &[String]) -> Vec<Vec<Node>> {
    let mut lines: Vec<Vec<Node>> = vec![];
    let transformed = transform(nodes, players);
    let mut line: Vec<Node> = vec![];
    for n in transformed {
        let n_lines: Vec<Vec<Node>> = match n {
            Node::Fg(color, children) => {
                to_lines(&children, players)
                    .iter()
                    .map(|l| vec![Node::Fg(color, l.to_owned())])
                    .collect()
            }
            Node::Bg(color, children) => {
                to_lines(&children, players)
                    .iter()
                    .map(|l| vec![Node::Bg(color, l.to_owned())])
                    .collect()
            }
            Node::Bold(children) => {
                to_lines(&children, players)
                    .iter()
                    .map(|l| vec![Node::Bold(l.to_owned())])
                    .collect()
            }
            Node::Action(action, children) => {
                to_lines(&children, players)
                    .iter()
                    .map(|l| vec![Node::Action(action.to_owned(), l.to_owned())])
                    .collect()
            }
            Node::Text(text) => text.split('\n').map(|l| vec![Node::text(l)]).collect(),
            Node::Player(_) |
            Node::Table(_) |
            Node::Align(_, _, _) |
            Node::Group(_) |
            Node::Indent(_, _) => panic!("found untransformed node"),
        };
        let n_lines_len = n_lines.len();
        if n_lines_len > 0 {
            line.extend(n_lines[0].to_owned());
            if n_lines_len > 1 {
                lines.push(line);
                for l in n_lines.iter().take(n_lines_len - 1).skip(1) {
                    lines.push(l.to_owned());
                }
                line = n_lines[n_lines_len - 1].to_owned();
            }
        }
    }
    lines.push(line);
    lines
}

pub fn from_lines(lines: &[Vec<Node>]) -> Vec<Node> {
    lines.iter()
        .enumerate()
        .flat_map(|(i, l)| {
            let mut new_l = if i == 0 {
                vec![]
            } else {
                vec![Node::text("\n")]
            };
            new_l.extend(l.clone());
            new_l
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use brdgme_color::*;
    use plain::render;
    use ast::{Node as N, Align as A};

    #[test]
    fn align_works() {
        assert_eq!(transform(&vec![N::Align(A::Left, 10, vec![N::text("abc")])], &vec![]),
                   vec![N::text("abc"), N::text("       ")]);
        assert_eq!(transform(&vec![N::Align(A::Center, 10, vec![N::text("abc")])],
                             &vec![]),
                   vec![N::text("   "), N::text("abc"), N::text("    ")]);
        assert_eq!(transform(&vec![N::Align(A::Right, 10, vec![N::text("abc")])], &vec![]),
                   vec![N::text("       "), N::text("abc")]);
    }

    #[test]
    fn table_align_works() {
        assert_eq!("           blah     \nheadersome long text".to_string(),
                   render(&vec![N::Table(vec![vec![(A::Left, vec![]),
                                                   (A::Center,
                                                    vec![N::Fg(GREY, vec![N::text("blah")])])],
                                              vec![(A::Right, vec![N::text("header")]),
                                                   (A::Center,
                                                    vec![N::text("some long text")])]])],
                          &vec![]));
    }

    #[test]
    fn table_in_table_works() {
        let t = transform(&vec![N::Table(vec![vec![(A::Left, vec![N::text("one")])],
                                              vec![(A::Left, vec![N::text("two")])],
                                              vec![(A::Left, vec![N::text("three")])]])],
                          &vec![]);
        assert_eq!(render(&t, &vec![]),
                   render(&vec![N::Table(vec![vec![(A::Left, t.clone())]])], &vec![]));
    }

    #[test]
    fn to_lines_works() {
        assert_eq!(to_lines(&vec![N::text("one\ntwo")], &vec![]),
                   vec![vec![N::text("one")], vec![N::text("two")]]);
    }
}
