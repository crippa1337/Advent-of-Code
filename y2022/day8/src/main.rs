use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    solution::<true>();
    // solution::<false>();
}

fn solution<const PART_1: bool>() {
    let f = File::open("test_input").unwrap();
    let amount_of_lines = BufReader::new(&f).lines().count();

    let f = File::open("test_input").unwrap();
    let r = BufReader::new(f);

    let mut visible_trees = 0;

    for (idx, row) in r.lines().map(|r| r.unwrap()).enumerate() {
        println!("row: {row}\nidx: {idx}");

        if idx == 0 || idx == amount_of_lines - 1 {
            visible_trees += row.len();
        }
    }

    println!("{visible_trees}");
}
