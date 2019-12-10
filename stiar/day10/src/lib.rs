use num::Integer;
use derive_more::{AddAssign, Sub};

use std::collections::HashSet;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, AddAssign, Sub)]
pub struct Cell {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct AngleOrder {
    pub group: u32,
    pub ratio: num::Rational,
    pub len: u32,
}

impl Cell {
    pub fn get_order(&self) -> AngleOrder {
        let ratio = if self.x == 0 || self.y == 0 {
            num::Rational::new(0, 1)
        } else {
            num::Rational::new(self.x as isize, self.y as isize)
        };
        let len = if self.x == 0 {
            self.y.abs() as u32
        } else {
            self.x.abs() as u32
        };
        let group = match (self.x, self.y) {
            (0, 0) => 0,
            (0, y) if y > 0 => 1,
            (x, y) if x > 0 && y > 0 => 2,
            (x, 0) if x > 0 => 3,
            (x, y) if x > 0 && y < 0 => 4,
            (0, y) if y < 0 => 5,
            (x, y) if x < 0 && y < 0 => 6,
            (x, 0) if x < 0 => 7,
            (x, y) if x < 0 && y > 0 => 8,
            _ => panic!("WAT"),
        };
        AngleOrder { group, ratio, len }
    }
}

pub fn parse_from_map(map: &str) -> HashSet<Cell> {
    map.split('\n')
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, cell)| *cell == '#')
                .map(|(col, _)| Cell {
                    x: col as i32,
                    y: -(row as i32),
                })
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect()
}

fn calculate_step(start: &Cell, end: &Cell) -> Cell {
    let dx = end.x - start.x;
    let dy = end.y - start.y;
    let gcd = dx.gcd(&dy);
    Cell {
        x: dx / gcd,
        y: dy / gcd,
    }
}

fn are_blocking(start: &Cell, first: &Cell, second: &Cell) -> bool {
    calculate_step(start, first) == calculate_step(start, second)
}

struct LineCellsIterator {
    current_cell: Cell,
    target_cell: Cell,
    step: Cell,
}

impl LineCellsIterator {
    pub fn new(start_cell: Cell, target_cell: Cell) -> Self {
        Self {
            current_cell: start_cell,
            target_cell: target_cell,
            step: calculate_step(&start_cell, &target_cell),
        }
    }
}

impl Iterator for LineCellsIterator {
    type Item = Cell;

    fn next(&mut self) -> Option<Cell> {
        if self.current_cell == self.target_cell {
            return None;
        }

        self.current_cell += self.step;

        if self.current_cell != self.target_cell {
            Some(self.current_cell)
        } else {
            None
        }
    }
}

pub fn find_best_monitoring_station(asteroids: &HashSet<Cell>) -> (Cell, usize) {
    let max = asteroids
        .iter()
        .map(|&asteroid| {
            (
                asteroid,
                asteroids
                    .iter()
                    .filter(|&&target| {
                        asteroid != target
                            && LineCellsIterator::new(asteroid, target)
                                .all(|cell| !asteroids.contains(&cell))
                    })
                    .count(),
            )
        })
        .max_by_key(|(_, count)| count.clone());
    max.unwrap()
}

pub struct LaserIterator {
    asteroids: Vec<Cell>,
    center: Cell,
    current_index: usize,
    last_output: Option<Cell>,
}

impl LaserIterator {
    pub fn new(asteroids: &HashSet<Cell>, center: Cell) -> Self {
        let mut vec = asteroids
            .iter()
            .filter(|&&asteroid| asteroid != center)
            .cloned()
            .collect::<Vec<_>>();
        vec.sort_by(|&a, &b| (a - center).get_order().cmp(&(b - center).get_order()));

        Self {
            asteroids: vec,
            center: center,
            current_index: 0,
            last_output: None,
        }
    }
}

impl Iterator for LaserIterator {
    type Item = Cell;

    fn next(&mut self) -> Option<Cell> {
        loop {
            if self.asteroids.is_empty() {
                return None;
            }

            if self.last_output.is_none() {
                self.last_output = Some(self.asteroids[self.current_index]);
                self.asteroids.remove(self.current_index);
                return self.last_output;
            }
            while self.current_index < self.asteroids.len()
                && are_blocking(
                    &self.center,
                    &self.last_output.unwrap(),
                    &self.asteroids[self.current_index],
                )
            {
                self.current_index += 1;
            }

            if self.current_index < self.asteroids.len() {
                self.last_output = Some(self.asteroids[self.current_index]);
                self.asteroids.remove(self.current_index);
                return self.last_output;
            } else {
                self.current_index = 0;
                self.last_output = None;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_step() {
        assert_eq!(
            calculate_step(&Cell { x: 0, y: 0 }, &Cell { x: 4, y: 2 }),
            Cell { x: 2, y: 1 }
        );

        assert_eq!(
            calculate_step(&Cell { x: 0, y: 0 }, &Cell { x: 0, y: 2 }),
            Cell { x: 0, y: 1 }
        );

        assert_eq!(
            calculate_step(&Cell { x: 2, y: 2 }, &Cell { x: -4, y: -4 }),
            Cell { x: -1, y: -1 }
        );
    }

    #[test]
    fn test_blocking() {
        assert!(are_blocking(
            &Cell { x: 0, y: 0 },
            &Cell { x: 4, y: 6 },
            &Cell { x: 6, y: 9 }
        ));
        assert!(!are_blocking(
            &Cell { x: 0, y: 0 },
            &Cell { x: -4, y: -6 },
            &Cell { x: 6, y: 9 }
        ));
    }

    #[test]
    fn test_order() {
        let sorted = [
            Cell { x: 0, y: 1 },
            Cell { x: 0, y: 2 },
            Cell { x: 1, y: 2 },
            Cell { x: 2, y: 4 },
            Cell { x: 2, y: 3 },
            Cell { x: 2, y: 2 },
            Cell { x: 3, y: 3 },
            Cell { x: 5, y: 5 },
            Cell { x: 1, y: 0 },
            Cell { x: 2, y: 0 },
            Cell { x: 3, y: -1 },
            Cell { x: 2, y: -1 },
            Cell { x: 3, y: -2 },
            Cell { x: 1, y: -1 },
            Cell { x: 0, y: -1 },
            Cell { x: 0, y: -2 },
            Cell { x: -1, y: -2 },
            Cell { x: -2, y: -2 },
            Cell { x: -2, y: -1 },
            Cell { x: -1, y: 0 },
            Cell { x: -2, y: 0 },
            Cell { x: -2, y: 1 },
            Cell { x: -2, y: 2 },
            Cell { x: -1, y: 2 },
        ];

        for i in 0..sorted.len() - 1 {
            for j in i + 1..sorted.len() {
                assert!(sorted[i].get_order() < sorted[j].get_order());
                assert!(!(sorted[i].get_order() > sorted[j].get_order()));
            }
        }
    }

    #[test]
    fn sample_test_1() {
        assert_eq!(
            find_best_monitoring_station(&parse_from_map(
                ".#..#
.....
#####
....#
...##"
            ))
            .1,
            8
        );
    }

    #[test]
    fn sample_test_2() {
        assert_eq!(
            find_best_monitoring_station(&parse_from_map(
                "......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####"
            ))
            .1,
            33
        );
    }

    #[test]
    fn sample_test_3() {
        assert_eq!(
            find_best_monitoring_station(&parse_from_map(
                ".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##"
            ))
            .1,
            210
        );
    }

    #[test]
    fn sample_test_4() {
        let asteroids = parse_from_map(
            ".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##",
        );

        let center = find_best_monitoring_station(&asteroids).0;

        assert_eq!(
            LaserIterator::new(&asteroids, center).nth(0),
            Some(Cell { x: 11, y: -12 })
        );
        assert_eq!(
            LaserIterator::new(&asteroids, center).nth(1),
            Some(Cell { x: 12, y: -1 })
        );
        assert_eq!(
            LaserIterator::new(&asteroids, center).nth(2),
            Some(Cell { x: 12, y: -2 })
        );
        assert_eq!(
            LaserIterator::new(&asteroids, center).nth(9),
            Some(Cell { x: 12, y: -8 })
        );
        assert_eq!(
            LaserIterator::new(&asteroids, center).nth(19),
            Some(Cell { x: 16, y: 0 })
        );
        assert_eq!(
            LaserIterator::new(&asteroids, center).nth(49),
            Some(Cell { x: 16, y: -9 })
        );
        assert_eq!(
            LaserIterator::new(&asteroids, center).nth(99),
            Some(Cell { x: 10, y: -16 })
        );
        assert_eq!(
            LaserIterator::new(&asteroids, center).nth(198),
            Some(Cell { x: 9, y: -6 })
        );
        assert_eq!(
            LaserIterator::new(&asteroids, center).nth(199),
            Some(Cell { x: 8, y: -2 })
        );
        assert_eq!(
            LaserIterator::new(&asteroids, center).nth(200),
            Some(Cell { x: 10, y: -9 })
        );
        assert_eq!(
            LaserIterator::new(&asteroids, center).nth(298),
            Some(Cell { x: 11, y: -1 })
        );
        assert_eq!(
            LaserIterator::new(&asteroids, center).nth(299),
            None
        );
    }
}
