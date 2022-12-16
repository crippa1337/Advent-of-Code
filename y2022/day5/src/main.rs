use std::collections::HashMap;
use std::fs;

fn main() {
    // read the initial crate map
    let crates = fs::read_to_string("crates").unwrap();
    let instructions = fs::read_to_string("input").unwrap();
    let mut crate_map = init_map(crates);
    println!("Top crates before: {}", get_top_crates(&crate_map));
    part1(&instructions, crate_map.clone());
    part2(instructions, crate_map);
}

fn part1(instructions: &String, mut crate_map: HashMap<u32, Vec<char>>) {
    for line in instructions.lines() {
        let (move_amount, move_from, move_to) = parse_instruction(line);
        for _ in 0..move_amount {
            let move_stack = crate_map.get_mut(&move_from).unwrap();
            let top_crate = move_stack.pop().unwrap();
            let target_stack = crate_map.get_mut(&move_to).unwrap();
            target_stack.push(top_crate)
        }
    }
    println!("[PART 1]\nTop crates after: {}", get_top_crates(&crate_map))
}

fn part2(instructions: String, mut crate_map: HashMap<u32, Vec<char>>) {
    for line in instructions.lines() {
        let (move_amount, move_from, move_to) = parse_instruction(line);
        let mut crate_vec: Vec<char> = vec![];
        for _ in 0..move_amount {
            let move_stack = crate_map.get_mut(&move_from).unwrap();
            let top_crate = move_stack.pop().unwrap();
            crate_vec.push(top_crate);
        }
        crate_vec.reverse();
        let target_stack = crate_map.get_mut(&move_to).unwrap();
        target_stack.append(&mut crate_vec);
    }
    println!("[PART 2]\nTop crates after: {}", get_top_crates(&crate_map));
}

fn init_map(file: String) -> HashMap<u32, Vec<char>> {
    let mut crate_map: HashMap<u32, Vec<char>> = HashMap::new();
    for line in file.lines().rev() {
        // start at the second char because the first is a bracket
        let mut i: usize = 1;
        let mut stack_index: u32 = 1;
        // while there are still chars to read in the line
        while i < line.len() {
            // iterate through the chars
            let char = line.chars().nth(i).unwrap();
            if char.is_alphabetic() {
                // insert char as crate into stack or create a new stacks if there isn't on at the stack index
                let crate_map = crate_map.entry(stack_index).or_insert(Vec::new());
                crate_map.push(char);
            }
            // step 4 chars
            i += 4;
            stack_index += 1;
        }
    }

    return crate_map;
}

fn parse_instruction(instruction: &str) -> (u32, u32, u32) {
    let mut line_instructions: Vec<u32> = Vec::new();
    for number in instruction.split(" ") {
        match number.to_string().parse::<u32>() {
            Ok(x) => line_instructions.push(x),
            Err(_) => continue,
        }
    }

    let (move_amount, move_from, move_to) = (
        line_instructions[0],
        line_instructions[1],
        line_instructions[2],
    );

    return (move_amount, move_from, move_to);
}

fn get_top_crates(crate_map: &HashMap<u32, Vec<char>>) -> String {
    let mut top_crates: Vec<char> = Vec::new();
    for i in 1..crate_map.len() + 1 {
        let crates = crate_map.get(&(i as u32)).unwrap();
        top_crates.push(crates[crates.len() - 1]);
    }
    // iterate over the top crates and collect to return type, string
    return top_crates.iter().collect();
}
