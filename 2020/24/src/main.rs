use std::collections::{HashMap, HashSet};
use std::fmt;
use std::io::{self, BufRead, Error, Lines, StdinLock};
use std::iter;
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

#[derive(Debug, PartialEq, Clone)]
enum Color {
    Black,
    White,
}

#[derive(Debug, PartialEq, Clone)]
struct Board {
    tiles: HashMap<(i64, i64, i64), Color>,
}

static COORDS: &'static [(i64, i64, i64)] = &[
    (1, 1, 0),
    (1, 0, -1),
    (0, -1, -1),
    (-1, -1, 0),
    (-1, 0, 1),
    (0, 1, 1),
];

impl Board {
    fn from_paths(paths: &[Path]) -> Board {
        let mut m: HashMap<(i64, i64, i64), Color> = HashMap::new();

        for path in paths {
            let p = path.traverse();
            m.entry(p)
                .and_modify(|c| {
                    *c = match c {
                        Color::Black => Color::White,
                        Color::White => Color::Black,
                    };
                })
                .or_insert(Color::Black);
        }

        Board { tiles: m }
    }

    fn count_black_adjacent(&self, coord: &(i64, i64, i64)) -> u64 {
        COORDS
            .iter()
            .map(|direction| {
                let moved = (
                    coord.0 + direction.0,
                    coord.1 + direction.1,
                    coord.2 + direction.2,
                );

                //println!("checking color of {:?}", moved);
                self.tiles.get(&moved).unwrap_or(&Color::White)
            })
            .filter(|color| **color == Color::Black)
            .count() as u64
    }

    fn iterate(&self) -> Board {
        let mut b = self.clone();

        for (key, _) in self.tiles.iter() {
            //println!("examining {:?}", key);
            for diff in COORDS.iter().chain(iter::once(&(0, 0, 0))) {
                let key_adj = (key.0 + diff.0, key.1 + diff.1, key.2 + diff.2);
                //let key_adj = (key.0, key.1, key.2);
                //println!("examining {:?}", key_adj);
                let adj = self.count_black_adjacent(&key_adj);
                //println!("{:?} has {} adjacent black tiles", key_adj, adj);

                let v = self.tiles.get(&key_adj).unwrap_or(&Color::White);

                let new = match (v, adj) {
                    (Color::Black, 0) => Color::White,
                    (Color::Black, 1) => Color::Black,
                    (Color::Black, 2) => Color::Black,
                    (Color::Black, _) => Color::White,
                    (Color::White, 2) => Color::Black,
                    (Color::White, _) => Color::White,
                };

                if new == Color::Black {
                    b.tiles.insert(key_adj, new);
                } else {
                    b.tiles.remove(&key_adj);
                }
            }
        }
        b
    }

    fn count_black(&self) -> u64 {
        self.tiles.values().filter(|c| **c == Color::Black).count() as u64
    }
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    let paths = read_input(lines);

    let mut board = Board::from_paths(&paths);

    println!("{}", board.count_black());

    for i in 0..=100 {
        println!("Day {}: {}", i, board.count_black());
        board = board.iterate();
    }
    //println!("{}", board.iterate().count_black());
    //println!("{}", board.iterate().iterate().count_black());
}
