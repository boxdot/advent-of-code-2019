use derive_more::AddAssign;
use num::Integer;

#[derive(Debug, Copy, Clone, Eq, PartialEq, AddAssign)]
pub struct Vec3 {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Moon {
    pub position: Vec3,
    pub velocity: Vec3,
}

impl Moon {
    pub fn new(x: i64, y: i64, z: i64) -> Self {
        Self {
            position: Vec3 { x, y, z },
            velocity: Vec3 { x: 0, y: 0, z: 0 },
        }
    }

    pub fn get_potential_energy(&self) -> u64 {
        (self.position.x.abs() + self.position.y.abs() + self.position.z.abs()) as u64
    }

    pub fn get_kinetic_energy(&self) -> u64 {
        (self.velocity.x.abs() + self.velocity.y.abs() + self.velocity.z.abs()) as u64
    }
}

pub fn apply_gravity(moons: &mut [Moon]) {
    for i in 0..moons.len() - 1 {
        for j in i + 1..moons.len() {
            if moons[i].position.x < moons[j].position.x {
                moons[i].velocity.x += 1;
                moons[j].velocity.x -= 1;
            } else if moons[i].position.x > moons[j].position.x {
                moons[i].velocity.x -= 1;
                moons[j].velocity.x += 1;
            }
            if moons[i].position.y < moons[j].position.y {
                moons[i].velocity.y += 1;
                moons[j].velocity.y -= 1;
            } else if moons[i].position.y > moons[j].position.y {
                moons[i].velocity.y -= 1;
                moons[j].velocity.y += 1;
            }
            if moons[i].position.z < moons[j].position.z {
                moons[i].velocity.z += 1;
                moons[j].velocity.z -= 1;
            } else if moons[i].position.z > moons[j].position.z {
                moons[i].velocity.z -= 1;
                moons[j].velocity.z += 1;
            }
        }
    }

    moons
        .iter_mut()
        .for_each(|moon| moon.position += moon.velocity);
}

fn get_axis(moon: &Moon, axis: u32) -> (i64, i64) {
    match axis {
        0 => (moon.position.x, moon.velocity.x),
        1 => (moon.position.y, moon.velocity.y),
        2 => (moon.position.z, moon.velocity.z),
        _ => panic!("Unknown axis"),
    }
}

fn get_vec_axis(moons: &[Moon], axis: u32) -> Vec<(i64, i64)> {
    moons.iter().map(|moon| get_axis(moon, axis)).collect()
}

fn get_axis_period(moons: &[Moon], axis: u32) -> u64 {
    let moons_axis = get_vec_axis(moons, axis);

    let mut moons_mut = moons.to_vec();
    apply_gravity(&mut moons_mut);

    let mut iter = 1;
    while get_vec_axis(&moons_mut, axis)
        != moons_axis{
        apply_gravity(&mut moons_mut);
        iter += 1;
    }
    iter
}

pub fn calculate_period(moons: &[Moon]) -> u64 {
    let x_period = get_axis_period(&moons, 0);
    let y_period = get_axis_period(&moons, 1);
    let z_period = get_axis_period(&moons, 2);
    x_period.lcm(&y_period).lcm(&z_period)
}

pub fn calculate_energy_after(mut moons: Vec<Moon>, steps: u32) -> u64 {
    for _ in 0..steps {
        apply_gravity(&mut moons);
    }
    moons
        .iter()
        .map(|moon| moon.get_potential_energy() * moon.get_kinetic_energy())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_test_1() {
        let moons = vec![
            Moon::new(-1, 0, 2),
            Moon::new(2, -10, -7),
            Moon::new(4, -8, 8),
            Moon::new(3, 5, -1),
        ];
        assert_eq!(calculate_energy_after(moons.clone(), 10), 179);
        assert_eq!(calculate_period(&moons), 2772);
    }
}
