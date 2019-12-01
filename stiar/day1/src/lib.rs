pub fn calculate_fuel_for_modules(modules: &[u32], include_fuel_for_fuel: bool) -> u32 {
    modules
        .iter()
        .map(|weight| {
            let fuel_for_weight = calculate_fuel_for_weight(*weight);
            if include_fuel_for_fuel {
                fuel_for_weight + calculate_fuel_for_fuel(fuel_for_weight)
            } else {
                fuel_for_weight
            }
        })
        .sum()
}

fn calculate_fuel_for_fuel(mut fuel: u32) -> u32 {
    std::iter::from_fn(|| {
        fuel = calculate_fuel_for_weight(fuel);
        if fuel > 0 {
            Some(fuel)
        } else {
            None
        }
    })
    .sum()
}

fn calculate_fuel_for_weight(weight: u32) -> u32 {
    std::cmp::max(weight / 3, 2) - 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fuel_for_modules_without_fuel_weight() {
        assert_eq!(
            calculate_fuel_for_modules(&vec![12, 14, 1969, 100756], false),
            2 + 2 + 654 + 33583
        );
    }

    #[test]
    fn fuel_for_modules_with_fuel_weight() {
        assert_eq!(
            calculate_fuel_for_modules(&vec![14, 1969, 100756], true),
            2 + 966 + 50346
        );
    }
}
