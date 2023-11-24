#![allow(dead_code)]
use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_until},
    character::complete::{char, newline, u32},
    multi::{many0, separated_list1},
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};

use crate::common::{Instruction, Stacks};

pub fn parse(s: &str) -> (Stacks, Vec<Instruction>) {
    let (_, res) = parse_all(s).unwrap();
    res
}

pub fn parse_simple(s: &str) {
    let (rem, _crates) = parse_stacks(s).unwrap();
    let (rem, _) = skip_mid(rem).unwrap();
    let (_rem, _instructions) = parse_instructions(rem).unwrap();
}

fn parse_all(s: &str) -> IResult<&str, (Stacks, Vec<Instruction>)> {
    let (rem, crates) = parse_stacks(s)?;
    let (rem, _) = skip_mid(rem)?;
    let (rem, instructions) = parse_instructions(rem)?;

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

    Ok((rem, (Stacks(stacks), instructions)))
}

fn skip_mid(s: &str) -> IResult<&str, &str> {
    let (rem, _) = take_until("\n")(s)?;
    take(2u8)(rem)
}

fn parse_crate(s: &str) -> IResult<&str, char> {
    let (rem, c) = delimited(tag("["), take(1u8), tag("]"))(s)?;
    Ok((rem, c.chars().next().unwrap()))
}

fn parse_empty(s: &str) -> IResult<&str, char> {
    let (rem, _) = tag("   ")(s)?;
    Ok((rem, ' '))
}

fn parse_crates_line(s: &str) -> IResult<&str, Vec<char>> {
    terminated(
        separated_list1(char(' '), alt((parse_crate, parse_empty))),
        newline,
    )(s)
}

fn parse_stacks(s: &str) -> IResult<&str, Vec<Vec<char>>> {
    many0(parse_crates_line)(s)
}

fn parse_move(s: &str) -> IResult<&str, Instruction> {
    let (remaining, (amount, from, to)) = terminated(
        tuple((
            preceded(tag("move "), u32),
            preceded(tag(" from "), u32),
            preceded(tag(" to "), u32),
        )),
        newline,
    )(s)?;

    Ok((
        remaining,
        Instruction {
            amount,
            from: from as usize,
            to: to as usize,
        },
    ))
}

fn parse_instructions(s: &str) -> IResult<&str, Vec<Instruction>> {
    let (remaining, instructions) = many0(parse_move)(s)?;
    Ok((remaining, instructions))
}
