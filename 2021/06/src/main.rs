use std::collections::HashMap;
use std::hash::Hash;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|line| line.unwrap());
    let mut input: Vec<i32> = lines
        .nth(0)
        .unwrap()
        .split(",")
        .map(|st| st.parse().unwrap())
        .collect();

    let mut m: Vec<i64> = vec![0; 9];
    for fish_day in input {
        m[fish_day as usize] += 1;
    }

    // Part 1
    println!("Initial state: {:?}", m);
    for d in 1..=256 {
        let mut new_m = vec![0; 9];
        for fish_day in 0..9 {
            if fish_day == 0 {
                new_m[8] += m[0];
                new_m[6] += m[0];
            } else {
                new_m[fish_day - 1] += m[fish_day];
            }
        }
        m = new_m;
        println!("After {} day: {:?}", d, m);
    }

    println!("{}", m.into_iter().sum::<i64>());
}
