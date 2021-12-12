use std::collections::{HashMap, HashSet};
use std::fmt;
use std::io::{self, BufRead};

#[derive(Debug)]
struct Heatmap(Vec<Vec<u32>>);

impl Heatmap {
    fn find_low_points(&self) -> Vec<(usize, usize)> {
        let mut low_points = vec![];
        for r in 0..self.0.len() {
            for c in 0..self.0[0].len() {
                let height = self.0[r][c];

                let mut low_point = true;
                for tup in [(0, -1), (-1, 0), (1, 0), (0, 1)] {
                    //self.0[(r as i32 + tup.0) as usize][(c as i32 + tup.1) as usize];
                    let bounder = self
                        .0
                        .get((r as i32 + tup.0) as usize)
                        .and_then(|row| row.get((c as i32 + tup.1) as usize));

                    match bounder {
                        Some(b) => low_point &= &height < b,
                        None => (),
                    }
                }

                if low_point {
                    low_points.push((r, c))
                }
            }
        }

        low_points
    }

    fn find_basin_size(&self, point: &(usize, usize)) -> usize {
        let mut q = vec![point.clone()];

        let mut in_basin: HashSet<(usize, usize)> = HashSet::new();
        in_basin.insert(point.clone());

        while q.len() > 0 {
            let pt = q.pop().unwrap();

            for tup in [(0, -1), (-1, 0), (1, 0), (0, 1)] {
                let bounder = self
                    .0
                    .get((pt.0 as i32 + tup.0) as usize)
                    .and_then(|row| row.get((pt.1 as i32 + tup.1) as usize));

                match bounder {
                    Some(v) => {
                        // 9s are special
                        if *v == 9 {
                            continue;
                        };

                        if *v > self.0[pt.0][pt.1] {
                            //println!(
                            //    "adding to basin bounder: {:?}. this: {:?}",
                            //    bounder, self.0[pt.0][pt.1]
                            //);

                            in_basin.insert((
                                (pt.0 as i32 + tup.0) as usize,
                                (pt.1 as i32 + tup.1) as usize,
                            ));
                            q.push((
                                (pt.0 as i32 + tup.0) as usize,
                                (pt.1 as i32 + tup.1) as usize,
                            ))
                        }
                    }
                    None => (),
                }
            }
        }

        in_basin.len()
    }
}

impl fmt::Display for Heatmap {
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

    let heatmap = Heatmap(v);

    let lp = heatmap.find_low_points();
    //println!("{:?}", lp);
    println!("{:?}", lp.len());

    // Part 1
    let s: u32 = lp.iter().map(|p| heatmap.0[p.0][p.1] + 1).sum();
    //println!("{:?}", s);

    let mut basins: Vec<usize> = lp.iter().map(|pt| heatmap.find_basin_size(pt)).collect();
    basins.sort_by(|a, b| b.cmp(a));

    //println!("basins: {:?}", basins);
    println!(
        "{}*{}*{}={}",
        basins[0],
        basins[1],
        basins[2],
        basins[0] * basins[1] * basins[2],
    );
}
