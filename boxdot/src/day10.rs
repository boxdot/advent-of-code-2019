use itertools::iproduct;
use std::ops::Index;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

pub fn solve(input: &str) -> Result<(usize, usize)> {
    let space = parse(input)?;
    let (station_pos, max_detected) = max_detected(&space).ok_or_else(|| "no asteroids")?;
    let (x, y) = vaporize(&space, station_pos)
        .nth(199)
        .ok_or_else(|| "no 200th vaporized asteroid")?;
    Ok((max_detected, x * 100 + y))
}

type Coord = (usize, usize);

struct Space {
    data: Vec<bool>,
    width: usize,
}

impl Space {
    fn asteroids(&self) -> impl Iterator<Item = Coord> + '_ {
        let height = self.data.len() / self.width;
        iproduct!(0..self.width, 0..height).filter(move |&(x, y)| self.data[x + self.width * y])
    }

    fn is_in_sight(&self, origin: Coord, target: Coord) -> bool {
        line(origin, target).skip(1).find(|&target| self[target]) == Some(target)
    }
}

impl Index<Coord> for Space {
    type Output = bool;
    fn index(&self, (x, y): Coord) -> &Self::Output {
        &self.data[x + y * self.width]
    }
}

fn parse(input: &str) -> Result<Space> {
    let data: Result<_> = input
        .lines()
        .map(|l| {
            l.bytes().map(|c| match c {
                b'.' => Ok(false),
                b'#' => Ok(true),
                _ => Err(format!("unexpected char: {}", c).into()),
            })
        })
        .flatten()
        .collect();
    Ok(Space {
        data: data?,
        width: input.lines().next().ok_or_else(|| "empty input")?.len(),
    })
}

fn max_detected(space: &Space) -> Option<(Coord, usize)> {
    let asteroids: Vec<Coord> = space.asteroids().collect();
    asteroids
        .iter()
        .map(|&origin| {
            let detected = asteroids
                .iter()
                .filter(|&&target| space.is_in_sight(origin, target))
                .count();
            (origin, detected)
        })
        .max_by_key(|&(_, detected)| detected)
}

fn vaporize<'a>(space: &'a Space, station_pos: Coord) -> impl Iterator<Item = Coord> + 'a {
    let angle = |(x0, y0), (x1, y1)| {
        let dx = x1 as f64 - x0 as f64;
        let dy = -(y1 as f64 - y0 as f64); // mirrored y-axis
        let mut angle = dx.atan2(dy);
        while angle < 0.0 {
            angle += 2. * std::f64::consts::PI;
        }
        angle
    };

    let mut asteroids: Vec<Coord> = space.asteroids().collect();
    asteroids.sort_unstable_by(|&target0, &target1| {
        let alpha0 = angle(station_pos, target0);
        let alpha1 = angle(station_pos, target1);
        alpha0.partial_cmp(&alpha1).unwrap()
    });

    std::iter::from_fn(move || {
        asteroids
            .iter()
            .position(|&target| space.is_in_sight(station_pos, target))
            .map(|target_index| asteroids.remove(target_index))
    })
}

fn line((x0, y0): Coord, (x1, y1): Coord) -> impl Iterator<Item = Coord> {
    let mut dx = x1 as isize - x0 as isize;
    let mut dy = y1 as isize - y0 as isize;
    let gcd = num_integer::gcd(dx, dy);
    if gcd != 0 {
        dx /= gcd;
        dy /= gcd;
    }

    std::iter::successors(Some((x0, y0)), move |&(x, y)| {
        if (x, y) == (x1, y1) {
            None
        } else {
            let x = (x as isize + dx) as usize;
            let y = (y as isize + dy) as usize;
            if (x, y) == (x0, y0) {
                None
            } else {
                Some((x, y))
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_detected() {
        let space = parse(
            r#".#..#
.....
#####
....#
...##"#,
        )
        .unwrap();
        assert_eq!(max_detected(&space).unwrap().1, 8);

        let space = parse(
            r#"......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####"#,
        )
        .unwrap();
        assert_eq!(max_detected(&space).unwrap().1, 33);

        let space = parse(
            r#"#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###."#,
        )
        .unwrap();
        assert_eq!(max_detected(&space).unwrap().1, 35);

        let space = parse(
            r#".#..#..###
####.###.#
....###.#.
..###.##.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#.."#,
        )
        .unwrap();
        assert_eq!(max_detected(&space).unwrap().1, 41);

        let space = parse(
            r#".#..##.###...#######
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
###.##.####.##.#..##"#,
        )
        .unwrap();
        assert_eq!(max_detected(&space).unwrap().1, 210);
    }

    #[test]
    fn test_vaporize() {
        let space = parse(
            r#".#....#####...#..
##...##.#####..##
##...#...#.#####.
..#.........###..
..#.#.....#....##"#,
        )
        .unwrap();
        let asteroids: Vec<(usize, usize)> = vaporize(&space, (8, 3)).collect();
        println!("{:?}", asteroids);
        assert_eq!(asteroids[0], (8, 1));
        assert_eq!(asteroids[1], (9, 0));

        let space = parse(
            r#".#..##.###...#######
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
###.##.####.##.#..##"#,
        )
        .unwrap();
        let asteroids: Vec<(usize, usize)> = vaporize(&space, (11, 13)).collect();
        assert_eq!(asteroids[0], (11, 12));
        assert_eq!(asteroids[1], (12, 1));
        assert_eq!(asteroids[2], (12, 2));
    }
}
