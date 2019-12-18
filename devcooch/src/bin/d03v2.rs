use std::cmp::Ordering;

struct Vertical {
    pub x: i32,
    pub y1: i32,
    pub y2: i32,
    pub path: i32,
    pub inverted: bool,
}

struct Horizontal {
    pub y: i32,
    pub x1: i32,
    pub x2: i32,
    pub path: i32,
    pub inverted: bool,
}

struct Dot {
    pub x: i32,
    pub y: i32,
    pub d1: i32,
    pub d2: i32,
    pub h_i: bool,
    pub v_i: bool,
}

fn def_to_lines(wire: &str) -> (Vec<Vertical>, Vec<Horizontal>) {
    let mut verticals = Vec::new();
    let mut horizontals = Vec::new();
    let (mut x, mut y) = (0, 0);
    let mut path = 0;
    for command in wire.split(',') {
        let direction = command.get(0..1).unwrap();
        let amount: i32 = command.get(1..).unwrap().parse().unwrap();
        match direction {
            "R" => {
                horizontals.push(Horizontal {
                    y,
                    x1: x,
                    x2: x + amount,
                    path,
                    inverted: false,
                });
                x += amount;
            }
            "L" => {
                horizontals.push(Horizontal {
                    y,
                    x1: x - amount,
                    x2: x,
                    path,
                    inverted: true,
                });
                x -= amount;
            }
            "U" => {
                verticals.push(Vertical {
                    x,
                    y1: y,
                    y2: y + amount,
                    path,
                    inverted: false,
                });
                y += amount;
            }
            "D" => {
                verticals.push(Vertical {
                    x,
                    y1: y - amount,
                    y2: y,
                    path,
                    inverted: true,
                });
                y -= amount;
            }
            _ => panic!(),
        }
        path += amount;
    }
    debug_print(&verticals, &horizontals);
    verticals.sort_by(|a, b| a.y1.cmp(&b.y1));
    horizontals.sort_by(|a, b| a.x1.cmp(&b.x1));
    (verticals, horizontals)
}

fn create_intersections(verticals: &[Vertical], horizontals: &[Horizontal]) -> Vec<Dot> {
    let mut dots = Vec::new();
    for v in verticals {
        let any_h = horizontals.binary_search_by(|probe| match probe.x1.cmp(&v.x) {
            Ordering::Less | Ordering::Equal => match probe.x2.cmp(&v.x) {
                Ordering::Greater | Ordering::Equal => Ordering::Equal,
                Ordering::Less => Ordering::Less,
            },
            Ordering::Greater => Ordering::Greater,
        });
        if any_h.is_err() {
            continue;
        }
        let h = &horizontals[any_h.unwrap()];
        if h.y >= v.y1 && h.y <= v.y2 {
            let distance1 = v.path + if v.inverted { v.y2 - h.y } else { h.y - v.y1 };
            let distance2 = h.path + if h.inverted { h.x2 - v.x } else { v.x - h.x1 };
            if (v.path + h.path) > 0 {
                let dot = Dot {
                    x: v.x,
                    y: h.y,
                    d1: distance1,
                    d2: distance2,
                    h_i: h.inverted,
                    v_i: v.inverted,
                };
                print_vertical(&v);
                print_horizontal(&h);
                print_dot(&dot);
                dots.push(dot);
            }
        }
    }
    dots
}

fn direction(inverted: bool) -> char {
    if inverted {
        '-'
    } else {
        '+'
    }
}

fn print_vertical(v: &Vertical) {
    println!(
        "V --> X:{}, Y:[{},{}], D:{}, {}",
        v.x,
        v.y1,
        v.y2,
        v.path,
        direction(v.inverted),
    );
}

fn print_horizontal(h: &Horizontal) {
    println!(
        "H --> X:[{},{}], Y:{}, D:{}, {}",
        h.x1,
        h.x2,
        h.y,
        h.path,
        direction(h.inverted)
    );
}

fn print_dot(d: &Dot) {
    println!(
        "D --> P:[{},{}], D:[{},{},{}], {}/{}",
        d.x,
        d.y,
        d.d1,
        d.d2,
        d.d1 + d.d2,
        direction(d.v_i),
        direction(d.h_i),
    );
}

fn debug_print(vert: &[Vertical], horiz: &[Horizontal]) {
    for v in vert {
        print_vertical(&v);
    }
    for h in horiz {
        print_horizontal(&h);
    }
}

fn debug_print_dots(dots: &[Dot]) {
    for dot in dots {
        print_dot(dot);
    }
}

fn main() {
    let contents = include_str!("day03.txt");
    let mut lines = contents.lines();
    let wire1 = def_to_lines(lines.next().unwrap());
    let wire2 = def_to_lines(lines.next().unwrap());
    println!("------");
    let mut dots = create_intersections(&wire1.0, &wire2.1);
    dots.extend(create_intersections(&wire2.0, &wire1.1));
    println!("------");
    debug_print_dots(&dots);
    println!("------");
    println!("{}", dots.iter().map(|x| x.d1 + x.d2).min().unwrap());
}
