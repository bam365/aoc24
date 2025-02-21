use aoc24::aoclib::*;

type Pair = (i32, i32);

mod parsers {
    use nom::{Parser, IResult, sequence::*, multi::*, character::complete::* };

    pub fn number_pair_lines(input: &str) -> IResult<&str, Vec<super::Pair>> {
        separated_list0(newline, separated_pair(i32, space1, i32)).parse(input)
    }
}

fn sorted_pair_diff(pairs: &Vec<Pair>) -> i32 {
    let mut firsts: Vec<_> = pairs.iter().map(|v| v.0).collect();
    let mut seconds: Vec<_> = pairs.iter().map(|v| v.1).collect();
    firsts.sort();
    seconds.sort();
    firsts.iter().zip(seconds.iter()).map(|p| (p.0 - p.1).abs()).sum()
}

fn main() {
    let pairs = parse_all_stdin(parsers::number_pair_lines);
    println!("{}", sorted_pair_diff(&pairs));
}
