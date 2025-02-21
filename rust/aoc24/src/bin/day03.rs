use aoc24::aoclib::*;

#[derive(Debug)]
enum Instruction {
    Mul(i32, i32)
}

fn eval_instruction(i: &Instruction) -> i32 {
    match i {
        Instruction::Mul(x, y) => x * y
    }
}

mod parsers {
    use nom::{Parser, IResult, bytes::complete::tag, character::anychar, character::complete::*, multi::*, sequence::*};
    use super::Instruction;

    pub fn instruction(input: &str) -> IResult<&str, Instruction> {
        delimited(
            tag("mul("),
            separated_pair(i32, tag(","), i32),
            tag(")")
        )
            .map(|r| Instruction::Mul(r.0, r.1))
            .parse(input)
    }

    pub fn interspersed_instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
        many0(many_till(anychar, instruction).map(|r| r.1)).parse(input)
    }
}

fn main() {
    let instructions = parse_all_stdin(parsers::interspersed_instructions);
    let answer: i32 = instructions.iter().map(eval_instruction).sum();
    println!("{}", answer)
}
