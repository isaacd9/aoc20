use std::collections::{HashSet, VecDeque};
use std::fmt;
use std::io::{self, BufRead};

#[derive(Debug, Clone)]
struct Octopi(Vec<Vec<u32>>);

impl Octopi {
    fn iterate(&self) -> (Octopi, HashSet<(usize, usize)>) {
        let mut ret = self.clone();

        let mut flashed: HashSet<(usize, usize)> = HashSet::new();

        for r in 0..self.0.len() {
            for c in 0..self.0[0].len() {
                // First, the energy level of each octopus increases by 1.
                ret.0[r][c] += 1;
            }
        }

        for r in 0..self.0.len() {
            for c in 0..self.0[0].len() {
                let mut flash_q: VecDeque<(usize, usize)> = VecDeque::from([(r, c)]);
                while flash_q.len() > 0 {
                    let pt = flash_q.pop_back().unwrap();

                    // Then, any octopus with an energy level greater than 9
                    // flashes.
                    if ret.0[pt.0][pt.1] < 10 || flashed.contains(&pt) {
                        continue;
                    }

                    // println!("flashing {:?}", pt);
                    // println!("{}", ret);

                    // This increases the energy level of all adjacent
                    // octopuses by 1, including octopuses that are diagonally
                    // adjacent. If this causes an octopus to have an energy level
                    // greater than 9, it also flashes. This process continues as
                    // long as new octopuses keep having their energy level
                    // increased beyond 9. (An octopus can only flash at most once
                    // per step.)
                    for tup_x in -1..=1 {
                        for tup_y in -1..=1 {
                            if tup_x == 0 && tup_y == 0 {
                                continue;
                            }

                            let bound_x = (pt.0 as i32 + tup_x) as usize;
                            let bound_y = (pt.1 as i32 + tup_y) as usize;

                            let bounder = ret.0.get(bound_x).and_then(|row| row.get(bound_y));

                            match bounder {
                                Some(_) => {
                                    ret.0[bound_x][bound_y] = ret.0[bound_x][bound_y] + 1;
                                    flash_q.push_back((bound_x, bound_y));
                                }
                                None => (),
                            }
                        }
                    }

                    // Finally, any octopus that flashed during this step has
                    // its energy level set to 0, as it used all of its energy
                    // to flash.
                    flashed.insert(pt);
                }
            }
        }

        for pt in &flashed {
            ret.0[pt.0][pt.1] = 0
        }

        (ret, flashed)
    }
}

impl fmt::Display for Octopi {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in &self.0 {
            writeln!(f, "{:?}", line)?
        }
        Ok(())
    }
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(|line| line.unwrap());

    let v: Vec<Vec<u32>> = lines
        .map(|line| line.chars().map(|ch| ch.to_digit(10).unwrap()).collect())
        .collect();

    let mut o = Octopi(v);
    let mut part_one_o = o.clone();
    let mut sum = 0;

    for i in 0..100 {
        let (new_o, f) = part_one_o.iterate();
        sum += f.len();
        part_one_o = new_o;
    }

    // Part 1
    println!("{}", o);
    println!("{}", sum);

    // Part 2
    let mut part_two_o = o.clone();
    let mut first_where_all_flash = None;

    for i in 0..10000 {
        let (new_o, f) = part_two_o.iterate();
        sum += f.len();
        part_two_o = new_o;

        //println!("{}", f.len());
        if first_where_all_flash.is_none() && f.len() == o.0.len() * o.0[0].len() {
            first_where_all_flash = Some(i + 1)
        }
    }
    println!("first where all flash: {:?}", first_where_all_flash);
}
