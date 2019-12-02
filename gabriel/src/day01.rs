#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn required_fuel_test() {
        assert_eq!(required_fuel(12), Some(2));
        assert_eq!(required_fuel(14), Some(2));
        assert_eq!(required_fuel(1969), Some(654));
        assert_eq!(required_fuel(100756), Some(33583));
    }

    #[test]
    fn actual_required_fuel_test() {
        assert_eq!(total_required_fuel(14), 2);
        assert_eq!(total_required_fuel(1969), 966);
        assert_eq!(total_required_fuel(100756), 50346);
    }
}

fn required_fuel(mass: usize) -> Option<usize> {
    (mass / 3).checked_sub(2)
}

fn total_required_fuel(mass: usize) -> usize {
    std::iter::successors(required_fuel(mass), |m| required_fuel(*m)).sum()
}

pub fn unlock(input: &str) -> Result<(usize, usize), Box<dyn std::error::Error>> {
    let masses = input.lines().map(|l| l.parse().unwrap());

    let required_fuel = masses.clone().filter_map(required_fuel).sum();

    let total_required_fuel = masses.map(total_required_fuel).sum();

    Ok((required_fuel, total_required_fuel))
}
