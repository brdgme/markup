use ast::{Node, Row, Align};
use brdgme_color::player_color;
use std::cmp;
use std::iter;
use std::ops::Range;

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
            Node::Canvas(els) => {
                let mut t = canvas(&els, players);
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
            let cell_lines = to_lines(&transform(children, players));
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
                                    Node::Align(align.to_owned(),
                                                *w,
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
    output
}

fn align(a: Align, width: usize, children: &[Node], players: &[String]) -> Vec<Node> {
    let mut aligned: Vec<Node> = vec![];
    for l in to_lines(&transform(children, players)) {
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
    from_lines(&to_lines(&transform(children, players))
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
fn to_lines(nodes: &[Node]) -> Vec<Vec<Node>> {
    let mut lines: Vec<Vec<Node>> = vec![];
    let mut line: Vec<Node> = vec![];
    for n in nodes {
        let n_lines: Vec<Vec<Node>> = match *n {
            Node::Fg(ref color, ref children) => {
                to_lines(children)
                    .iter()
                    .map(|l| vec![Node::Fg(*color, l.to_owned())])
                    .collect()
            }
            Node::Bg(ref color, ref children) => {
                to_lines(children)
                    .iter()
                    .map(|l| vec![Node::Bg(*color, l.to_owned())])
                    .collect()
            }
            Node::Bold(ref children) => {
                to_lines(children)
                    .iter()
                    .map(|l| vec![Node::Bold(l.to_owned())])
                    .collect()
            }
            Node::Action(ref action, ref children) => {
                to_lines(children)
                    .iter()
                    .map(|l| vec![Node::Action(action.to_owned(), l.to_owned())])
                    .collect()
            }
            Node::Text(ref text) => text.split('\n').map(|l| vec![Node::text(l)]).collect(),
            Node::Player(_) |
            Node::Table(_) |
            Node::Align(_, _, _) |
            Node::Group(_) |
            Node::Indent(_, _) |
            Node::Canvas(_) => panic!("found untransformed node"),
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

fn from_lines(lines: &[Vec<Node>]) -> Vec<Node> {
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

fn slice(nodes: &[Node], range: Range<usize>) -> Vec<Node> {
    if range.start >= range.end {
        return vec![];
    }
    let mut s = vec![];
    let mut start = range.start;
    let mut end = range.end;
    for n in nodes {
        let n_len = len(&[n.clone()]);
        if n_len < start {
            start -= n_len;
            end -= n_len;
            continue;
        }
        let n_s: Node = match *n {
            Node::Fg(ref color, ref children) => Node::Fg(*color, slice(children, start..end)),
            Node::Bg(ref color, ref children) => Node::Bg(*color, slice(children, start..end)),
            Node::Bold(ref children) => Node::Bold(slice(children, start..end)),
            Node::Action(ref action, ref children) => {
                Node::Action(action.to_string(), slice(children, start..end))
            }
            Node::Text(ref text) => Node::Text(text[start..cmp::min(text.len(), end)].to_string()),
            Node::Player(_) |
            Node::Table(_) |
            Node::Align(_, _, _) |
            Node::Group(_) |
            Node::Indent(_, _) |
            Node::Canvas(_) => panic!("found untransformed node"),
        };

        let n_s_len = len(&[n_s.clone()]);
        s.push(n_s);
        end -= cmp::min(start + n_s_len, end);
        if end == 0 {
            break;
        }
        start = 0;
    }
    s
}

fn canvas(els: &[(usize, usize, Vec<Node>)], players: &[String]) -> Vec<Node> {
    // Output is split into lines each with a start position.
    let mut lines: Vec<Vec<(usize, Vec<Node>)>> = vec![];
    for &(x, y, ref nodes) in els {
        let lines_len = lines.len();
        let node_lines = to_lines(&transform(nodes, players));
        let node_lines_len = node_lines.len();
        if y + node_lines_len > lines_len {
            lines.extend(iter::repeat(vec![]).take(y + node_lines_len - lines_len));
        }
        for (n_i, n_line) in node_lines.iter().enumerate() {
            let n_line_y = y + n_i;
            let n_line_len = len(n_line);
            lines[n_line_y] = lines[n_line_y]
                .iter()
                .flat_map(|&(ex_x, ref ex_n_line)| {
                    let ex_n_line_len = len(ex_n_line);
                    if ex_x >= x && ex_x + ex_n_line_len <= x + n_line_len {
                        // Full overlap, remove.
                        return vec![];
                    }
                    if ex_x > x + n_line_len || x > ex_x + ex_n_line_len {
                        // No overlap, keep.
                        return vec![(ex_x, ex_n_line.clone())];
                    }
                    let mut new_parts = vec![];
                    if x > ex_x {
                        new_parts.push((ex_x, slice(ex_n_line, 0..x - ex_x)))
                    }
                    if ex_x + ex_n_line_len > x + n_line_len {
                        new_parts.push((x + n_line_len,
                                        slice(ex_n_line,
                                              ex_n_line_len -
                                              ((ex_x + ex_n_line_len) - (x + n_line_len))..
                                              ex_n_line_len)));
                    }
                    new_parts
                })
                .collect();
            lines[n_line_y].push((x, n_line.clone()));
        }
    }
    from_lines(&lines.iter()
                    .map(|l| {
        let mut sorted_l = l.clone();
        sorted_l.sort_by(|&(ref a, _), &(ref b, _)| a.cmp(b));
        let mut last_x = 0;
        sorted_l.iter()
            .flat_map(|&(x, ref nodes)| {
                let c_nodes = if x > last_x {
                    vec![Node::Indent(x - last_x, nodes.clone())]
                } else {
                    nodes.clone()
                };
                last_x = x + len(nodes);
                c_nodes
            })
            .collect()
    })
                    .collect::<Vec<Vec<Node>>>())
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
        assert_eq!(to_lines(&vec![N::text("one\ntwo")]),
                   vec![vec![N::text("one")], vec![N::text("two")]]);
    }

    #[test]
    fn slice_works() {
        assert_eq!(slice(&vec![N::Fg(RED, vec![N::Bold(vec![N::text("blah")])])],
                         1..3),
                   vec![N::Fg(RED, vec![N::Bold(vec![N::text("la")])])]);
        assert_eq!(slice(&vec![N::Bold(vec![N::Fg(RED, vec![N::text("one"), N::text("two")]),
                                            N::Bg(BLUE,
                                                  vec![N::text("three"), N::text("four")]),
                                            N::Action("fart".to_string(),
                                                      vec![N::text("five"), N::text("six")])])],
                         10..16),
                   vec![N::Bold(vec![N::Bg(BLUE, vec![N::text("e"), N::text("four")]),
                                     N::Action("fart".to_string(), vec![N::text("f")])])]);
    }
}

