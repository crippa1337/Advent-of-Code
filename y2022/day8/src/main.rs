use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    solution::<true>();
    // solution::<false>();
}

fn solution<const PART_1: bool>() {
    let mut visible_trees = 0;

    let all_trees: Vec<Vec<char>> = BufReader::new(File::open("test_input").unwrap())
        .lines()
        .map(|r| r.unwrap().chars().collect())
        .collect();

    // count the edges
    let width = all_trees[0].len();
    let height = all_trees.len();
    visible_trees += width * 2 + height * 2 - 4;

    //

    println!("{visible_trees}");
}
