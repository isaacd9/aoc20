use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead, Error, Lines, StdinLock};

use std::fmt;

#[derive(Debug, PartialEq, Clone)]
enum State {
    Empty,
    Occupied,
    Floor,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match &self {
            State::Empty => 'L',
            State::Occupied => '#',
            State::Floor => '.',
        };

        write!(f, "{}", c)
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Grid(Vec<Vec<State>>);

impl Grid {
    fn count_state(&self, state: State) -> u32 {
        let mut count_occupied = 0;
        for row in self.0.iter() {
            for col in row.iter() {
                if *col == state {
                    count_occupied += 1;
                }
            }
        }
        count_occupied
    }

    fn count_adjacent(&self, point: (usize, usize), expected_state: State) -> u32 {
        let mut num_adjacent = 0;
        for r in -1..=1 {
            for c in -1..=1 {
                if r == 0 && c == 0 {
                    continue;
                }

                let x = &self
                    .0
                    .get((point.0 as i32 + r) as usize)
                    .and_then(|v| v.get((point.1 as i32 + c) as usize));

                //println!("r: {}, c: {}, x: {:?}", r, c, x);
                match x {
                    Some(state) if **state == expected_state => num_adjacent += 1,
                    _ => (),
                }
            }
        }
        num_adjacent
    }

    fn count_first_occupied(&self, point: (usize, usize)) -> u32 {
        let mut num_adjacent = 0;

        for r in -1..=1 {
            for c in -1..=1 {
                if r == 0 && c == 0 {
                    continue;
                }

                let mut magnitude = 1;

                loop {
                    let x = &self
                        .0
                        .get((point.0 as i32 + (r * magnitude)) as usize)
                        .and_then(|v| v.get((point.1 as i32 + (c * magnitude)) as usize));

                    //println!("r: {}, c: {}, x: {:?}", r + magnitude, c + magnitude, x);
                    match x {
                        Some(state) => match state {
                            State::Empty => break,
                            State::Occupied => {
                                num_adjacent += 1;
                                break;
                            }
                            State::Floor => {
                                magnitude += 1;
                                continue;
                            }
                        },
                        None => {
                            break;
                        }
                    }
                }
            }
        }
        num_adjacent
    }

    fn iterate(&self) -> Grid {
        let mut g = self.clone();
        for (r, row) in self.0.iter().enumerate() {
            for (c, col) in row.iter().enumerate() {
                let adjacent_occupied = self.count_first_occupied((r, c));
                //println!("r: {}, c: {}, occupied: {:?}", r, c, adjacent_occupied);
                let new = match (col, adjacent_occupied) {
                    (State::Floor, _) => State::Floor,
                    (_, 0) => State::Occupied,
                    (_, x) if x >= 5 => State::Empty,
                    (_, _) => col.clone(),
                };
                g.0[r][c] = new
            }
        }

        g
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rows: Vec<String> = self
            .0
            .iter()
            .map(|line| line.iter().map(|pos| pos.to_string()).collect::<String>())
            .collect();

        write!(f, "{}", rows.join("\n"))
    }
}

fn read_input(lines: Lines<StdinLock>) -> Result<Grid, io::Error> {
    let g = lines
        .map(|li| li.unwrap())
        .map(|li| {
            li.chars()
                .map(|c| match c {
                    'L' => State::Empty,
                    '#' => State::Occupied,
                    '.' => State::Floor,
                    _ => panic!("unknown state: {}", c),
                })
                .collect()
        })
        .collect();

    Ok(Grid(g))
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let mut grid = read_input(lines).unwrap();
    let mut next = grid.iterate();

    //println!("{}", grid.iterate());
    //println!("---");
    //println!("{}", grid.iterate().iterate());

    while grid != next {
        grid = next;
        next = grid.iterate();
    }
    println!("{}", grid.count_state(State::Occupied))
}
