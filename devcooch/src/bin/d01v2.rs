fn fuel(mass: u64) -> i64 {
    (mass as f64 / 3.0).floor() as i64 - 2
}

fn main() {
    let contents = include_str!("day01.txt");
    let mut tot: u64 = 0;
    for line in contents.lines() {
        let module_mass = line.parse::<u64>().unwrap();
        let mut need: i64 = fuel(module_mass);
        let mut module_fuel: u64 = need as u64;
        while need > 0 {
            need = fuel(need as u64);
            if need > 0 {
                module_fuel += need as u64;
            }
        }
        tot += module_fuel;
    }
    println!("{}", tot);
}
