use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::time::Instant;

fn main() {
    part1();
    println!("---------------------");
    part2();
}

fn part1() {
    let instant = Instant::now();
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut sum: u32 = 0;
    let mut fattest_elf_calories: u32 = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        if line == "" {
            if sum > fattest_elf_calories {
                fattest_elf_calories = sum;
            }
            sum = 0;
            continue;
        }
        sum += line.trim().parse::<u32>().unwrap();
    }

    let time = instant.elapsed();
    println!("[PART 1]\nTime: {:?}", time);
    println!("Answer: {fattest_elf_calories}");
}

fn part2() {
    let instant = Instant::now();
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut sum: u32 = 0;
    let mut vec: Vec<u32> = vec![];

    for line in reader.lines() {
        let line = line.unwrap();
        if line == "" {
            vec.push(sum);
            sum = 0;
            continue;
        }
        sum += line.trim().parse::<u32>().unwrap();
    }

    vec.sort();
    vec.reverse();
    let answer = vec[0] + vec[1] + vec[2];
    let time = instant.elapsed();
    println!("[PART 2]\nTime: {:?}", time);
    println!("Answer: {answer}");
}
