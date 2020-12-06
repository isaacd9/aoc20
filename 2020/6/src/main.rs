use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead, Error, Lines, StdinLock};

struct Group {
    questions_answered: Vec<HashSet<char>>,
}

fn count_questions(lines: Lines<StdinLock>) -> Result<Vec<u32>, io::Error> {
    let mut count_by_group: Vec<u32> = vec![];
    let mut unwrapped = lines.map(|li| li.unwrap());

    let mut cur_group: HashMap<char, u32> = HashMap::new();
    let mut group_size = 0;

    for li in unwrapped {
        if li == "" {
            let qs = cur_group.iter().filter(|q| *q.1 == group_size).count();
            count_by_group.push(qs as u32);
            cur_group = HashMap::new();
            group_size = 0;
        } else {
            group_size += 1;
        }

        for c in li.chars() {
            *cur_group.entry(c).or_insert(0) += 1;
        }
    }
    let qs = cur_group.iter().filter(|q| *q.1 == group_size).count();
    count_by_group.push(qs as u32);

    //println!("{:?}", count_by_group);
    Ok(count_by_group)
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let counts = count_questions(lines).unwrap();
    let mut total = 0;
    for count in counts {
        total += count;
    }
    println!("{:?}", total)
}
