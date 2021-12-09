use std::collections::HashMap;
use std::hash::Hash;
use std::io::{self, BufRead};

fn median(arr: &[i64]) -> i64 {
    arr[arr.len() / 2]
}

fn summation(i: i64) -> i64 {
    (0..=i).sum()
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|line| line.unwrap());
    let mut input: Vec<i64> = lines
        .nth(0)
        .unwrap()
        .split(",")
        .map(|st| st.parse().unwrap())
        .collect();

    // Part 1
    input.sort();
    //println!("{:?}", median(&input));
    let m = median(&input);
    let sum: i64 = input.iter().map(|n| i64::abs(n - m)).sum();

    //println!("{:?}", input);
    //println!("{:?}", sum);

    // Part 2
    let mut min: i64 = std::i64::MAX;
    let mut min_n = 0;
    for n in 0..3000 {
        let sum = input.iter().map(|v| summation(i64::abs(v - n))).sum();
        if sum < min {
            min = sum;
            min_n = n;
        }
    }
    println!("{:?}", min);
    println!("{:?}", min_n);
}
