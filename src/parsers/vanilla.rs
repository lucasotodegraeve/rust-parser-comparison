#![allow(dead_code)]
use crate::common::{Instruction, Stacks};

pub fn parse(s: &str) -> (Stacks, Vec<Instruction>) {
    let (crates, instructions) = s.split_once("\n\n").unwrap();
    let stacks = parse_crates(crates);
    let instructions = parse_instructions(instructions);
    (stacks, instructions)
}

pub fn parse_vanilla_slow(s: &str) -> (Stacks, Vec<Instruction>) {
    let (crates, instructions) = s.split_once("\n\n").unwrap();
    let stacks = parse_crates(crates);
    let instructions = parse_instructions_slow(instructions);
    (stacks, instructions)
}

fn parse_crates(crates: &str) -> Stacks {
    let mut crates = crates
        .split("\n")
        .map(|s| s.chars().skip(1).step_by(4).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    crates.pop();

    let mut stacks = Vec::<Vec<char>>::new();
    for _ in 0..crates[0].len() {
        stacks.push(Vec::new());
    }

    for line in crates.iter().rev() {
        for (c, s) in line.iter().zip(stacks.iter_mut()) {
            if *c != ' ' {
                s.push(*c);
            }
        }
    }

    Stacks(stacks)
}

fn parse_instructions(instr_str: &str) -> Vec<Instruction> {
    let instr_str = instr_str.strip_suffix("\n").unwrap();
    instr_str
        .split("\n")
        .map(|s| {
            let s = &s.as_bytes()[5..];
            let (s, amount) = extract_u32_at_start(s);

            let s = &s[5..];
            let (s, from) = extract_u32_at_start(s);

            let s = &s[3..];
            let to = parse_u8_to_usize(s);
            Instruction {
                amount: amount as u32,
                from,
                to,
            }
        })
        .collect::<Vec<_>>()
}

fn parse_instructions_slow(instr_str: &str) -> Vec<Instruction> {
    let instr_str = instr_str.strip_suffix("\n").unwrap();
    instr_str
        .split("\n")
        .map(|s| {
            let mut s = s.split(" ");
            s.next();
            let amount = s.next().unwrap().parse::<u32>().unwrap();
            s.next();
            let from = s.next().unwrap().parse::<usize>().unwrap();
            s.next();
            let to = s.next().unwrap().parse::<usize>().unwrap();
            Instruction { amount, from, to }
        })
        .collect::<Vec<_>>()
}

fn extract_u32_at_start(s: &[u8]) -> (&[u8], usize) {
    let mut i = 1;
    loop {
        let c = s[i];
        if c == 32 {
            let d = parse_u8_to_usize(&s[0..i]);
            return (&s[i + 1..], d);
        }
        i += 1;
    }
}

fn parse_u8_to_usize(s: &[u8]) -> usize {
    std::str::from_utf8(s).unwrap().parse::<usize>().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_u32() {
        let s = "42 hello";
        let (_, d) = extract_u32_at_start(s.as_bytes());
        assert_eq!(d, 42);
    }
}
