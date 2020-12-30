use std::collections::{HashMap, HashSet};
use std::fmt;
use std::io::{self, BufRead, Error, Lines, StdinLock};
use std::iter::FromIterator;

#[derive(Debug, PartialEq, Clone)]
enum Direction {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast,
}

#[derive(Debug, PartialEq, Clone)]
struct Path {
    directions: Vec<Direction>,
}

impl Path {
    fn traverse(&self) -> (i64, i64, i64) {
        use Direction::*;

        let transitions = self.directions.iter().map(|direction| match direction {
            East => (1, 1, 0),
            SouthEast => (1, 0, -1),
            SouthWest => (0, -1, -1),
            West => (-1, -1, 0),
            NorthWest => (-1, 0, 1),
            NorthEast => (0, 1, 1),
        });

        transitions.fold((0, 0, 0), |cur, tr| {
            (cur.0 + tr.0, cur.1 + tr.1, cur.2 + tr.2)
        })
    }
}

fn match_first_ch(line: &String) -> (Option<Direction>, usize) {
    //println!("matching on {} ({})", line, line.len());
    if line.len() > 1 {
        match &line[0..2] {
            "nw" => return (Some(Direction::NorthWest), 2),
            "ne" => return (Some(Direction::NorthEast), 2),
            "sw" => return (Some(Direction::SouthWest), 2),
            "se" => return (Some(Direction::SouthEast), 2),
            _ => (),
        };
    }

    match &line[0..1] {
        "w" => return (Some(Direction::West), 1),
        "e" => return (Some(Direction::East), 1),
        _ => (),
    }

    (None, 0)
}

fn parse_line(line: String) -> Vec<Direction> {
    if line.len() < 1 {
        return vec![];
    }

    let this_dir = match_first_ch(&line);
    let mut dir = vec![this_dir.0.unwrap()];

    dir.extend(parse_line(line[this_dir.1..].to_string()));
    dir
}

fn read_input(lines: Lines<StdinLock>) -> Vec<Path> {
    lines
        .map(|li| li.unwrap())
        .map(|li| Path {
            directions: parse_line(li),
        })
        .collect()
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    let mut paths = read_input(lines);

    let mut m: HashMap<(i64, i64, i64), u64> = HashMap::new();
    for path in paths {
        let p = path.traverse();
        //println!("{:?}", p);
        *m.entry(p).or_default() += 1;
    }

    let count_black = m
        .values()
        .filter(|num_identified| *num_identified % 2 == 1)
        .count();
    println!("{}", count_black)
}
