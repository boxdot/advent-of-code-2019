use day24::*;

use ndarray::Array2;

use std::io;
use std::process;
use std::collections::BTreeMap;

fn read_input(mut in_stream: impl io::BufRead) -> io::Result<Array2<Cell>> {
    let mut buffer = String::new();
    in_stream.read_to_string(&mut buffer)?;
    Ok(parse_map(&buffer))
}

fn main() {
    let input = read_input(io::stdin().lock());
    match input {
        Ok(map) => {
            println!("{}", get_code(&first_equal(map.clone())));
            let mut multimap = BTreeMap::new();
            multimap.insert(0, map);
            println!("{}", count_bugs(&iterate_multimap(multimap, 200)));
        }
        Err(e) => {
            eprintln!("Error while parsing input: {}", e);
            process::exit(1);
        }
    }
}
