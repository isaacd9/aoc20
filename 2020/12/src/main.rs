use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead, Error, Lines, StdinLock};

#[derive(Debug, PartialEq, Clone)]
enum Cardinal {
    North(i32),
    South(i32),
    East(i32),
    West(i32),
}

#[derive(Debug, PartialEq, Clone)]
enum Turn {
    Left(i32),
    Right(i32),
}

#[derive(Debug, PartialEq, Clone)]
enum Direction {
    Forward(i32),
}

#[derive(Debug, PartialEq, Clone)]
enum Move {
    Cardinal(Cardinal),
    Turn(Turn),
    Direction(Direction),
}

fn read_input(lines: Lines<StdinLock>) -> Result<Vec<Move>, io::Error> {
    use crate::Move::*;

    use crate::Cardinal::*;
    use crate::Direction::*;
    use crate::Turn::*;

    let line_re = Regex::new(r"(?P<direction>\w)(?P<num>\d+)").unwrap();

    let moves = lines
        .map(|li| li.unwrap())
        .map(|li| {
            let caps = line_re.captures(&li).unwrap();
            match &caps["direction"] {
                "N" => Cardinal(North(caps["num"].parse().unwrap())),
                "S" => Cardinal(South(caps["num"].parse().unwrap())),
                "E" => Cardinal(East(caps["num"].parse().unwrap())),
                "W" => Cardinal(West(caps["num"].parse().unwrap())),
                "L" => Turn(Left(caps["num"].parse().unwrap())),
                "R" => Turn(Right(caps["num"].parse().unwrap())),
                "F" => Direction(Forward(caps["num"].parse().unwrap())),
                _ => panic!("oh no, unsupported direction: {}", &caps["direction"]),
            }
        })
        .collect();

    Ok(moves)
}

#[derive(Debug)]
struct Ship {
    direction: Facing,
    coords: (i32, i32),
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Facing {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}

impl Ship {
    fn next_pos(&mut self, c: Cardinal) -> (i32, i32) {
        use crate::Cardinal::*;
        match c {
            North(num) => (self.coords.0, self.coords.1 + num),
            South(num) => (self.coords.0, self.coords.1 - num),
            East(num) => (self.coords.0 + num, self.coords.1),
            West(num) => (self.coords.0 - num, self.coords.1),
        }
    }

    fn do_move(&mut self, m: Move) {
        use crate::Move::*;

        use crate::Cardinal::*;
        use crate::Direction::*;
        use crate::Turn::*;

        let facings = [Facing::North, Facing::East, Facing::South, Facing::West];

        match m {
            Cardinal(c) => {
                self.coords = self.next_pos(c);
            }
            Turn(c) => {
                self.direction = match c {
                    Right(deg) => {
                        facings[(((self.direction as i32) + deg / 90) % 4) as usize].clone()
                    }
                    Left(deg) => {
                        let mut index = ((self.direction as i32) - deg / 90) % 4;
                        if index < 0 {
                            index = 4 + index;
                        }
                        facings[index as usize].clone()
                    }
                }
            }
            Direction(d) => {
                self.coords = match d {
                    Forward(num) => match self.direction {
                        Facing::North => self.next_pos(North(num)),
                        Facing::South => self.next_pos(South(num)),
                        Facing::East => self.next_pos(East(num)),
                        Facing::West => self.next_pos(West(num)),
                    },
                }
            }
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let mut moves = read_input(lines).unwrap();

    let mut ship = Ship {
        direction: Facing::East,
        coords: (0, 0),
    };

    for m in moves {
        ship.do_move(m);
    }

    println!("{:?}", ship);
    println!("{}", (ship.coords.0.abs() + ship.coords.1.abs()).abs());
}
