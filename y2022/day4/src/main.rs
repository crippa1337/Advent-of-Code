use std::{fs, time::Instant};

fn main() {
    let file = fs::read_to_string("input").unwrap();
    part1(&file);
    println!("----------------");
    part2(&file);
}

fn create_two_numbers(string: &str) -> Vec<u32> {
    let (first_number, second_number): (&str, &str) = string.split_once("-").unwrap();
    let mut number_array: Vec<u32> = Vec::new();
    number_array.push(first_number.parse::<u32>().unwrap());
    number_array.push(second_number.parse::<u32>().unwrap());
    return number_array;
}

fn part1(file: &String) {
    let start = Instant::now();
    let mut answer: u32 = 0;
    for line in file.lines() {
        // Split each line at the comma to divide the ranges
        let (first_range, second_range): (&str, &str) = line.split_once(",").unwrap();
        // Use the create num function to parse, f.e (1-7) to [1, 7]
        let (first_range, second_range) = (
            create_two_numbers(&first_range),
            create_two_numbers(&second_range),
        );

        // Compare the numbers to see if any overlapment is found
        if (first_range[0] <= second_range[0] && first_range[1] >= second_range[1])
            || (first_range[0] >= second_range[0] && first_range[1] <= second_range[1])
        {
            answer += 1
        }
    }
    let stop = start.elapsed();
    println!("[PART 1]\nAnswer: {answer}\nTime: {:?}", stop)
}

fn part2(file: &String) {
    let start = Instant::now();
    let mut answer: u32 = 0;
    for line in file.lines() {
        let (first_range, second_range): (&str, &str) = line.split_once(",").unwrap();
        let (mut first_range, mut second_range) = (
            create_two_numbers(&first_range),
            create_two_numbers(&second_range),
        );

        if (first_range[0] > second_range[0])
            || (first_range[0] == second_range[0] && first_range[1] < second_range[1])
        {
            (
                first_range[0],
                first_range[1],
                second_range[0],
                second_range[1],
            ) = (
                second_range[0],
                second_range[1],
                first_range[0],
                first_range[1],
            );
        }

        if first_range[1] >= second_range[0] {
            answer += 1;
        }
    }

    let stop = start.elapsed();
    println!("[PART 2]\nAnswer: {answer}\nTime: {:?}", stop)
}
