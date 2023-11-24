use parsers::parse_pest;
use parsers::solve;
use std::fs;

fn main() {
    let file = fs::read_to_string("input/05").unwrap();
    let (stacks, instructions) = parse_pest(file.as_str());
    let result = solve(stacks, instructions);
    println!("Result: {}", result);
}
