use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::time::Instant;

fn main() {
    part1();
    println!("---------------------");
    part2();
}

fn part1() {
    fn str_to_value(str_hand: &str) -> u32 {
        match str_hand {
            // Draw with rock
            "AX" => 3 + 1,
            // Win with rock
            "AY" => 6 + 2,
            // Lose with rock
            "AZ" => 0 + 3,
            // Lose with paper
            "BX" => 0 + 1,
            // Draw with paper
            "BY" => 3 + 2,
            // Win with paper
            "BZ" => 6 + 3,
            // Win with scissors
            "CX" => 6 + 1,
            // Lose with scissors
            "CY" => 0 + 2,
            // Draw with scissors
            "CZ" => 3 + 3,
            // Won't happen with current input
            _ => unreachable!(),
        }
    }

    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);
    let mut sum: u32 = 0;
    let start = Instant::now();
    for line in reader.lines() {
        let line = line.unwrap();
        let line = line.replace(" ", "");
        sum += str_to_value(&line);
    }
    let stop = start.elapsed();
    println!("[PART 1]\nTime: {:?}\nAnswer: {sum}", stop);
}

fn part2() {
    fn str_to_value(str_hand: &str) -> u32 {
        match str_hand {
            // Lose with scissors
            "AX" => 0 + 3,
            // Draw with rock
            "AY" => 3 + 1,
            // Win with paper
            "AZ" => 6 + 2,
            // Lose with rock
            "BX" => 0 + 1,
            // Draw with paper
            "BY" => 3 + 2,
            // Win with scissors
            "BZ" => 6 + 3,
            // Lose with paper
            "CX" => 0 + 2,
            // Draw with scissors
            "CY" => 3 + 3,
            // Win with rock
            "CZ" => 6 + 1,
            // Won't happen with current input
            _ => unreachable!(),
        }
    }

    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);
    let mut sum: u32 = 0;
    let start = Instant::now();
    for line in reader.lines() {
        let line = line.unwrap();
        let line = line.replace(" ", "");
        sum += str_to_value(&line);
    }
    let stop = start.elapsed();
    println!("[PART 2]\nTime: {:?}\nAnswer: {sum}", stop);
}

// this should be committed as done, idk what i did on the last commit