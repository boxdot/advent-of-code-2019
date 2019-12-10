use num::Integer;

use std::collections::HashSet;
use std::ops::AddAssign;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Cell {
    pub x: i32,
    pub y: i32,
}

impl AddAssign for Cell {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
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
                    x: row as i32,
                    y: col as i32,
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

pub fn find_best_monitoring_station(asteroids: &HashSet<Cell>) -> u32 {
    asteroids
        .iter()
        .map(|&asteroid| {
            asteroids
                .iter()
                .filter(|&&target| {
                    asteroid != target
                        && LineCellsIterator::new(asteroid, target)
                            .all(|cell| !asteroids.contains(&cell))
                })
                .count()
        })
        .max()
        .unwrap() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            parse_from_map(
                ".#..#\n\
                 .....\n\
                 #####\n\
                 ....#\n\
                 ...##"
            ),
            [
                Cell { x: 0, y: 1 },
                Cell { x: 0, y: 4 },
                Cell { x: 2, y: 0 },
                Cell { x: 2, y: 1 },
                Cell { x: 2, y: 2 },
                Cell { x: 2, y: 3 },
                Cell { x: 2, y: 4 },
                Cell { x: 3, y: 4 },
                Cell { x: 4, y: 3 },
                Cell { x: 4, y: 4 },
            ]
            .iter()
            .cloned()
            .collect::<HashSet<_>>()
        );
    }

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
    fn sample_test_1() {
        assert_eq!(
            find_best_monitoring_station(&parse_from_map(
                ".#..#\n\
                 .....\n\
                 #####\n\
                 ....#\n\
                 ...##"
            )),
            8
        );
    }

    #[test]
    fn sample_test_2() {
        assert_eq!(
            find_best_monitoring_station(&parse_from_map(
                "......#.#.\n\
                 #..#.#....\n\
                 ..#######.\n\
                 .#.#.###..\n\
                 .#..#.....\n\
                 ..#....#.#\n\
                 #..#....#.\n\
                 .##.#..###\n\
                 ##...#..#.\n\
                 .#....####"
            )),
            33
        );
    }

    #[test]
    fn sample_test_3() {
        assert_eq!(
            find_best_monitoring_station(&parse_from_map(
                ".#..##.###...#######\n\
                 ##.############..##.\n\
                 .#.######.########.#\n\
                 .###.#######.####.#.\n\
                 #####.##.#.##.###.##\n\
                 ..#####..#.#########\n\
                 ####################\n\
                 #.####....###.#.#.##\n\
                 ##.#################\n\
                 #####.##.###..####..\n\
                 ..######..##.#######\n\
                 ####.##.####...##..#\n\
                 .#####..#.######.###\n\
                 ##...#.##########...\n\
                 #.##########.#######\n\
                 .####.#.###.###.#.##\n\
                 ....##.##.###..#####\n\
                 .#.#.###########.###\n\
                 #.#.#.#####.####.###\n\
                 ###.##.####.##.#..##"
            )),
            210
        );
    }
}
