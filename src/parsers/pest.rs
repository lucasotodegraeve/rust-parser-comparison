#![allow(dead_code)]

use pest::{
    iterators::{Pair, Pairs},
    Parser,
};
use pest_derive::Parser;

use crate::common::{Instruction, Stacks};

#[derive(Parser)]
#[grammar = "05.pest"]
struct StackParser;

pub fn parse(s: &str) -> (Stacks, Vec<Instruction>) {
    let mut program = StackParser::parse(Rule::input, &s).unwrap();
    let mut pairs = program.next().unwrap().into_inner();
    let stacks = parse_stacks(pairs.next().unwrap().into_inner());
    let instructions = parse_instructions(pairs.next().unwrap().into_inner());
    (stacks, instructions)
}

pub fn parse_simple(s: &str) {
    let _ = StackParser::parse(Rule::input, &s).unwrap();
}

fn parse_stacks(pairs: Pairs<Rule>) -> Stacks {
    let n_stacks = pairs.peek().unwrap().into_inner().len();
    let mut stacks: Vec<Vec<Option<char>>> = Vec::with_capacity(n_stacks);
    for _ in 0..n_stacks {
        stacks.push(Vec::new());
    }

    for stacklinepair in pairs {
        for (cratepair, stack) in stacklinepair.into_inner().zip(stacks.iter_mut()) {
            let crate_or_empty = match cratepair.as_rule() {
                Rule::crate_ => Some(parse_crate(cratepair)),
                Rule::empty => None,
                _ => unreachable!(),
            };
            stack.push(crate_or_empty);
        }
    }

    let stacks = stacks
        .iter_mut()
        .map(|stack| {
            stack
                .iter()
                .rev()
                .filter(|el| el.is_some())
                .map(|el| el.as_ref().unwrap().clone())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    Stacks(stacks)
}

fn parse_crate(pair: Pair<Rule>) -> char {
    pair.into_inner().as_str().chars().next().unwrap()
}

fn parse_instructions(pairs: Pairs<Rule>) -> Vec<Instruction> {
    let mut instructions = Vec::<Instruction>::new();
    for pair in pairs {
        instructions.push(parse_move(pair.into_inner()));
    }
    instructions
}

fn parse_move(mut pairs: Pairs<Rule>) -> Instruction {
    let amount = pairs.next().unwrap().as_str().parse::<u32>().unwrap();
    let from = pairs.next().unwrap().as_str().parse::<usize>().unwrap();
    let to = pairs.next().unwrap().as_str().parse::<usize>().unwrap();

    Instruction { amount, from, to }
}
