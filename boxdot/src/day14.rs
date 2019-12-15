use std::collections::HashMap;

pub fn solve(input: &str) -> (usize, usize) {
    let rules = parse(input);
    let part1 = ore(&rules, 1);
    let part2 = optimal_fuel(&rules);
    (part1, part2)
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Term {
    label: String,
    mult: usize,
}

impl Term {
    fn new(label: &str, mult: usize) -> Self {
        Self {
            label: label.to_string(),
            mult,
        }
    }
}

type Rules = HashMap<String, (usize, Vec<Term>)>;

fn parse(input: &str) -> Rules {
    let parse_term = |s: &str| {
        let mut parts = s.trim().split(' ');
        let quantity: usize = parts.next().unwrap().parse().unwrap();
        let label = parts.next().unwrap();
        Term::new(label, quantity)
    };

    input
        .lines()
        .map(|l| {
            let mut parts = l.split("=>");
            let lhs = parts.next().unwrap();
            let rhs = parts.next().unwrap();
            let target = parse_term(rhs);
            let terms: Vec<_> = lhs.split(',').map(parse_term).collect();
            (target.label, (target.mult, terms))
        })
        .collect()
}

fn ore(rules: &Rules, fuel_amount: usize) -> usize {
    let mut amounts = HashMap::new();
    amounts.insert("FUEL".to_string(), fuel_amount as isize);

    while let Some((label, amount)) = amounts
        .iter()
        .find(|&(label, &amount)| label != "ORE" && amount > 0)
        .map(|(label, &amount)| (label.clone(), amount))
    {
        let (rule_mult, terms) = rules.get(&label).unwrap();
        let rule_mult = *rule_mult as isize;
        let mult = (rule_mult + amount - 1) / rule_mult;
        amounts.insert(label, amount - mult * rule_mult);
        for term in terms {
            let value = amounts.entry(term.label.clone()).or_insert(0);
            *value += mult * term.mult as isize;
        }
    }

    *amounts.get("ORE").unwrap() as usize
}

fn optimal_fuel(rules: &Rules) -> usize {
    const MAX_ORE: usize = 1_000_000_000_000;
    let mut min = 1;
    let mut max = MAX_ORE;
    // binary search for the optimal amount of fuel
    while min + 1 < max {
        let mid = (min + max) / 2;
        let ore = ore(rules, mid);
        if ore < MAX_ORE {
            min = mid;
        } else if ore > MAX_ORE {
            max = mid;
        } else {
            return mid;
        }
    }
    min
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ore() {
        let rules = parse(
            r#"10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL"#,
        );
        assert_eq!(ore(&rules, 1), 31);

        let rules = parse(
            r#"9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL"#,
        );
        assert_eq!(ore(&rules, 1), 165);

        let rules = parse(
            r#"157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT"#,
        );
        assert_eq!(ore(&rules, 1), 13312);

        let rules = parse(
            r#"2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
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
176 ORE => 6 VJHF"#,
        );
        assert_eq!(ore(&rules, 1), 180697);

        let rules = parse(
            r#"171 ORE => 8 CNZTR
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
5 BHXH, 4 VRPVC => 5 LTCX"#,
        );
        assert_eq!(ore(&rules, 1), 2210736);
    }

    #[test]
    fn test_optimal_fuel() {
        let rules = parse(
            r#"157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT"#,
        );
        assert_eq!(optimal_fuel(&rules), 82892753);

        let rules = parse(
            r#"2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
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
176 ORE => 6 VJHF"#,
        );
        assert_eq!(optimal_fuel(&rules), 5586022);

        let rules = parse(
            r#"171 ORE => 8 CNZTR
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
5 BHXH, 4 VRPVC => 5 LTCX"#,
        );
        assert_eq!(optimal_fuel(&rules), 460664);
    }
}
