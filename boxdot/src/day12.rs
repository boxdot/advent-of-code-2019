use num_integer::lcm;
use std::collections::{hash_map::Entry, HashMap};

pub fn solve(input: &str) -> (i64, usize) {
    let init_config = parse(input);
    let config = simulate_motion(&init_config).nth(1_000).unwrap();
    (total_energy(&config), period(&init_config))
}

fn parse(input: &str) -> Configuration {
    let positions = input.lines().map(|l| {
        let mut coords = l[1..l.len() - 1].split(',').map(|elem| {
            let coord: i64 = elem.trim().split('=').nth(1).unwrap().parse().unwrap();
            coord
        });
        let x = coords.next().unwrap();
        let y = coords.next().unwrap();
        let z = coords.next().unwrap();
        [x, y, z]
    });

    let mut pos = [[0; 4]; 3];
    for (i, p) in positions.enumerate() {
        for axis in 0..3 {
            pos[axis][i] = p[axis];
        }
    }
    let vel = [[0; 4]; 3];

    Configuration { pos, vel }
}

// we store transposed matrix since we simulate each axis independently
#[derive(Debug, Clone, PartialEq, Eq)]
struct Configuration {
    pos: [[i64; 4]; 3],
    vel: [[i64; 4]; 3],
}

fn simulate_motion_for_axis(
    pos: [i64; 4],
    vel: [i64; 4],
) -> impl Iterator<Item = ([i64; 4], [i64; 4])> {
    std::iter::successors(Some((pos, vel)), |(pos, vel)| {
        let mut pos = *pos;
        let mut vel = *vel;
        for i in 0..4 {
            for j in i + 1..4 {
                if pos[i] < pos[j] {
                    vel[i] += 1;
                    vel[j] -= 1;
                } else if pos[i] > pos[j] {
                    vel[i] -= 1;
                    vel[j] += 1;
                }
            }
        }
        for i in 0..4 {
            pos[i] += vel[i];
        }
        Some((pos, vel))
    })
}

fn simulate_motion(config: &Configuration) -> impl Iterator<Item = Configuration> {
    let motion_x = simulate_motion_for_axis(config.pos[0], config.vel[0]);
    let motion_y = simulate_motion_for_axis(config.pos[1], config.vel[1]);
    let motion_z = simulate_motion_for_axis(config.pos[2], config.vel[2]);
    motion_x.zip(motion_y.zip(motion_z)).map(
        |((pos_x, vel_x), ((pos_y, vel_y), (pos_z, vel_z)))| Configuration {
            pos: [pos_x, pos_y, pos_z],
            vel: [vel_x, vel_y, vel_z],
        },
    )
}

fn total_energy(config: &Configuration) -> i64 {
    (0..4)
        .map(|i| {
            let pot: i64 = config.pos.iter().map(|x| x[i].abs()).sum();
            let kin: i64 = config.vel.iter().map(|x| x[i].abs()).sum();
            pot * kin
        })
        .sum()
}

/// Finds and returns fixpoint as (offset, period).
fn fixpoint(config: &Configuration, axis: usize) -> (usize, usize) {
    let mut states = HashMap::new();
    for (i, (pos, vel)) in simulate_motion_for_axis(config.pos[axis], config.vel[axis]).enumerate()
    {
        match states.entry((pos, vel)) {
            Entry::Occupied(entry) => {
                let offset = *entry.get();
                let period = i - offset;
                return (offset, period);
            }
            Entry::Vacant(entry) => {
                entry.insert(i);
            }
        }
    }
    unreachable!();
}

fn period(config: &Configuration) -> usize {
    let (offset_x, period_x) = fixpoint(&config, 0);
    let (offset_y, period_y) = fixpoint(&config, 1);
    let (offset_z, period_z) = fixpoint(&config, 2);
    assert_eq!(offset_x, offset_y);
    assert_eq!(offset_y, offset_z);
    lcm(lcm(period_x, period_y), period_z)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_energy_and_period() {
        let init_config = parse(
            r#"<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>"#,
        );
        let config = simulate_motion(&init_config).nth(10).unwrap();
        assert_eq!(
            config,
            Configuration {
                pos: [[2, 1, 3, 2], [1, -8, -6, 0], [-3, 0, 1, 4]],
                vel: [[-3, -1, 3, 1], [-2, 1, 2, -1], [1, 3, -3, -1]]
            }
        );
        assert_eq!(total_energy(&config), 179);
        assert_eq!(period(&init_config), 2772);
    }

    #[test]
    fn test_long_period() {
        let init_config = parse(
            r#"<x=-8, y=-10, z=0>
<x=5, y=5, z=10>
<x=2, y=-7, z=3>
<x=9, y=-8, z=-3>"#,
        );
        assert_eq!(period(&init_config), 4686774924);
    }
}
