use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead, Error, Lines, StdinLock};

#[derive(Default, Debug)]
struct Group {
    questions_answered: Vec<HashSet<char>>,
}

impl Group {
    fn count_by_question(&self) -> HashMap<char, u32> {
        let mut hm = HashMap::new();
        for person in &self.questions_answered {
            for q in person {
                *hm.entry(*q).or_insert(0) += 1;
            }
        }
        hm
    }

    fn anyone_answered_yes(&self) -> u32 {
        self.count_by_question().len() as u32
    }

    fn everyone_answered_yes(&self) -> u32 {
        self.count_by_question()
            .iter()
            .filter(|(_, c)| **c == self.questions_answered.len() as u32)
            .count() as u32
    }
}

fn count_questions(lines: Lines<StdinLock>) -> Result<Vec<Group>, io::Error> {
    let unwrapped = lines.map(|li| li.unwrap());

    let mut cur_group: Group = Default::default();
    let mut groups: Vec<Group> = vec![];

    for li in unwrapped {
        if li == "" {
            groups.push(cur_group);
            cur_group = Default::default();
        } else {
            let mut hs: HashSet<char> = HashSet::new();
            for c in li.chars() {
                hs.insert(c);
            }
            cur_group.questions_answered.push(hs);
        }
    }

    groups.push(cur_group);
    Ok(groups)
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let groups = count_questions(lines).unwrap();
    let mut part_one = 0;
    let mut part_two = 0;

    for group in groups {
        part_one += group.anyone_answered_yes();
        part_two += group.everyone_answered_yes();
    }
    println!("{}", part_one);
    println!("{}", part_two);
}
