use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead, Error, Lines, StdinLock};

fn read_input(lines: &mut Lines<StdinLock>) -> Vec<u64> {
    let numbers = lines.next().unwrap().unwrap();
    numbers.split(",").map(|n| n.parse().unwrap()).collect()
}

fn find_n(nums: &Vec<u64>, v: u64) -> u64 {
    let mut positions: HashMap<u64, (u64, Option<u64>)> = HashMap::new();
    let mut all = vec![0; v as usize];

    for i in 0..v {
        match nums.get(i as usize) {
            Some(u) => {
                positions.insert(*u, (i, None));
                all[i as usize] = *u;
            }
            None => {
                let last = all[(i - 1) as usize];

                let last_i = positions.get(&last);

                let u = match last_i {
                    Some((iii, ii)) => match *ii {
                        None => 0,
                        Some(ii) => iii - ii,
                    },
                    None => 0,
                };

                let last_u = positions.get(&u);
                let prev = match last_u {
                    Some((iii, _)) => Some(*iii),
                    None => None,
                };

                positions.insert(u, (i, prev));
                all[i as usize] = u;
            }
        }
    }
    all[v as usize - 1]
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let nums = read_input(&mut lines);
    let n = find_n(&nums, 2020);
    println!("{}", n);
    let n = find_n(&nums, 30000000);
    println!("{}", n);
}
