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
struct ThreeDeeSpace(Vec<Vec<Vec<Cube>>>);

#[derive(Debug, PartialEq, Clone)]
struct FourDeeSpace(Vec<Vec<Vec<Vec<Cube>>>>);

impl fmt::Display for ThreeDeeSpace {
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

impl fmt::Display for FourDeeSpace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let st: String = self
            .0
            .iter()
            .map(|z| {
                z.iter()
                    .map(|w| {
                        w.iter()
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
                    .join("\n\n")
            })
            .collect::<Vec<String>>()
            .join("\n\n");

        write!(f, "{}", st)
    }
}

impl FourDeeSpace {
    fn count_active(&self) -> u64 {
        self.0
            .iter()
            .flat_map(|hyper| {
                hyper
                    .iter()
                    .flat_map(|slice| slice.iter().flat_map(|row| row.iter().map(|cell| cell)))
            })
            .filter(|k| k == &&Cube::Active)
            .count() as u64
    }

    fn count_adjacent_active(&self, pos: (usize, usize, usize, usize)) -> u64 {
        (-1..=1)
            .flat_map(move |z| {
                (-1..=1).flat_map(move |s| {
                    (-1..=1).flat_map(move |r| {
                        (-1..=1).map(move |c| {
                            if z == 0 && s == 0 && r == 0 && c == 0 {
                                return None;
                            }

                            let zz = (pos.0 as i32 + z) as usize;
                            let ss = (pos.1 as i32 + s) as usize;
                            let rr = (pos.2 as i32 + r) as usize;
                            let cc = (pos.3 as i32 + c) as usize;

                            //println!("{},{},{}", ss, rr, cc);

                            let v = self.0.get(zz).and_then(|hyper| {
                                hyper
                                    .get(ss)
                                    .and_then(|slice| slice.get(rr).and_then(|row| row.get(cc)))
                            });

                            v
                        })
                    })
                })
            })
            .filter(|k| k == &Some(&Cube::Active))
            .count() as u64
    }

    fn expand(&mut self) {
        let old_len = self.0[0][0][0].len();

        for hyper in self.0.iter_mut() {
            for slice in hyper.iter_mut() {
                for row in slice.iter_mut() {
                    row.insert(0, Cube::Inactive);
                    row.push(Cube::Inactive);
                }

                slice.insert(0, vec![Cube::Inactive; old_len + 2]);
                slice.push(vec![Cube::Inactive; old_len + 2]);
            }

            hyper.insert(0, vec![vec![Cube::Inactive; old_len + 2]; old_len + 2]);
            hyper.push(vec![vec![Cube::Inactive; old_len + 2]; old_len + 2]);
        }

        self.0.insert(
            0,
            vec![vec![vec![Cube::Inactive; old_len + 2]; old_len + 2]; old_len + 2],
        );
        self.0.push(vec![
            vec![vec![Cube::Inactive; old_len + 2]; old_len + 2];
            old_len + 2
        ]);

        //println!("it: {:?}, {:?}", new.0, new.0.len());
    }

    fn iterate(&mut self) -> FourDeeSpace {
        use Cube::*;

        self.expand();
        let mut new = self.clone();

        for (z, hyper) in self.0.iter().enumerate() {
            for (s, slice) in hyper.iter().enumerate() {
                for (r, row) in slice.iter().enumerate() {
                    for (c, cell) in row.iter().enumerate() {
                        let adj_count = self.count_adjacent_active((z, s, r, c));

                        //println!("{},{},{}: {:?}", s, r, c, adj_count);

                        let new_cube = match (cell, adj_count) {
                            (Active, 2) => Active,
                            (Active, 3) => Active,
                            (Inactive, 3) => Active,
                            _ => Inactive,
                        };

                        new.0[z][s][r][c] = new_cube;
                    }
                }
            }
        }

        new
    }
}

impl ThreeDeeSpace {
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
            vec![Cube::Inactive; old_row_len + 2];
            old_slice_len + 2
        ]);
        //println!("it: {:?}, {:?}", new.0, new.0.len());
    }

    fn iterate(&mut self) -> ThreeDeeSpace {
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

fn read_input(lines: Lines<StdinLock>) -> Vec<Vec<Vec<Cube>>> {
    let slice = lines
        .map(|li| li.unwrap())
        .map(|li| li.chars().map(|c| Cube::from(&c)).collect::<Vec<Cube>>())
        .collect();

    vec![slice]
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    let mut cubes = read_input(lines);

    let mut three_dee = ThreeDeeSpace(cubes.clone());
    for _ in (0..6) {
        three_dee = three_dee.iterate()
    }
    println!("{}", three_dee.count_active());

    let mut four_dee = FourDeeSpace(vec![cubes]);
    for _ in (0..6) {
        four_dee = four_dee.iterate()
    }
    println!("{}", four_dee.count_active());
}
