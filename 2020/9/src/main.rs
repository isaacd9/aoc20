use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead, Error, Lines, StdinLock};

fn read_input(lines: Lines<StdinLock>) -> Result<Vec<i64>, io::Error> {
    let numbers = lines
        .map(|li| li.unwrap())
        .map(|li| li.parse().unwrap())
        .collect();

    Ok(numbers)
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let all_numbers = read_input(lines).unwrap();
    let predicate_len = 25;

    for (index, number) in all_numbers.iter().enumerate().skip(predicate_len) {
        let mut required: HashSet<i64> = HashSet::new();

        for i in 0..predicate_len {
            let ix = index as i32 - i as i32 - 1;
            required.insert(*number - all_numbers[ix as usize]);
        }

        let mut found = false;
        for i in 0..predicate_len {
            let ix = index as i32 - i as i32 - 1;
            found |= required.get(&all_numbers[ix as usize]).is_some();
        }

        if !found {
            println!("{}, {}", number, found)
        }
    }

    let target = 29221323;

    let mut acc = 0;
    let mut bottom_i = 0;
    let mut top_i = 1;
    for (index, number) in all_numbers.iter().enumerate() {
        if acc == target {
            break;
        }

        acc += number;
        top_i = index;

        while acc > target {
            acc -= all_numbers[bottom_i];
            bottom_i += 1;
        }
    }

    println!("bottom: {}, top: {}", bottom_i, top_i);

    let mut min = std::i64::MAX;
    let mut max = 0;
    for i in bottom_i..top_i + 1 {
        if all_numbers[i] > max {
            max = all_numbers[i]
        }

        if all_numbers[i] < min {
            min = all_numbers[i]
        }
    }

    println!("min: {}, max: {}, sum: {}", min, max, min + max)
}
