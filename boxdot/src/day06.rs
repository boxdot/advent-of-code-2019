use std::collections::{HashSet, VecDeque};

type Error = Box<dyn std::error::Error>;

pub fn solve(input: &str) -> Result<(usize, usize), Error> {
    let g = parse(input)?;
    let part1 = g.num_orbits();
    let part2 = g
        .shortest_path_len(Vertex("YOU"), Vertex("SAN"))
        .and_then(|len| len.checked_sub(2))
        .ok_or_else(|| "path YOU -> SAN not found")?;
    Ok((part1, part2))
}

fn parse(input: &str) -> Result<Graph, Error> {
    let edges: Vec<(Vertex, Vertex)> = input
        .lines()
        .filter_map(|s| {
            let mut parts = s.split(')');
            let from = parts.next()?;
            let to = parts.next()?;
            Some((Vertex(from), Vertex(to)))
        })
        .collect();
    Ok(Graph { edges })
}

struct Graph<'a> {
    edges: Vec<(Vertex<'a>, Vertex<'a>)>,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Vertex<'a>(&'a str);

impl<'a> Graph<'a> {
    fn out_edges(&self, v: Vertex<'a>) -> impl Iterator<Item = &Vertex> {
        self.edges
            .iter()
            .filter_map(move |(from, to)| if *from == v { Some(to) } else { None })
    }

    fn edges(&self, v: Vertex<'a>) -> impl Iterator<Item = &Vertex> {
        self.edges.iter().filter_map(move |(from, to)| {
            if *from == v {
                Some(to)
            } else if *to == v {
                Some(from)
            } else {
                None
            }
        })
    }

    fn num_orbits(&self) -> usize {
        let mut stack = vec![(Vertex("COM"), 0)];

        let mut num_orbits = 0;
        while let Some((v, depth)) = stack.pop() {
            num_orbits += depth;
            for &w in self.out_edges(v) {
                // we assume that the graph does not contain cycles
                stack.push((w, depth + 1))
            }
        }

        num_orbits
    }

    fn shortest_path_len(&self, orig: Vertex, dest: Vertex) -> Option<usize> {
        let mut queue = VecDeque::new();
        queue.push_back((orig, 0));

        // the graph is not directed
        let mut seen = HashSet::new();
        seen.insert(orig);

        while let Some((v, depth)) = queue.pop_front() {
            if v == dest {
                return Some(depth);
            }

            for &w in self.edges(v) {
                if w != v && !seen.contains(&w) {
                    seen.insert(w);
                    queue.push_back((w, depth + 1));
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_orbits() {
        let g = parse(
            r#"COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L"#,
        )
        .unwrap();
        assert_eq!(g.num_orbits(), 42);
    }

    #[test]
    fn test_shortest_path() {
        let g = parse(
            r#"COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN"#,
        )
        .unwrap();
        assert_eq!(g.shortest_path_len(Vertex("YOU"), Vertex("SAN")), Some(6));
    }
}
