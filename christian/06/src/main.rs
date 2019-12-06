use itertools::*;
use std::collections::{hash_map::Entry, HashMap};
use std::io::{self, prelude::*};

type Graph = HashMap<String, Vec<String>>;

fn parse(input: &Vec<String>) -> (Graph, Graph) {
    let mut result = (Graph::new(), Graph::new());
    for x in input {
        let (to, from) = x.split(')').next_tuple().unwrap();
        result.0.entry(from.into()).or_default().push(to.into());
        result.1.entry(to.into()).or_default().push(from.into());
        result.1.entry(from.into()).or_default();
        result.0.entry(to.into()).or_default();
    }

    return result;
}

fn solve1(data: &(Graph, Graph)) -> i64 {
    let mut orbits: HashMap<_, _> = // id -> (num_in_missing, num_orbits)
        data.0.iter().map(|(x, edges)| (x, (edges.len(), 0))).collect();

    let mut candidates: Vec<_> = orbits
        .iter()
        .filter_map(|(&x, &(count, _))| Some(x).filter(|_| count == 0))
        .collect();
    while let Some(next) = candidates.pop() {
        let edges = data.0.get(next).unwrap().iter();
        let orbit_count = |target| orbits.get(target).unwrap().1 + 1;
        orbits.get_mut(next).unwrap().1 = edges.map(orbit_count).sum();
        let in_edges = data.1.get(next).unwrap().iter();
        for target in in_edges {
            let remaining = &mut orbits.get_mut(target).unwrap().0;
            *remaining -= 1;
            if *remaining == 0 {
                candidates.push(target);
            }
        }
    }
    orbits.iter().map(|(_, (_, num))| num).sum()
}

fn solve2(data: &(Graph, Graph)) -> Option<i64> {
    let mut visited: HashMap<_, _> = HashMap::new();
    visited.insert("YOU", 0);
    let mut candidates = vec!["YOU".into()];
    while let Some(next) = candidates.pop() {
        let dist = *visited.get(next)?;
        let edges = (data.0.get(next)?.iter()).chain(data.1.get(next)?.iter());
        for target in edges {
            if let Entry::Vacant(entry) = visited.entry(target) {
                entry.insert(dist + 1);
                candidates.push(target);
            }
        }
    }
    visited.get("SAN").map(|&x| x - 2)
}

fn main() {
    let input = parse(&io::stdin().lock().lines().map(|x| x.unwrap()).collect());
    println!("Part1: {:?}", solve1(&input));
    println!("Part2: {:?}", solve2(&input));
}
