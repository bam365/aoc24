use aoc24::aoclib::*;

fn sequence(start: &Coord, deltas: &(i32, i32)) -> Vec<Coord> {
    (0..4)
        .filter_map(|i| start.change((deltas.0 * i, deltas.1 * i)))
        .collect()
}

fn all_changes() -> Vec<(i32, i32)> {
    let mut r: Vec<(i32, i32)> = vec![];
    for x in -1..=1 {
        for y in -1..=1 {
            if !(x == 0 && y == 0) {
                r.push((x, y));
            }
        }
    }
    r
}

struct Puzzle {
    chars: Vec<Vec<char>>,
}

impl Puzzle {
    pub fn sequence_is_xmas(&self, sequence: &Vec<Coord>) -> bool {
        let myget = |coord: &Coord| -> Option<&char> {
            self.chars.get(coord.y).and_then(|v| v.get(coord.x))
        };
        let result: String = sequence.iter().filter_map(myget).collect();
        result == "XMAS"
    }

    pub fn count_xmases_at_position(&self, pos: &Coord) -> u32 {
        all_changes()
            .iter()
            .map(|c| {
                if self.sequence_is_xmas(&sequence(pos, c)) {
                    1
                } else {
                    0
                }
            })
            .sum()
    }

    pub fn count_all_xmases(&self) -> u32 {
        let mut count: u32 = 0;
        for y in 0..(self.chars.len()) {
            for x in 0..(self.chars[y].len()) {
                count += self.count_xmases_at_position(&(Coord { x, y }));
            }
        }
        count
    }
}

mod parsers {
    use super::Puzzle;
    use nom::{character::complete::*, multi::*, IResult, Parser};

    pub fn puzzle(input: &str) -> IResult<&str, Puzzle> {
        many0(many_till(anychar, newline).map(|v| v.0))
            .map(|v| Puzzle { chars: v })
            .parse(input)
    }
}

fn main() {
    let puzzle = parse_all_stdin(parsers::puzzle);
    let answer = puzzle.count_all_xmases();
    println!("{answer}")
}
