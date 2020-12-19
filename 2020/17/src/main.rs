use std::collections::{HashMap, HashSet};
use std::fmt;
use std::io::{self, BufRead, Error, Lines, StdinLock};
use std::ops::RangeInclusive;

#[derive(Debug, PartialEq, Clone)]
enum Cube {
    Inactive,
    Active,
}

impl From<&char> for Cube {
    fn from(c: &char) -> Self {
        use Cube::*;

        match c {
            '.' => Inactive,
            '#' => Active,
            _ => panic!("unexpected char {}", c),
        }
    }
}

impl fmt::Display for Cube {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Cube::*;

        match &self {
            Inactive => write!(f, "."),
            Active => write!(f, "#"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Space(Vec<Vec<Vec<Cube>>>);

impl fmt::Display for Space {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let st: String = self
            .0
            .iter()
            .map(|slice| {
                slice
                    .iter()
                    .map(|row| {
                        row.iter()
                            .map(|cube| format!("{}", cube))
                            .collect::<Vec<String>>()
                            .join("")
                    })
                    .collect::<Vec<String>>()
                    .join("\n")
            })
            .collect::<Vec<String>>()
            .join("\n\n");

        write!(f, "{}", st)
    }
}

impl Space {
    fn count_active(&self) -> u64 {
        self.0
            .iter()
            .flat_map(|slice| slice.iter().flat_map(|row| row.iter().map(|cell| cell)))
            .filter(|k| k == &&Cube::Active)
            .count() as u64
    }

    fn count_adjacent_active(&self, pos: (usize, usize, usize)) -> u64 {
        (-1..=1)
            .flat_map(|s| {
                (-1..=1).flat_map(move |r| {
                    (-1..=1).map(move |c| {
                        if s == 0 && r == 0 && c == 0 {
                            return None;
                        }

                        let ss = (pos.0 as i32 + s) as usize;
                        let rr = (pos.1 as i32 + r) as usize;
                        let cc = (pos.2 as i32 + c) as usize;

                        //println!("{},{},{}", ss, rr, cc);

                        let v = self
                            .0
                            .get(ss)
                            .and_then(|slice| slice.get(rr).and_then(|row| row.get(cc)));

                        v
                    })
                })
            })
            .filter(|k| k == &Some(&Cube::Active))
            .count() as u64
    }

    fn expand(&mut self) {
        let old_row_len = self.0[0][0].len();
        let old_slice_len = self.0[0].len();

        for slice in self.0.iter_mut() {
            for row in slice.iter_mut() {
                row.insert(0, Cube::Inactive);
                row.push(Cube::Inactive);
            }

            slice.insert(0, vec![Cube::Inactive; old_row_len + 2]);
            slice.push(vec![Cube::Inactive; old_row_len + 2]);
        }

        self.0.insert(
            0,
            vec![vec![Cube::Inactive; old_row_len + 2]; old_slice_len + 2],
        );
        self.0.push(vec![
            vec![Cube::Inactive; old_slice_len + 2];
            old_slice_len + 2
        ]);
        //println!("it: {:?}, {:?}", new.0, new.0.len());
    }

    fn iterate(&mut self) -> Space {
        use Cube::*;

        self.expand();
        let mut new = self.clone();

        for (s, slice) in self.0.iter().enumerate() {
            for (r, row) in slice.iter().enumerate() {
                for (c, cell) in row.iter().enumerate() {
                    let adj_count = self.count_adjacent_active((s, r, c));

                    //println!("{},{},{}: {:?}", s, r, c, adj_count);

                    let new_cube = match (cell, adj_count) {
                        (Active, 2) => Active,
                        (Active, 3) => Active,
                        (Inactive, 3) => Active,
                        _ => Inactive,
                    };

                    new.0[s][r][c] = new_cube;
                }
            }
        }

        new
    }
}

fn read_input(lines: Lines<StdinLock>) -> Space {
    let slice = lines
        .map(|li| li.unwrap())
        .map(|li| li.chars().map(|c| Cube::from(&c)).collect())
        .collect();

    Space(vec![slice])
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    let mut cube = read_input(lines);
    println!("{}", cube);

    for _ in (0..6) {
        cube = cube.iterate()
    }
    println!("After 6 cycles:");
    println!("{}", cube.count_active());
}
