use std::collections::HashMap;
use std::io:: {BufReader, prelude::*};
use std::fs::File;
use std::time::Instant;

fn main() {
    part1();
    println!("--------------------------------");
    part2();
}

fn part1() {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);
    let mut sum: u32 = 0;
    let alph = String::from("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ");
    let start = Instant::now();

    let mut hash: HashMap<char, usize> = HashMap::new();
    for (i, c) in alph.chars().enumerate() {
        hash.insert(c, i + 1);
    }

    for line in reader.lines() {
        let line = line.unwrap();
        let (lower, upper) = line.split_at(line.len() / 2);
        let lower = lower.to_string();
        let mut upper = upper.to_string();
        for char in lower.chars() {
            if upper.contains(char) {
                // remove duplicate chars
                upper = upper.replace(char, "");
                // get char value from hash
                let val = hash.get(&char);
                match val {
                    // add val to sum
                    Some(val) => sum += *val as u32,
                    _ => continue,
                }
            }

        }
    }
    let stop = start.elapsed();
    println!("[PART 1]\nAnswer: {}\nTime: {:?}", sum, stop);
}

fn part2() {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);
    let mut sum: u32 = 0;
    let alph = String::from("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ");
    let start = Instant::now();

    let mut hash: HashMap<char, usize> = HashMap::new();
    for (i, c) in alph.chars().enumerate() {
        hash.insert(c, i + 1);
    }

    let mut line_num: u32 = 0;
    let mut three_lines: Vec<String> = vec![];
    for line in reader.lines() {
        let line = line.unwrap();
        three_lines.push(line);
        line_num += 1;
        if line_num % 3 == 0 {
            let first_line = three_lines[0].to_string();
            let mut second_line = three_lines[1].to_string();
            let mut third_line = three_lines[2].to_string();

            for char in first_line.chars() {
                if second_line.contains(char) && third_line.contains(char) {
                    second_line = second_line.replace(char, "");
                    third_line = third_line.replace(char, "");
                    let value_of_char = hash.get(&char);
                    match value_of_char {
                        Some(val) => sum += *val as u32,
                        _ => continue,
                    }
                }
            }

            three_lines.clear()
        }

    }
    let stop = start.elapsed();
    println!("[PART 2]\nAnswer: {}\nTime: {:?}", sum, stop);
}
