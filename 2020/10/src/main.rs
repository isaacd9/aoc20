use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead, Error, Lines, StdinLock};

fn read_input(lines: Lines<StdinLock>) -> Result<Vec<u32>, io::Error> {
    let numbers = lines
        .map(|li| li.unwrap())
        .map(|li| li.parse().unwrap())
        .collect();

    Ok(numbers)
}

fn not_factorial(num: u64) -> u128 {
    match num {
        4 => 7,
        3 => 4,
        2 => 2,
        1 => 1,
        _ => panic!("oh no"),
    }
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    let mut numbers = read_input(lines).unwrap();

    numbers.push(0);
    numbers.sort();
    numbers.push(numbers.last().unwrap() + 3);

    let differences: Vec<u32> = numbers
        .windows(2)
        .map(|window| window[1] - window[0])
        .collect();

    println!("{:?}", differences);
    let mut m: HashMap<u32, u32> = HashMap::new();
    for diff in &differences {
        *m.entry(*diff).or_default() += 1
    }

    let mut runs: Vec<u64> = vec![];
    let mut cur_run = 1;

    for i in (1..differences.len()) {
        if differences[i] == 1 && differences[i - 1] == differences[i] {
            cur_run += 1;
        } else {
            if cur_run > 1 {
                runs.push(cur_run);
                cur_run = 1;
            }
        }
    }

    println!("{:?}", differences);
    println!("{:?}", runs);
    let tot: u128 = runs.iter().fold(1, |acc, x| acc * not_factorial(*x));
    println!("{:?}", tot);
}
