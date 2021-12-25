use std::{
    collections::{BinaryHeap, HashSet, VecDeque},
    fmt::Display,
    io::{self, BufRead, Read},
};

#[derive(Debug)]
struct Cave(Vec<Vec<u32>>);

impl Display for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in self.0.iter() {
            writeln!(
                f,
                "{}",
                line.iter().map(|i| i.to_string()).collect::<String>()
            )?
        }
        Ok(())
    }
}

impl Cave {
    fn parse(st: &String) -> Self {
        let v: Vec<Vec<u32>> = st
            .lines()
            .map(|line| {
                line.chars()
                    .map(|ch| ch.to_string().parse().unwrap())
                    .collect()
            })
            .collect();
        Cave(v)
    }

    pub(crate) fn find_lowest_risk_path(&self) -> Vec<(usize, usize)> {
        let mut distances: Vec<Vec<u32>> = vec![vec![u32::MAX; self.0[0].len()]; self.0.len()];
        let mut parents: Vec<Vec<Option<(usize, usize)>>> =
            vec![vec![None; self.0[0].len()]; self.0.len()];
        distances[0][0] = self.0[0][0];

        let mut q: BinaryHeap<(i32, (usize, usize))> =
            std::collections::BinaryHeap::from([(-(distances[0][0] as i32), (0, 0))]);
        let mut visited = HashSet::new();
        while !q.is_empty() {
            let cur = q.pop().unwrap().1;
            if visited.contains(&cur) {
                continue;
            }

            let cur_dist = distances[cur.0][cur.1];

            for adj in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let n = (
                    (cur.0 as i32 + adj.0) as usize,
                    (cur.1 as i32 + adj.1) as usize,
                );

                let neighbor = distances.get(n.0).and_then(|row| row.get(n.1));
                match neighbor {
                    Some(v) => {
                        let weight = self.0[n.0][n.1];
                        let neighbor_dist = cur_dist + weight;
                        if neighbor_dist < *v {
                            distances[n.0][n.1] = neighbor_dist;
                            parents[n.0][n.1] = Some(cur);
                        }

                        q.push((-(neighbor_dist as i32), n));
                    }
                    None => continue,
                }
            }

            visited.insert(cur);
        }

        for line in &distances {
            //println!("{:?}", line);
        }
        let mut cur = (self.0.len() - 1, self.0[0].len() - 1);
        let mut st = vec![];
        while cur != (0, 0) {
            //println!("moving to {:?} ({})", cur, self.0[cur.0][cur.1]);
            st.push(cur);
            cur = parents[cur.0][cur.1].unwrap();
        }
        st.reverse();
        st
    }

    fn tile(&self, n: usize) -> Cave {
        let mut cave: Vec<Vec<u32>> = vec![vec![0; self.0[0].len() * n]; self.0.len() * n];
        for row_tile in 0..n {
            for col_tile in 0..n {
                let row_offset = row_tile * self.0.len();
                let col_offset = col_tile * self.0[0].len();
                let multipler = u32::max(row_tile as u32, col_tile as u32);
                for (row, row_v) in self.0.iter().enumerate() {
                    for (col, col_v) in row_v.iter().enumerate() {
                        let mut new_v = *col_v + multipler;
                        if new_v > 9 {
                            new_v = (*col_v + multipler) % 10 + 1;
                        }
                        cave[row_offset + row][col_offset + col] = new_v;
                    }
                }
            }
        }
        Cave(cave)
    }
}

fn main() {
    let stdin = io::stdin();
    //let mut lines = stdin.lock().lines().map(|line| line.unwrap());
    let mut st = String::new();
    stdin.lock().read_to_string(&mut st).unwrap();

    let cave = Cave::parse(&st);
    let path = cave.find_lowest_risk_path();

    // Part 1
    let mut sum = 0;
    for p in path {
        sum += cave.0[p.0][p.1];
    }
    //println!("{:?}", sum);

    // Part 2
    let new_cave = cave.tile(5);
    println!("{}", new_cave);
}
