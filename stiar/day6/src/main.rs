use day6::*;

use std::io::{self, BufRead};

fn main() {
    let lines: Vec<_> = io::stdin().lock().lines().map(|x| x.unwrap()).collect();
    let tree = parse_orbits(lines.iter().map(|x| x.as_str()), "COM");
    println!("Total number of orbits {}", calculate_sum_on_depths(&tree));
    println!("Distance {}", calculate_distance(&tree, "YOU", "SAN"));
}
