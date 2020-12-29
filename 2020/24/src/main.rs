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
    println!("{:?}", paths)
}
