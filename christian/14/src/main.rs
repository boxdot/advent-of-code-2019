use itertools::*;
use std::collections::HashMap;
use std::io::{self, prelude::*};
#[macro_use]
extern crate text_io;

#[derive(Debug, Clone)]
struct Reagent {
    amount: isize,
    name: String,
}

#[derive(Debug, Clone)]
struct Reaction {
    inputs: Vec<Reagent>,
    amount: isize,
}

fn parse(input: &Vec<String>) -> HashMap<String, Reaction> {
    let parse = |x: &String| {
        let to_reagent = |x: &str| {
            let (amount, name);
            scan!(x.bytes() => "{} {}", amount, name);

            Reagent { amount, name }
        };
        let (left, right) = x.split(" => ").next_tuple().unwrap();
        let inputs: Vec<Reagent> = left.split(", ").map(to_reagent).collect();
        let Reagent { amount, name } = to_reagent(right);
        (name, Reaction { inputs, amount })
    };
    input.iter().map(parse).collect()
}

fn solve1(reactions: HashMap<String, Reaction>) -> (Option<isize>, isize) {
    let mut stack = Vec::new();
    let mut stock: HashMap<&str, _> = [("ORE", 1_000_000_000_000)].iter().cloned().collect();
    let mut per_fuel = None;
    let mut max_fuel = 0;
    loop {
        let fuel = per_fuel
            .map(|x| stock.get("ORE").unwrap() / x)
            .unwrap_or(1)
            .max(1);
        stack.push((fuel, "FUEL"));
        'search: while !stack.is_empty() {
            let target = stack.last().unwrap();
            if target.1 == "ORE" {
                return (per_fuel, max_fuel);
            }
            let reaction = reactions.get(target.1).unwrap();
            let amount = (target.0 + reaction.amount - 1) / reaction.amount;
            for input in &reaction.inputs {
                let diff = *stock.get(input.name.as_str()).unwrap_or(&0) - input.amount * amount;
                if diff < 0 {
                    stack.push((-diff, &input.name));
                    continue 'search;
                }
            }
            for input in &reaction.inputs {
                let diff = *stock.get(input.name.as_str()).unwrap_or(&0) - input.amount * amount;
                stock.insert(input.name.as_str(), diff);
            }
            let new_amount = stock.get(target.1).unwrap_or(&0) + amount * reaction.amount;
            stock.insert(target.1, new_amount);

            stack.pop();
        }
        if per_fuel.is_none() {
            per_fuel = Some(1_000_000_000_000 - stock.get("ORE").unwrap());
        }
        max_fuel += fuel;
    }
}

fn main() {
    let input = parse(&io::stdin().lock().lines().map(|x| x.unwrap()).collect());
    println!("{:?}", solve1(input));
}
