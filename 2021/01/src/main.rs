use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let nums: Vec<i32> = lines
        .map(|line| line.unwrap())
        .map(|line| line.parse().unwrap())
        .collect();

    let part_1: i32 = nums
        .windows(2)
        .map(|window| if window[1] > window[0] { 1 } else { 0 })
        .sum();

    println!("{}", part_1);

    let three_sums: Vec<i32> = nums.windows(3).map(|window| window.iter().sum()).collect();
    //println!("{:?}", three_sums);
    let part_2: i32 = three_sums
        .windows(2)
        .map(|window| if window[1] > window[0] { 1 } else { 0 })
        .sum();
    println!("{}", part_2);
}
