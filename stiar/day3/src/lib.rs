use std::collections::HashMap;
use std::ops::AddAssign;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Action {
    Left(u32),
    Right(u32),
    Up(u32),
    Down(u32),
}

pub fn parse_actions(actions_str: &str) -> Vec<Action> {
    actions_str
        .split(',')
        .map(|action_str| match &action_str[..1] {
            "L" => Action::Left(action_str[1..].parse().unwrap()),
            "R" => Action::Right(action_str[1..].parse().unwrap()),
            "U" => Action::Up(action_str[1..].parse().unwrap()),
            "D" => Action::Down(action_str[1..].parse().unwrap()),
            _ => panic!("Unknown action"),
        })
        .collect()
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Mode {
    Manhattan,
    WireLength,
}

pub fn find_closest_intersection_distance(
    first_wire: &[Action],
    second_wire: &[Action],
    mode: Mode,
) -> u32 {
    let first_wire_map = follow_wire(first_wire, mode);
    let second_wire_map = follow_wire(second_wire, mode);
    first_wire_map
        .iter()
        .map(|(key, &value)| {
            if let Some(&second_value) = second_wire_map.get(key) {
                match mode {
                    Mode::Manhattan => Some(value),
                    Mode::WireLength => Some(value + second_value),
                }
            } else {
                None
            }
        })
        .flatten()
        .min()
        .unwrap()
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

fn follow_wire(wire: &[Action], mode: Mode) -> HashMap<Point, u32> {
    let mut visited_points = HashMap::new();
    let mut current_point = Point { x: 0, y: 0 };
    let mut total_length = 0;
    wire.iter().for_each(|action| {
        let (direction, amount) = match action {
            Action::Left(amount) => (Point { x: -1, y: 0 }, *amount),
            Action::Right(amount) => (Point { x: 1, y: 0 }, *amount),
            Action::Up(amount) => (Point { x: 0, y: 1 }, *amount),
            Action::Down(amount) => (Point { x: 0, y: -1 }, *amount),
        };
        for _ in 1..=amount {
            total_length += 1;
            current_point += direction;
            visited_points.insert(
                current_point,
                match mode {
                    Mode::Manhattan => (current_point.x.abs() + current_point.y.abs()) as u32,
                    Mode::WireLength => total_length,
                },
            );
        }
    });
    visited_points
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_actions_test() {
        assert_eq!(
            parse_actions("R75,D30,U83,L12"),
            vec![
                Action::Right(75),
                Action::Down(30),
                Action::Up(83),
                Action::Left(12)
            ]
        )
    }

    #[test]
    fn sample_test_1() {
        assert_eq!(
            find_closest_intersection_distance(
                &parse_actions("R8,U5,L5,D3"),
                &parse_actions("U7,R6,D4,L4"),
                Mode::Manhattan
            ),
            6
        );
    }

    #[test]
    fn sample_test_2() {
        assert_eq!(
            find_closest_intersection_distance(
                &parse_actions("R75,D30,R83,U83,L12,D49,R71,U7,L72"),
                &parse_actions("U62,R66,U55,R34,D71,R55,D58,R83"),
                Mode::Manhattan
            ),
            159
        );
    }

    #[test]
    fn sample_test_3() {
        assert_eq!(
            find_closest_intersection_distance(
                &parse_actions("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51"),
                &parse_actions("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"),
                Mode::Manhattan
            ),
            135
        );
    }

    #[test]
    fn sample_test_4() {
        assert_eq!(
            find_closest_intersection_distance(
                &parse_actions("R75,D30,R83,U83,L12,D49,R71,U7,L72"),
                &parse_actions("U62,R66,U55,R34,D71,R55,D58,R83"),
                Mode::WireLength
            ),
            610
        );
    }

    #[test]
    fn sample_test_5() {
        assert_eq!(
            find_closest_intersection_distance(
                &parse_actions("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51"),
                &parse_actions("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"),
                Mode::WireLength
            ),
            410
        );
    }
}
