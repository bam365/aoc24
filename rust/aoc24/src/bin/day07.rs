use aoc24::aoclib::*;

#[derive(Clone, Copy, Debug)]
enum Op {
    Add,
    Mult,
}

impl Op {
    fn eval(&self, x: &i64, y: &i64) -> i64 {
        match self {
            Op::Add => x + y,
            Op::Mult => x * y,
        }
    }
}

fn eval_strand(numbers: &Vec<i64>, operands: &Vec<Op>) -> i64 {
    let mut ret = numbers[0];
    for (x, op) in numbers.iter().skip(1).zip(operands.iter()) {
        ret = op.eval(&ret, x);
    }
    ret
}

fn eval_all(numbers: &Vec<i64>, op_patterns: &Vec<Vec<Vec<Op>>>) -> Vec<i64> {
    if numbers.is_empty() {
        return vec![];
    } else {
        op_patterns[numbers.len() - 1]
            .iter()
            .map(|o| eval_strand(numbers, o))
            .collect()
    }
}

#[derive(Debug)]
struct Equation {
    test: i64,
    values: Vec<i64>,
}

impl Equation {
    fn is_possible(&self, op_patterns: &Vec<Vec<Vec<Op>>>) -> bool {
        eval_all(&self.values, op_patterns)
            .iter()
            .any(|v| *v == self.test)
    }
}

mod parsers {
    use super::*;
    use nom::{bytes::complete::*, character::complete::*, multi::*, sequence::*, IResult, Parser};

    pub fn equation(input: &str) -> IResult<&str, Equation> {
        separated_pair(i64, tag(": "), separated_list1(space1, i64))
            .map(|v| Equation {
                test: v.0,
                values: v.1,
            })
            .parse(input)
    }

    pub fn equation_lines(input: &str) -> IResult<&str, Vec<Equation>> {
        separated_list0(newline, equation).parse(input)
    }
}

fn make_op_patterns() -> Vec<Vec<Vec<Op>>> {
    let op_patterns: Vec<Vec<Vec<Op>>> = (0..15)
        .map(|n| {
            let ops = vec![vec![Op::Add, Op::Mult]; n];
            permute_choices(ops)
        })
        .collect();
    op_patterns
}

fn main() {
    let input = parse_all_stdin(parsers::equation_lines);
    let op_patterns = make_op_patterns();
    let answer: i64 = input
        .iter()
        .filter_map(|e| {
            if e.is_possible(&op_patterns) {
                Some(e.test)
            } else {
                None
            }
        })
        .sum();
    println!("{answer}")
}
