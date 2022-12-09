use std::{
    collections::HashSet,
    io::{self, BufRead},
    iter,
};

fn main() {
    let stdin = io::stdin();
    let compartments: Vec<(HashSet<char>)> = stdin
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            let len = line.len();
            line.chars().collect()
        })
        .collect();

    let commonalities: Vec<char> = compartments
        .chunks(3)
        .map(|chunk| {
            let mut set = chunk[0].clone();
            for (snd) in chunk {
                set = set.intersection(snd).copied().collect();
            }
            *set.iter().next().unwrap()
        })
        .collect();

    println!("{:?}", commonalities);
    let sum: u32 = commonalities
        .iter()
        .map(|c| {
            if c.is_lowercase() {
                *c as u32 - 96
            } else {
                26 + (*c as u32 - 64)
            }
        })
        .map(|c| {
            println!("{:?}", c);
            c
        })
        .sum();

    println!("{:?}", sum);
}
