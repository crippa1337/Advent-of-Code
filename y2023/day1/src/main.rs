use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

fn main() {
    solution(false);
    solution(true)
}

#[derive(Debug, Copy, Clone)]
struct Num {
    val: u32,
    idx: usize,
}

impl Num {
    fn new(val: u32, idx: usize) -> Self {
        Self { val, idx }
    }

    fn later_than(&self, opponent: Num) -> bool {
        if self.idx > opponent.idx {
            return true;
        }

        false
    }

    fn earlier_than(&self, opponent: Num) -> bool {
        if self.idx < opponent.idx {
            return true;
        }

        false
    }
}

fn solution(part_2: bool) {
    let f = File::open("src/input").unwrap();
    let r = BufReader::new(f);
    let mut calibration_vals: Vec<[u32; 2]> = vec![];

    let numbers: [(&str, u32); 9] = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    for line in r.lines() {
        let line = line.unwrap();
        let mut all_finds = vec![];

        // Digit numbers
        for (k, c) in line.char_indices() {
            match c.to_digit(10) {
                Some(x) => all_finds.push(Num::new(x, k)),
                None => continue,
            }
        }
        // String numbers
        if part_2 {
            for (name, val) in numbers {
                if line.contains(name) {
                    for i in 0..(line.len() - name.len() + 1) {
                        let sub = &line[i..(i + name.len())];

                        if sub == name {
                            let num = Num::new(val, i);
                            all_finds.push(num)
                        }
                    }
                }
            }
        }

        let mut first_num = Num::new(0, 99);
        let mut second_num = Num::new(0, 0);
        for n in &all_finds {
            if n.earlier_than(first_num) {
                first_num = *n
            }

            if n.later_than(second_num) {
                second_num = *n
            }
        }

        if second_num.val == 0 {
            second_num = first_num
        }

        calibration_vals.push([first_num.val, second_num.val]);
    }

    let sum = sum_of_digits(calibration_vals);
    println!("Answer is: {sum}")
}

fn sum_of_digits(values: Vec<[u32; 2]>) -> u32 {
    let mut sum = 0;
    for pair in values {
        let mut s = 0;
        for i in pair {
            s *= 10;
            s += i;
        }

        sum += s
    }

    sum
}
