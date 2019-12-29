use crate::day09::{execute, parse, Memory};

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

pub fn solve(input: &str) -> Result<(i64, i64)> {
    let mem = parse(input);

    // +--->
    // @ABCD
    // #####
    //
    // (~A OR ~B OR ~C) AND D
    let walk_script = b"\
        NOT A J
        NOT B T
        OR T J
        NOT C T
        OR T J
        AND D J
        WALK
    ";
    let part1 = run(mem.clone(), walk_script)?;

    // +--->
    // @ABCDEFGHI
    // ##########
    //
    // ((~A OR ~B OR ~C) AND D) AND
    // (
    //  ((~E OR ~F OR ~G) AND H) OR
    //  (E AND F AND G AND H) OR
    //  (E AND (~F OR ~G OR ~H OR ~K))
    // )
    // <=> (~A OR ~B OR ~C) AND D AND (E OR H)
    // <=> ~(A AND B AND C) AND D AND (E OR H)

    let run_script = b"\
        NOT T T
        AND A T
        AND B T
        AND C T
        NOT T T
        AND D T
        OR E J
        OR H J
        AND T J
        RUN
    ";
    let part2 = run(mem, run_script)?;
    Ok((part1, part2))
}

fn run(mut mem: Memory, script: &[u8]) -> Result<i64> {
    let mut index = 0;
    let mut retval = None;
    let mut ip = Some(0);
    while let Some(next_ip) = ip {
        ip = execute(
            &mut mem,
            next_ip,
            || {
                let value = script[index] as i64;
                index += 1;
                value
            },
            |value| {
                if value < 128 {
                    print!("{}", value as u8 as char);
                } else {
                    retval = Some(value);
                }
            },
        )?;
        if retval.is_some() {
            break;
        }
    }

    retval.ok_or_else(|| "did not make it across".into())
}
