use aoc24::aoclib::*;

struct Report {
    numbers:  Vec<i32>
}

impl Report {
    fn safe(&self) -> bool {
        let differences: Vec<_> = self.numbers.iter().zip(self.numbers.iter().skip(1))
            .map(|v| *v.1 - *v.0).collect();
        differences.iter().all(|v| *v >= -3 && *v < 0 )
        || differences.iter().all(|v| *v > 0 && *v <= 3 )
    }
}

mod parsers {
    use nom::{Parser, IResult, character::complete::*, multi::*};
    use super::Report;

    pub fn report(input: &str) -> IResult<&str, Report> {
        separated_list1(space1, i32)
            .map(|v| Report { numbers: v })
            .parse(input)
    }

    pub fn report_lines(input: &str) -> IResult<&str, Vec<Report>> {
        separated_list0(newline, report).parse(input)
    }
}

fn main() {
    let reports = parse_all_stdin(parsers::report_lines);
    let answer = reports.iter().filter(|v| v.safe()).count();
    println!("{answer}");
}
