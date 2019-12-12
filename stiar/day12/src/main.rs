use day12::*;

use regex::Regex;

use std::io::{self, BufRead};

pub fn parse_moons(input: &[String]) -> Vec<Moon> {
    let re = Regex::new(r"x=(-?\d+), y=(-?\d+), z=(-?\d+)").unwrap();
    input.iter().map(|line| {
        let cap = re.captures(line).unwrap();
        Moon::new(
            cap.get(1).unwrap().as_str().parse().unwrap(),
            cap.get(2).unwrap().as_str().parse().unwrap(),
            cap.get(3).unwrap().as_str().parse().unwrap()
        )
    }).collect()
}

fn main() {
    let lines: Vec<_> = io::stdin().lock().lines().map(|x| x.unwrap()).collect();
    let moons = parse_moons(&lines);
    println!("Energy after 1000 turns: {}", calculate_energy_after(moons.clone(), 1000));
    println!("Period is: {}", calculate_period(&moons));
}
