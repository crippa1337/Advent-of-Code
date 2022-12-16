use std::fs;

fn main() {
    let input = fs::read_to_string("test_input").unwrap();
    part1(&input);
}

fn part1(input: &String) {
    let mut answer: u32 = 0;

    // Get the height of the grid
    let lines = input.lines();
    let height = lines.count();

    // Get the width of the grid
    let mut lines = input.lines();
    let first_line = lines.next().unwrap();

    let mut width = 0;
    for ch in first_line.chars() {
        if !ch.is_whitespace() {
            width += 1;
        }
    }

    // Create the grid
    let mut grid: Vec<Vec<u32>> = vec![vec![0; width]; height];

    // Populate the grid
    let mut i = 0;
    for line in input.lines() {
        let mut j = 0;
        for elem in line.split(" ") {
            grid[i][j] = elem.parse().expect("Invalid input");
            j += 1;
        }
        i += 1;
    }

    // Create a vector containing only the inner elements of the grid
    let mut inner_grid = Vec::new();

    for i in 1..height - 1 {
        for j in 1..width - 1 {
            inner_grid.push(grid[i][j]);
        }
    }
}
