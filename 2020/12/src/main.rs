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
    coords: (i32, i32),
    waypoint: (i32, i32),
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Facing {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}

impl Ship {
    fn next_waypoint_pos(&mut self, c: &Cardinal) -> (i32, i32) {
        use crate::Cardinal::*;
        match c {
            North(num) => (self.waypoint.0, self.waypoint.1 + num),
            South(num) => (self.waypoint.0, self.waypoint.1 - num),
            East(num) => (self.waypoint.0 + num, self.waypoint.1),
            West(num) => (self.waypoint.0 - num, self.waypoint.1),
        }
    }

    fn do_move(&mut self, m: &Move) {
        use crate::Move::*;

        use crate::Cardinal::*;
        use crate::Direction::*;
        use crate::Turn::*;

        match m {
            Cardinal(c) => {
                self.waypoint = self.next_waypoint_pos(&c);
            }
            Turn(c) => {
                let rotations = match c {
                    Left(deg) => -deg / 90,
                    Right(deg) => deg / 90,
                };

                self.waypoint = match rotations % 4 {
                    -3 => (self.waypoint.1, -1 * self.waypoint.0),
                    -2 => (-1 * self.waypoint.0, -1 * self.waypoint.1),
                    -1 => (-1 * self.waypoint.1, self.waypoint.0),
                    0 => (self.waypoint.0, self.waypoint.1),
                    1 => (self.waypoint.1, -1 * self.waypoint.0),
                    2 => (-1 * self.waypoint.0, -1 * self.waypoint.1),
                    3 => (-1 * self.waypoint.1, self.waypoint.0),
                    _ => panic!("unsatisiable rotation: {:?}", rotations),
                };
            }
            Direction(Forward(d)) => {
                // Always relative to ship

                self.coords = (
                    self.coords.0 + (self.waypoint.0 * d),
                    self.coords.1 + (self.waypoint.1 * d),
                )
            }
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let mut moves = read_input(lines).unwrap();

    let mut ship = Ship {
        coords: (0, 0),
        waypoint: (10, 1),
    };

    for m in moves {
        ship.do_move(&m);
        println!("{:?}: {:?}", m, ship);
    }

    println!("{:?}", ship);
    println!("{}", (ship.coords.0.abs() + ship.coords.1.abs()).abs());
}
