use std::collections::HashMap;
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

    // for fish in input {}

    // Part 1
    println!("Initial state: {:?}", input);
    for d in 1..=256 {
        for fish_n in 0..input.len() {
            if input[fish_n] == 0 {
                input[fish_n] = 6;
                input.push(8)
            } else {
                input[fish_n] -= 1;
            }
        }
        println!("After {} day", d);
    }

    println!("{:?}", input);
    println!("{}", input.len());
}
