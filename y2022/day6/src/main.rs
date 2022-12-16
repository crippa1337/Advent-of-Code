use std::collections::HashSet;
use std::fs;

fn main() {
    let file = fs::read_to_string("input").unwrap();
    let file = file.replace("\n", "");
    let file = file.replace("\r", "");
    let mut vector: Vec<char> = Vec::new();

    for (i, c) in file.chars().enumerate() {
        vector.push(c);
        // 14 should be 4 to solve part 1
        if vector.len() > 14 {
            vector.remove(0);
        }

        let set: HashSet<&char> = vector.iter().collect();
        if set.len() == vector.len() {
            if i > 3 {
                println!("Vec: {:?}\nSet: {:?}", set, vector);
                println!("{}", i + 1);
                break;
            }
        }
    }
}
