use day3::*;

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let line1 = lines.next().unwrap().unwrap();
    let line2 = lines.next().unwrap().unwrap();

    let first_wire = parse_actions(&line1);
    let second_wire = parse_actions(&line2);

    println!(
        "Manhattan: {}, length: {}",
        find_closest_intersection_distance(&first_wire, &second_wire, Mode::Manhattan),
        find_closest_intersection_distance(&first_wire, &second_wire, Mode::WireLength),
    );
}
