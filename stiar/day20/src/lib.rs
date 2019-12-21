#[macro_use]
extern crate itertools;
use derive_more::Add;
use itertools::Itertools;
use ndarray::{Array1, Array2};
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Add)]
pub struct Position {
    pub i: i32,
    pub j: i32,
}

impl Position {
    pub fn new(i: i32, j: i32) -> Self {
        Self { i, j }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Cell {
    Teleport(u32),
    Entrance,
    Exit,
    Empty,
    Wall,
}

fn is_teleport_part(c: char) -> bool {
    c.is_ascii_alphabetic() && c.is_ascii_uppercase()
}

fn get_teleport_code(c1: char, c2: char) -> u32 {
    100 * (c1 as u32 - 'A' as u32) + (c2 as u32 - 'A' as u32)
}

pub fn parse_maze(input: &str) -> (Array2<Cell>, HashMap<Position, Position>) {
    let mut vec: Vec<Vec<_>> = input.split('\n').map(|x| x.chars().collect()).collect();
    if vec.last().unwrap().len() == 0 {
        vec.pop();
    }

    let mut inner_teleports = HashMap::new();
    let mut outer_teleports = HashMap::new();

    let n = vec.len();
    let m = vec[0].len();
    for (i, j) in iproduct!(0..n - 1, 0..m - 1) {
        if is_teleport_part(vec[i][j]) && is_teleport_part(vec[i + 1][j]) {
            match (i, j) {
                (0, _) => {
                    outer_teleports.insert(
                        Position::new(i as i32, (j - 2) as i32),
                        get_teleport_code(vec[i][j], vec[i + 1][j]),
                    );
                }
                (i, j) if i + 2 == n => {
                    outer_teleports.insert(
                        Position::new((i - 3) as i32, (j - 2) as i32),
                        get_teleport_code(vec[i][j], vec[i + 1][j]),
                    );
                }
                (i, j) if vec[i - 1][j] == '.' => {
                    inner_teleports.insert(
                        Position::new((i - 3) as i32, (j - 2) as i32),
                        get_teleport_code(vec[i][j], vec[i + 1][j]),
                    );
                }
                (i, j) if vec[i + 2][j] == '.' => {
                    inner_teleports.insert(
                        Position::new(i as i32, (j - 2) as i32),
                        get_teleport_code(vec[i][j], vec[i + 1][j]),
                    );
                }
                _ => (),
            }
        }
        if is_teleport_part(vec[i][j]) && is_teleport_part(vec[i][j + 1]) {
            match (i, j) {
                (_, 0) => {
                    outer_teleports.insert(
                        Position::new((i - 2) as i32, j as i32),
                        get_teleport_code(vec[i][j], vec[i][j + 1]),
                    );
                }
                (i, j) if j + 2 == m => {
                    outer_teleports.insert(
                        Position::new((i - 2) as i32, (j - 3) as i32),
                        get_teleport_code(vec[i][j], vec[i][j + 1]),
                    );
                }
                (i, j) if vec[i][j - 1] == '.' => {
                    inner_teleports.insert(
                        Position::new((i - 2) as i32, (j - 3) as i32),
                        get_teleport_code(vec[i][j], vec[i][j + 1]),
                    );
                }
                (i, j) if vec[i][j + 2] == '.' => {
                    inner_teleports.insert(
                        Position::new((i - 2) as i32, j as i32),
                        get_teleport_code(vec[i][j], vec[i][j + 1]),
                    );
                }
                _ => (),
            }
        }
    }

    let width = vec.len() - 4;
    let flattened: Array1<_> = vec[2..vec.len() - 2]
        .iter()
        .enumerate()
        .map(|(i, v)| {
            v[2..v.len() - 2]
                .into_iter()
                .enumerate()
                .map(|(j, c)| match (i, j, c) {
                    (i, j, '.')
                        if inner_teleports
                            .get(&Position::new(i as i32, j as i32))
                            .is_some() =>
                    {
                        match *inner_teleports
                            .get(&Position::new(i as i32, j as i32))
                            .unwrap()
                        {
                            0 => Cell::Entrance,
                            2525 => Cell::Exit,
                            c => Cell::Teleport(c),
                        }
                    }
                    (i, j, '.')
                        if outer_teleports
                            .get(&Position::new(i as i32, j as i32))
                            .is_some() =>
                    {
                        match *outer_teleports
                            .get(&Position::new(i as i32, j as i32))
                            .unwrap()
                        {
                            0 => Cell::Entrance,
                            2525 => Cell::Exit,
                            c => Cell::Teleport(c),
                        }
                    }
                    (_, _, '.') => Cell::Empty,
                    _ => Cell::Wall,
                })
                .collect_vec()
        })
        .flatten()
        .collect();
    let height = flattened.len() / width;
    let char_maze = flattened.into_shape((width, height)).unwrap();
    let inverted_inner_teleports: HashMap<_, _> =
        inner_teleports.iter().map(|(k, v)| (v, k)).collect();
    let inverted_outer_teleports: HashMap<_, _> =
        outer_teleports.iter().map(|(k, v)| (v, k)).collect();

    (
        char_maze,
        inner_teleports
            .iter()
            .filter_map(|(&k, &v)| Some((k, **inverted_outer_teleports.get(&v)?)))
            .chain(
                outer_teleports
                    .iter()
                    .filter_map(|(&k, &v)| Some((k, **inverted_inner_teleports.get(&v)?))),
            )
            .collect(),
    )
}

fn is_valid(v: &Position, map: &Array2<Cell>) -> bool {
    v.i >= 0 && (v.i as usize) < map.nrows() && v.j >= 0 && (v.j as usize) < map.ncols()
}

fn is_outer_teleport(v: &Position, map: &Array2<Cell>) -> bool {
    v.i == 0 || (v.i as usize + 1) == map.nrows() || v.j == 0 || (v.j as usize + 1) == map.ncols()
}

pub fn calculate_shortest_path(
    maze: &Array2<Cell>,
    teleports: &HashMap<Position, Position>,
    track_levels: bool,
) -> Option<u32> {
    let entrance = iproduct!(0..maze.nrows(), 0..maze.ncols())
        .filter_map(|x| match maze[x] {
            Cell::Entrance => Some(Position::new(x.0 as i32, x.1 as i32)),
            _ => None,
        })
        .next()
        .unwrap();

    let mut used = HashSet::new();
    let mut queue = VecDeque::new();

    queue.push_back((entrance, 0, 0));
    used.insert((entrance, 0));

    while !queue.is_empty() {
        let (v, level, distance) = queue.pop_front().unwrap();
        if maze[(v.i as usize, v.j as usize)] == Cell::Exit && level == 0 {
            return Some(distance);
        }

        for dir in [(0, 1), (1, 0), (0, -1), (-1, 0)]
            .into_iter()
            .map(|&(di, dj)| Position::new(di, dj))
        {
            let u = v + dir;
            if is_valid(&u, maze)
                && !used.contains(&(u, level))
                && match maze[(u.i as usize, u.j as usize)] {
                    Cell::Wall => false,
                    _ => true,
                }
            {
                queue.push_back((u, level, distance + 1));
                used.insert((u, level));
            }
        }

        if let Some(&to) = teleports.get(&v) {
            if track_levels && is_outer_teleport(&v, maze) && level == 0 {
                continue;
            }
            let new_level = if track_levels {
                if is_outer_teleport(&v, maze) {
                    level - 1
                } else {
                    level + 1
                }
            } else {
                level
            };
            if !used.contains(&(to, new_level)) {
                queue.push_back((to, new_level, distance + 1));
                used.insert((to, new_level));
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_test_1() {
        let (maze, teleports) = parse_maze(
            "         A           
         A           
  #######.#########  
  #######.........#  
  #######.#######.#  
  #######.#######.#  
  #######.#######.#  
  #####  B    ###.#  
BC...##  C    ###.#  
  ##.##       ###.#  
  ##...DE  F  ###.#  
  #####    G  ###.#  
  #########.#####.#  
DE..#######...###.#  
  #.#########.###.#  
FG..#########.....#  
  ###########.#####  
             Z       
             Z       ",
        );
        assert_eq!(calculate_shortest_path(&maze, &teleports, false), Some(23));
        assert_eq!(calculate_shortest_path(&maze, &teleports, true), Some(26));
    }

    #[test]
    fn sample_test_2() {
        let (maze, teleports) = parse_maze(
            "                   A               
                   A               
  #################.#############  
  #.#...#...................#.#.#  
  #.#.#.###.###.###.#########.#.#  
  #.#.#.......#...#.....#.#.#...#  
  #.#########.###.#####.#.#.###.#  
  #.............#.#.....#.......#  
  ###.###########.###.#####.#.#.#  
  #.....#        A   C    #.#.#.#  
  #######        S   P    #####.#  
  #.#...#                 #......VT
  #.#.#.#                 #.#####  
  #...#.#               YN....#.#  
  #.###.#                 #####.#  
DI....#.#                 #.....#  
  #####.#                 #.###.#  
ZZ......#               QG....#..AS
  ###.###                 #######  
JO..#.#.#                 #.....#  
  #.#.#.#                 ###.#.#  
  #...#..DI             BU....#..LF
  #####.#                 #.#####  
YN......#               VT..#....QG
  #.###.#                 #.###.#  
  #.#...#                 #.....#  
  ###.###    J L     J    #.#.###  
  #.....#    O F     P    #.#...#  
  #.###.#####.#.#####.#####.###.#  
  #...#.#.#...#.....#.....#.#...#  
  #.#####.###.###.#.#.#########.#  
  #...#.#.....#...#.#.#.#.....#.#  
  #.###.#####.###.###.#.#.#######  
  #.#.........#...#.............#  
  #########.###.###.#############  
           B   J   C               
           U   P   P               ",
        );
        assert_eq!(calculate_shortest_path(&maze, &teleports, false), Some(58));
    }

    #[test]
    fn sample_test_3() {
        let (maze, teleports) = parse_maze(
            "             Z L X W       C                 
             Z P Q B       K                 
  ###########.#.#.#.#######.###############  
  #...#.......#.#.......#.#.......#.#.#...#  
  ###.#.#.#.#.#.#.#.###.#.#.#######.#.#.###  
  #.#...#.#.#...#.#.#...#...#...#.#.......#  
  #.###.#######.###.###.#.###.###.#.#######  
  #...#.......#.#...#...#.............#...#  
  #.#########.#######.#.#######.#######.###  
  #...#.#    F       R I       Z    #.#.#.#  
  #.###.#    D       E C       H    #.#.#.#  
  #.#...#                           #...#.#  
  #.###.#                           #.###.#  
  #.#....OA                       WB..#.#..ZH
  #.###.#                           #.#.#.#  
CJ......#                           #.....#  
  #######                           #######  
  #.#....CK                         #......IC
  #.###.#                           #.###.#  
  #.....#                           #...#.#  
  ###.###                           #.#.#.#  
XF....#.#                         RF..#.#.#  
  #####.#                           #######  
  #......CJ                       NM..#...#  
  ###.#.#                           #.###.#  
RE....#.#                           #......RF
  ###.###        X   X       L      #.#.#.#  
  #.....#        F   Q       P      #.#.#.#  
  ###.###########.###.#######.#########.###  
  #.....#...#.....#.......#...#.....#.#...#  
  #####.#.###.#######.#######.###.###.#.#.#  
  #.......#.......#.#.#.#.#...#...#...#.#.#  
  #####.###.#####.#.#.#.#.###.###.#.###.###  
  #.......#.....#.#...#...............#...#  
  #############.#.#.###.###################  
               A O F   N                     
               A A D   M                     ",
        );
        assert_eq!(calculate_shortest_path(&maze, &teleports, true), Some(396));
    }
}
