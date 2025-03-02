use aoc24::aoclib::*;

#[derive(PartialEq, Clone, Debug)]
enum Direction {
    Right,
    Up,
    Left,
    Down,
}

impl Direction {
    fn turn_right(&self) -> Direction {
        match self {
            Direction::Right => Direction::Down,
            Direction::Up => Direction::Right,
            Direction::Left => Direction::Up,
            Direction::Down => Direction::Left,
        }
    }

    fn coords_forward(&self) -> (i32, i32) {
        match self {
            Direction::Right => (1, 0),
            Direction::Up => (0, -1),
            Direction::Left => (-1, 0),
            Direction::Down => (0, 1),
        }
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum Spot {
    Empty,
    Obstacle,
    Player,
    PlayerPath,
}

struct Board {
    spots: Vec<Vec<Spot>>,
    player_direction: Direction,
    player_location: Option<Coord>,
}

impl Board {
    fn set_spot(&mut self, safe_coord: &Coord, spot: Spot) {
        self.spots[safe_coord.y][safe_coord.x] = spot;
    }

    fn next_position_and_spot(&self) -> Option<(Coord, Spot)> {
        let next_position = self
            .player_location
            .clone()?
            .change(self.player_direction.coords_forward())?;
        let row = self.spots.get(next_position.y)?;
        let spot = row.get(next_position.x)?;
        Some((next_position.clone(), *spot))
    }

    pub fn next_frame(&mut self) -> Option<bool> {
        let position = self.player_location.clone()?;
        match self.next_position_and_spot() {
            None => {
                self.set_spot(&position, Spot::PlayerPath);
                self.player_location = None;
                None
            }
            Some((next_position, Spot::Empty | Spot::PlayerPath | Spot::Player)) => {
                self.set_spot(&next_position, Spot::Player);
                self.set_spot(&position, Spot::PlayerPath);
                self.player_location = Some(next_position);
                Some(true)
            }
            Some((_, Spot::Obstacle)) => {
                self.player_direction = self.player_direction.turn_right();
                Some(false)
            }
        }
    }

    pub fn player_path_count(&self) -> usize {
        self.spots
            .iter()
            .map(|v| v.iter().filter(|s| **s == Spot::PlayerPath).count())
            .sum()
    }

    pub fn is_done(&self) -> bool {
        self.player_location.is_none()
    }
}

mod parsers {
    use super::*;
    use nom::{
        branch::alt, bytes::complete::tag, character::complete::*, multi::*, IResult, Parser,
    };

    pub fn spot(input: &str) -> IResult<&str, (Spot, Option<Direction>)> {
        alt((
            tag(".").map(|_| (Spot::Empty, None)),
            tag("#").map(|_| (Spot::Obstacle, None)),
            tag(">").map(|_| (Spot::Player, Some(Direction::Right))),
            tag("^").map(|_| (Spot::Player, Some(Direction::Up))),
            tag("<").map(|_| (Spot::Player, Some(Direction::Left))),
            tag("v").map(|_| (Spot::Player, Some(Direction::Down))),
        ))
        .parse(input)
    }

    pub fn spots(input: &str) -> IResult<&str, Vec<Vec<(Spot, Option<Direction>)>>> {
        many0(many_till(spot, newline).map(|line| line.0)).parse(input)
    }
}

fn board_from_spots(spots: Vec<Vec<(Spot, Option<Direction>)>>) -> Board {
    let mut player_info: Option<(Coord, Direction)> = None;
    for (y, row) in spots.iter().enumerate() {
        for (x, spot) in row.iter().enumerate() {
            if spot.1.is_some() {
                player_info = Some((Coord { x, y }, spot.1.clone().unwrap()));
            };
        }
    }
    let found_player_info = player_info.unwrap();
    return Board {
        spots: spots
            .iter()
            .map(|row| row.iter().map(|v| v.0).collect())
            .collect(),
        player_location: Some(found_player_info.0),
        player_direction: found_player_info.1,
    };
}

fn main() {
    let spots = parse_all_stdin(parsers::spots);
    let mut board = board_from_spots(spots);
    while !board.is_done() {
        board.next_frame();
    }
    let answer = board.player_path_count();
    println!("{answer}");
}
