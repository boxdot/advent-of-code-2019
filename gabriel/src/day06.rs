use itertools::*;
use std::collections::{hash_map::Entry, HashMap};

pub fn unlock(input: &str) -> Result<(usize, usize), Box<dyn std::error::Error>> {
    let (forward_graph, reverse_graph) = input.lines().fold(
        (HashMap::new(), HashMap::new()),
        |(mut forward_graph, mut reverse_graph), line| {
            let (n1, n2) = line.split(")").next_tuple().expect("malformed line");
            let node = forward_graph.entry(n1).or_insert(vec![]);
            (*node).push(n2);
            let node = reverse_graph.entry(n2).or_insert(vec![]);
            (*node).push(n1);
            (forward_graph, reverse_graph)
        },
    );

    let orbits: usize = forward_graph
        .values()
        .flatten()
        .map(|&node| {
            let mut stack = vec![node];
            let mut count = 0;
            while let Some(node) = stack.pop() {
                if let Some(parent) = reverse_graph.get(node) {
                    stack.append(&mut parent.clone());
                    count += 1;
                }
            }
            count
        })
        .sum();

    let mut visited = HashMap::new();
    visited.insert("YOU", 0);
    let mut stack = vec!["YOU"];
    while let Some(node) = stack.pop() {
        let distance = *visited.get(node).expect("something is terribly wrong");
        if let Some(edges) = reverse_graph.get(node) {
            for target in edges.iter() {
                if let Entry::Vacant(entry) = visited.entry(target) {
                    entry.insert(distance + 1);
                    stack.push(target);
                }
            }
        }
        if let Some(edges) = forward_graph.get(node) {
            for target in edges.iter() {
                if let Entry::Vacant(entry) = visited.entry(target) {
                    entry.insert(distance + 1);
                    stack.push(target);
                }
            }
        }
        if node == "SAN" {
            println!("Found santa at distance {}", distance - 2);
            break;
        }
    }

    let orbital_transfers_required = visited.get("SAN").expect("where is santa?");
    Ok((orbits, *orbital_transfers_required - 2))
}
