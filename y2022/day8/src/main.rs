fn main() {
    solution::<true>();
}

fn solution<const PART_1: bool>() {
    let s = std::fs::read_to_string("test_input").unwrap();
    let tree_rows: Vec<&str> = s.trim().split('\n').map(|x| x.trim()).collect();
    println!("{tree_rows:?}");
    let trees: Vec<Vec<u8>> = tree_rows.iter();
}
