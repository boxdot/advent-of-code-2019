use std::collections::HashMap;

pub fn solve(input: &str) -> Option<(usize, usize)> {
    let mut lines = input.lines();
    let wire1 = parse(lines.next()?);
    let wire2 = parse(lines.next()?);

    let grid = build_grid(&wire1, &wire2);
    Some((part1(&grid)?, part2(&grid)?))
}

fn parse(s: &str) -> Vec<(Direction, usize)> {
    s.split(',')
        .filter_map(|s| {
            let direction = Direction::from_char(s.chars().next()?)?;
            let steps = s[1..].parse().ok()?;
            Some((direction, steps))
        })
        .collect()
}

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn from_char(c: char) -> Option<Direction> {
        match c {
            'U' => Some(Direction::Up),
            'D' => Some(Direction::Down),
            'L' => Some(Direction::Left),
            'R' => Some(Direction::Right),
            _ => None,
        }
    }
}

fn follow_wire(wire: &[(Direction, usize)], mut action: impl FnMut(isize, isize, usize)) {
    let mut step = 0;
    let mut pos_x = 0;
    let mut pos_y = 0;
    for (direction, steps) in wire {
        for _ in 0..*steps {
            action(pos_x, pos_y, step);
            match direction {
                Direction::Up => pos_y += 1,
                Direction::Down => pos_y -= 1,
                Direction::Left => pos_x -= 1,
                Direction::Right => pos_x += 1,
            }
            step += 1;
        }
    }
}

fn build_grid(
    wire1: &[(Direction, usize)],
    wire2: &[(Direction, usize)],
) -> HashMap<(isize, isize), (Option<usize>, Option<usize>)> {
    let mut grid = HashMap::new();
    follow_wire(&wire1, |x, y, step| {
        grid.entry((x, y))
            .and_modify(|(steps1, _): &mut (Option<usize>, Option<usize>)| {
                steps1.get_or_insert(step);
            })
            .or_insert((Some(step), None));
    });
    follow_wire(&wire2, |x, y, step| {
        grid.entry((x, y))
            .and_modify(|(_, steps2)| {
                steps2.get_or_insert(step);
            })
            .or_insert((None, Some(step)));
    });
    grid
}

fn part1(grid: &HashMap<(isize, isize), (Option<usize>, Option<usize>)>) -> Option<usize> {
    grid.into_iter()
        .filter(|&((x, y), (wire1, wire2))| {
            wire1.is_some() && wire2.is_some() && (*x != 0 || *y != 0)
        })
        .map(|((x, y), _)| (x.abs() + y.abs()) as usize)
        .min()
}

fn part2(grid: &HashMap<(isize, isize), (Option<usize>, Option<usize>)>) -> Option<usize> {
    grid.into_iter()
        .filter(|&((x, y), (wire1, wire2))| {
            wire1.is_some() && wire2.is_some() && (*x != 0 || *y != 0)
        })
        .map(|(_, (steps1, steps2))| steps1.unwrap() + steps2.unwrap())
        .min()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let wire1 = parse("R8,U5,L5,D3");
        let wire2 = parse("U7,R6,D4,L4");
        assert_eq!(part1(&build_grid(&wire1, &wire2)), Some(6));

        let wire1 = parse("R75,D30,R83,U83,L12,D49,R71,U7,L72");
        let wire2 = parse("U62,R66,U55,R34,D71,R55,D58,R83");
        assert_eq!(part1(&build_grid(&wire1, &wire2)), Some(159));

        let wire1 = parse("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51");
        let wire2 = parse("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7");
        assert_eq!(part1(&build_grid(&wire1, &wire2)), Some(135));
    }

    #[test]
    fn test_part2() {
        let wire1 = parse("R8,U5,L5,D3");
        let wire2 = parse("U7,R6,D4,L4");
        assert_eq!(part2(&build_grid(&wire1, &wire2)), Some(30));

        let wire1 = parse("R75,D30,R83,U83,L12,D49,R71,U7,L72");
        let wire2 = parse("U62,R66,U55,R34,D71,R55,D58,R83");
        assert_eq!(part2(&build_grid(&wire1, &wire2)), Some(610));

        let wire1 = parse("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51");
        let wire2 = parse("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7");
        assert_eq!(part2(&build_grid(&wire1, &wire2)), Some(410));
    }
}
