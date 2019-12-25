use std::cmp::Ordering;

struct Vertical {
    pub x: i32,
    pub y1: i32,
    pub y2: i32,
}

struct Horizontal {
    pub y: i32,
    pub x1: i32,
    pub x2: i32,
}

fn def_to_lines(wire: &str) -> (Vec<Vertical>, Vec<Horizontal>) {
    let mut verticals = Vec::new();
    let mut horizontals = Vec::new();
    let (mut x, mut y) = (0, 0);
    for command in wire.split(',') {
        let direction = command.get(0..1).unwrap();
        let amount: i32 = command.get(1..).unwrap().parse().unwrap();
        match direction {
            "R" => {
                horizontals.push(Horizontal {
                    y,
                    x1: x,
                    x2: x + amount,
                });
                x += amount;
            }
            "L" => {
                horizontals.push(Horizontal {
                    y,
                    x1: x - amount,
                    x2: x,
                });
                x -= amount;
            }
            "U" => {
                verticals.push(Vertical {
                    x,
                    y1: y,
                    y2: y + amount,
                });
                y += amount;
            }
            "D" => {
                verticals.push(Vertical {
                    x,
                    y1: y - amount,
                    y2: y,
                });
                y -= amount;
            }
            _ => panic!(),
        }
    }
    verticals.sort_by(|a, b| a.y1.cmp(&b.y1));
    horizontals.sort_by(|a, b| a.x1.cmp(&b.x1));
    (verticals, horizontals)
}

fn create_intersections(verticals: &[Vertical], horizontals: &[Horizontal]) -> Vec<(i32, i32)> {
    let mut dots = Vec::new();
    for line in verticals {
        let any_h = horizontals.binary_search_by(|probe| match probe.x1.cmp(&line.x) {
            Ordering::Less | Ordering::Equal => match probe.x2.cmp(&line.x) {
                Ordering::Greater | Ordering::Equal => Ordering::Equal,
                Ordering::Less => Ordering::Less,
            },
            Ordering::Greater => Ordering::Greater,
        });
        if any_h.is_err() {
            continue;
        }
        let h = &horizontals[any_h.unwrap()];
        if h.y >= line.y1 && h.y <= line.y2 {
            print_vertical(&line);
            print_horizontal(&h);
            dots.push((line.x, h.y));
        }
    }
    dots
}

fn print_vertical(v: &Vertical) {
    println!("X:{}, Y:[{},{}]", v.x, v.y1, v.y2);
}

fn print_horizontal(h: &Horizontal) {
    println!("X:[{},{}], Y:{}", h.x1, h.x2, h.y);
}

/*
fn debug_print(vert: &Vec<Vertical>, horiz: &Vec<Horizontal>) {
    println!("V");
    for v in vert {
        print_vertical(&v);
    }
    println!("H");
    for h in horiz {
        print_horizontal(&h);
    }
}

fn debug_print_dots(dots: &Vec<(i32, i32)>) {
    for dot in dots {
        println!("{},{}", dot.0, dot.1);
    }
}
*/

fn main() {
    let contents = include_str!("day03.txt");
    let mut lines = contents.lines();
    let wire1 = def_to_lines(lines.next().unwrap());
    //debug_print(&wire1.0, &wire1.1);
    let wire2 = def_to_lines(lines.next().unwrap());
    //debug_print(&wire2.0, &wire2.1);
    let mut dots = create_intersections(&wire1.0, &wire2.1);
    dots.extend(create_intersections(&wire2.0, &wire1.1));
    dots.retain(|&x| x != (0, 0));
    //debug_print_dots(&dots);
    println!("{}", dots.len());
    println!(
        "{}",
        dots.iter().map(|x| x.0.abs() + x.1.abs()).min().unwrap()
    );
}
