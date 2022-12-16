use std::{
    collections::{hash_map::Entry, HashMap},
    fs,
    time::Instant,
};

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let fishes: Vec<u8> = input.split(",").map(|x| x.parse::<u8>().unwrap()).collect();
    let mut part: HashMap<u8, u64> = HashMap::new();
    for fish in &fishes {
        match part.entry(*fish) {
            Entry::Occupied(mut o) => {
                *o.get_mut() += 1;
            }
            Entry::Vacant(v) => {
                v.insert(1);
            }
        }
    }

    let start = Instant::now();
    println!(
        "[PART 1]\n{}\nTime: {:?}",
        part1(part.clone()),
        start.elapsed()
    );
    let start = Instant::now();
    println!("[PART 2]\n{}\nTime: {:?}", part2(part), start.elapsed());
}

fn part1(mut fishes: HashMap<u8, u64>) -> u64 {
    let days: u16 = 80;
    for _ in 0..days {
        let mut next_day: HashMap<u8, u64> = HashMap::new();
        for fish in &fishes {
            if *fish.0 == 0 {
                *next_day.entry(6).or_insert(0) += fish.1;
                *next_day.entry(8).or_insert(0) += fish.1;
            } else {
                *next_day.entry(fish.0 - 1).or_insert(0) += fish.1;
            }
        }
        fishes = next_day;
    }
    let mut answer: u64 = 0;
    for fish in &fishes {
        answer += fishes.get(fish.0).unwrap();
    }
    return answer;
}

fn part2(mut fishes: HashMap<u8, u64>) -> u64 {
    let days: u16 = 256;
    for _ in 0..days {
        let mut next_day: HashMap<u8, u64> = HashMap::new();
        for fish in &fishes {
            if *fish.0 == 0 {
                *next_day.entry(6).or_insert(0) += fish.1;
                *next_day.entry(8).or_insert(0) += fish.1;
            } else {
                *next_day.entry(fish.0 - 1).or_insert(0) += fish.1;
            }
        }
        fishes = next_day;
    }
    let mut answer: u64 = 0;
    for fish in &fishes {
        answer += fishes.get(fish.0).unwrap();
    }
    return answer;
}
