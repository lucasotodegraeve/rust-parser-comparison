use parsers::parse_nom;
use parsers::solve;
use std::fs;

fn main() {
    let file = fs::read_to_string("input/05").unwrap();
    let (stacks, instructions) = parse_nom(file.as_str());
    let result = solve(stacks, instructions);
    println!("Result: {}", result);
}
