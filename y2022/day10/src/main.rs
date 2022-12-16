use std::fs;

fn main() {
    let input = fs::read_to_string("largetest_input").unwrap();
    part1(&input);
}

fn part1(input: &String) {
    let xreg: i32 = 1;
    let cycles: i32 = 0;
    let mut cycles_xreg = [cycles, xreg];
    let mut checks_done: usize = 0;
    let mut signal_strength = 0;

    for line in input.lines() {
        match line.split_once("addx") {
            // Calls Cycles() with the value of the addx
            Some(addx) => cycles_xreg = cpu(addx.1, cycles_xreg),
            // Call Cycles() with noop
            None => cycles_xreg = cpu(line, cycles_xreg),
        }

        signal_strength += add_signal_strength(cycles_xreg, checks_done).0;
        checks_done += add_signal_strength(cycles_xreg, checks_done).1;

        println!("{:?}", cycles_xreg[0]);
    }
}

fn cpu(mut string: &str, mut cycles_xreg: [i32; 2]) -> [i32; 2] {
    // CYCLES_XREG[0] -> CYCLE COUNT
    // CYCLES_XREG[1] -> X VALUE OF CPU
    string = string.trim();
    if string == "noop" {
        cycles_xreg[0] += 1;
    } else {
        cycles_xreg[0] += 2;
        cycles_xreg[1] += string.parse::<i32>().expect("Failed to parse addx!");
    }

    return cycles_xreg;
}

fn add_signal_strength(cycles_xreg: [i32; 2], mut checks_done: usize) -> (i32, usize) {
    let cycle_checks: [i32; 6] = [20, 60, 100, 140, 180, 220];

    if cycles_xreg[0] == cycle_checks[checks_done] {
        checks_done += 1;
        return (cycles_xreg[0] * cycles_xreg[1], checks_done);
    } else {
        return (0, 0);
    }
}

// cant figure out how to check each cycle by itself, cycle increments at addx skips checks
