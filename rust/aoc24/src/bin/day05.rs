type Page = u32;

#[derive(Debug)]
struct Rule {
    before: Page,
    after: Page,
}

#[derive(Debug, Clone)]
struct Update {
    pages: Vec<Page>,
}

#[derive(Debug)]
struct Problem {
    rules: Vec<Rule>,
    updates: Vec<Update>,
}

impl Update {
    pub fn is_ordered(&self, rules: &Vec<Rule>) -> bool {
        rules
            .iter()
            .filter_map(|rule| {
                let p1 = self.pages.iter().position(|p| *p == rule.before)?;
                let p2 = self.pages.iter().position(|p| *p == rule.after)?;
                Some(p1 < p2)
            })
            .all(|v| v == true)
    }

    pub fn middle(&self) -> Page {
        self.pages[self.pages.len() / 2]
    }
}

mod parsers {
    use super::{Problem, Rule, Update};
    use nom::{
        bytes::complete::tag, character::complete::*, multi::*, sequence::*, IResult, Parser,
    };

    fn rule(input: &str) -> IResult<&str, Rule> {
        separated_pair(u32, tag("|"), u32)
            .map(|v| Rule {
                before: v.0,
                after: v.1,
            })
            .parse(input)
    }

    fn update(input: &str) -> IResult<&str, Update> {
        separated_list0(tag(","), u32)
            .map(|v| Update { pages: v })
            .parse(input)
    }

    pub fn problem<'a>(input: &'a str) -> IResult<&'a str, Problem> {
        let rules = |s: &'a str| separated_list0(newline, rule).parse(s);
        let updates = |s: &'a str| separated_list0(newline, update).parse(s);
        separated_pair(rules, pair(newline, newline), updates)
            .map(|v| Problem {
                rules: v.0,
                updates: v
                    .1
                    .iter()
                    .filter(|v| v.pages.is_empty() == false)
                    .cloned()
                    .collect(),
            })
            .parse(input)
    }
}

fn main() {
    let input = aoc24::aoclib::parse_all_stdin(parsers::problem);
    let answer: u32 = input
        .updates
        .iter()
        .filter_map(|u| {
            if u.is_ordered(&input.rules) {
                Some(u.middle())
            } else {
                None
            }
        })
        .sum();
    println!("{answer}");
}
