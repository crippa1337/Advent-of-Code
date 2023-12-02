use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const RED: u32 = 12;
const GREEN: u32 = 13;
const BLUE: u32 = 14;

#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Debug)]
struct Info {
    color: Color,
    amount: u32,
}

impl Info {
    fn new(input: &str) -> Self {
        let split: Vec<&str> = input.split_whitespace().collect();
        let amount = split[0].parse::<u32>().unwrap();
        let color = match split[1] {
            "red" => Color::Red,
            "green" => Color::Green,
            "blue" => Color::Blue,
            _ => unreachable!(),
        };

        Self { color, amount }
    }

    fn possible(&self) -> bool {
        if self.amount > self.color_limit() {
            return false;
        }

        true
    }

    fn color_limit(&self) -> u32 {
        match self.color {
            Color::Red => RED,
            Color::Green => GREEN,
            Color::Blue => BLUE,
        }
    }
}

fn solution(part_2: bool) {
    let f = File::open("src/input").unwrap();
    let r = BufReader::new(f);
    let mut sum = 0;

    for line in r.lines() {
        let line = line.unwrap();
        let split: Vec<&str> = line.split(':').collect();
        let game_id = split[0].split(' ').collect::<Vec<&str>>()[1]
            .parse::<u32>()
            .unwrap();
        let split2: Vec<&str> = split[1].split(';').collect();
        let mut split3: Vec<Vec<&str>> = vec![];
        for s in split2 {
            split3.push(s.split(',').map(|x| x.trim()).collect())
        }
        let mut given_info = vec![];
        for i in split3 {
            for j in i {
                given_info.push(Info::new(j))
            }
        }

        if !part_2 {
            let mut possible_game = true;
            for info in &given_info {
                if !info.possible() {
                    possible_game = false;
                    break;
                }
            }

            if possible_game {
                sum += game_id
            }
        }

        if part_2 {
            let mut least_reds = 0;
            let mut least_greens = 0;
            let mut least_blues = 0;
            for info in given_info {
                match info.color {
                    Color::Red => {
                        if info.amount > least_reds {
                            least_reds = info.amount
                        }
                    }
                    Color::Green => {
                        if info.amount > least_greens {
                            least_greens = info.amount
                        }
                    }
                    Color::Blue => {
                        if info.amount > least_blues {
                            least_blues = info.amount
                        }
                    }
                }
            }

            let power = least_reds * least_greens * least_blues;
            sum += power
        }
    }

    println!("Answer: {sum}")
}

fn main() {
    solution(false);
    solution(true)
}
