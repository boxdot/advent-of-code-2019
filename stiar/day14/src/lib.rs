#[macro_use]
extern crate scan_fmt;

use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ElementPortion {
    pub element: String,
    pub count: u64,
}

impl FromStr for ElementPortion {
    type Err = scan_fmt::parse::ScanError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (count, element) = scan_fmt!(s, "{d} {}", u64, String)?;
        Ok(Self { element, count })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Reaction {
    pub target: ElementPortion,
    pub ingridients: Vec<ElementPortion>,
}

pub type ReactionsTable = HashMap<String, Reaction>;

pub fn parse_reactions(input: &str) -> ReactionsTable {
    input
        .trim()
        .split('\n')
        .map(|line| {
            let mut split = line.split(" => ");
            let ingridients = split
                .next()
                .unwrap()
                .split(", ")
                .map(|ingridient| ingridient.parse::<ElementPortion>().unwrap())
                .collect::<Vec<_>>();
            let target = split
                .next()
                .unwrap()
                .to_string()
                .parse::<ElementPortion>()
                .unwrap();
            (
                target.element.clone(),
                Reaction {
                    target,
                    ingridients,
                },
            )
        })
        .collect()
}

// We could do topological sort of the elements to speed up the calculation, but it's not needed
// given the size of the input.
pub fn calculate_needed_source(
    reactions: &ReactionsTable,
    source: &str,
    target: &str,
    fuel_multiplier: u64,
) -> u64 {
    let mut leftovers = HashMap::new();
    let mut current_ingridients: HashMap<&str, _> = (&reactions.get(target).unwrap().ingridients)
        .iter()
        .map(|ingridient| {
            (
                ingridient.element.as_str(),
                ingridient.count * fuel_multiplier,
            )
        })
        .collect();

    while let Some(ingridient) = current_ingridients
        .iter()
        .filter(|(&key, _)| key != source)
        .map(|(&key, &value)| (key.clone(), value))
        .next()
    {
        let new_needed = ingridient
            .1
            .saturating_sub(*leftovers.entry(ingridient.0).or_insert(0));
        let reaction = reactions.get(ingridient.0).unwrap();
        let num_reactions = (new_needed + reaction.target.count - 1) / reaction.target.count;

        for subingridient in &reaction.ingridients {
            *current_ingridients
                .entry(&subingridient.element)
                .or_insert(0) += subingridient.count * num_reactions;
        }
        let num_should_create = num_reactions * reaction.target.count;

        let leftover = leftovers.entry(&ingridient.0).or_insert(0);
        *leftover = leftover.saturating_sub(ingridient.1);
        *leftover += num_should_create - new_needed;

        current_ingridients.remove(ingridient.0);
    }
    *current_ingridients.get(source).unwrap()
}

fn is_enough_ore(
    reactions: &ReactionsTable,
    source: &str,
    target: &str,
    fuel_multiplier: u64,
    ore: u64,
) -> bool {
    calculate_needed_source(reactions, source, target, fuel_multiplier) <= ore
}

pub fn calculate_max_fuel(reactions: &ReactionsTable, source: &str, target: &str, ore: u64) -> u64 {
    let mut left = 0;
    let mut right = ore;
    while left + 1 < right {
        let mid = (left + right) / 2;
        if is_enough_ore(reactions, source, target, mid, ore) {
            left = mid;
        } else {
            right = mid;
        }
    }
    if is_enough_ore(reactions, source, target, right, ore) {
        right
    } else {
        left
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_test_1() {
        let reactions = parse_reactions(
            "10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL",
        );
        assert_eq!(calculate_needed_source(&reactions, "ORE", "FUEL", 1), 31);
    }

    #[test]
    fn sample_test_2() {
        let reactions = parse_reactions(
            "9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL",
        );
        assert_eq!(calculate_needed_source(&reactions, "ORE", "FUEL", 1), 165);
    }

    #[test]
    fn sample_test_3() {
        let reactions = parse_reactions(
            "157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT",
        );
        assert_eq!(calculate_needed_source(&reactions, "ORE", "FUEL", 1), 13312);
        assert_eq!(
            calculate_max_fuel(&reactions, "ORE", "FUEL", 1000000000000),
            82892753
        );
    }

    #[test]
    fn sample_test_4() {
        let reactions = parse_reactions(
            "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
17 NVRVD, 3 JNWZP => 8 VPVL
53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
22 VJHF, 37 MNCFX => 5 FWMGM
139 ORE => 4 NVRVD
144 ORE => 7 JNWZP
5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
145 ORE => 6 MNCFX
1 NVRVD => 8 CXFTF
1 VJHF, 6 MNCFX => 4 RFSQX
176 ORE => 6 VJHF",
        );
        assert_eq!(
            calculate_needed_source(&reactions, "ORE", "FUEL", 1),
            180697
        );
        assert_eq!(
            calculate_max_fuel(&reactions, "ORE", "FUEL", 1000000000000),
            5586022
        );
    }

    #[test]
    fn sample_test_5() {
        let reactions = parse_reactions(
            "171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX",
        );
        assert_eq!(
            calculate_needed_source(&reactions, "ORE", "FUEL", 1),
            2210736
        );
        assert_eq!(
            calculate_max_fuel(&reactions, "ORE", "FUEL", 1000000000000),
            460664
        );
    }
}
