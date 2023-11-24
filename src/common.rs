#![allow(dead_code)]

#[derive(Debug)]
pub struct Stacks(pub Vec<Vec<char>>);

#[derive(Debug)]
pub struct Instruction {
    pub amount: u32,
    pub from: usize,
    pub to: usize,
}

pub fn solve(mut stacks: Stacks, instructions: Vec<Instruction>) -> String {
    for instr in instructions {
        for _ in 0..instr.amount {
            let c = stacks.0[instr.from - 1].pop().unwrap();
            stacks.0[instr.to - 1].push(c);
        }
    }

    let mut result = String::with_capacity(stacks.0.len());
    for stack in stacks.0 {
        result.push(*stack.last().unwrap());
    }

    result
}
