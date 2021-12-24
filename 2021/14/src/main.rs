#![feature(linked_list_cursors)]

use std::{
    collections::{HashMap, LinkedList},
    i32::MAX,
    io::{self, BufRead},
    iter,
};

#[derive(Debug, Clone)]
struct Rule {
    pat: String,
    insertion: char,
}

impl Rule {
    fn parse(st: String) -> Self {
        let mut sp = st.split(" -> ");
        Rule {
            pat: sp.next().unwrap().to_string(),
            insertion: sp.next().unwrap().chars().next().unwrap(),
        }
    }
}

fn apply(template: String, rules: &[Rule]) -> String {
    let mut chrs: LinkedList<char> = template.chars().collect();
    let mut cursor = chrs.cursor_front_mut();
    while cursor.current().is_some() && cursor.peek_next().is_some() {
        let cur = *cursor.current().unwrap();
        let next = *cursor.peek_next().unwrap();
        let st: String = iter::once(cur).chain(iter::once(next)).collect();

        for rule in rules {
            if rule.pat == st {
                cursor.insert_after(rule.insertion);
                cursor.move_next();
                continue;
            }
        }

        cursor.move_next();
    }

    chrs.iter().collect()
}

fn apply_part_two(m: HashMap<String, u32>, rules: &[Rule]) -> HashMap<String, u32> {
    m
}

fn count_elements(template: &String) -> HashMap<char, u32> {
    let chs = template.chars();
    let mut m: HashMap<char, u32> = HashMap::new();
    for ch in chs {
        *m.entry(ch).or_default() += 1
    }
    m
}

fn max_min(template: &String) -> (u32, u32) {
    let mut min = std::u32::MAX;
    let mut max = 0;
    for v in count_elements(template).values() {
        if *v < min {
            min = *v
        }
        if *v > max {
            max = *v
        }
    }
    (max, min)
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|line| line.unwrap());

    let mut template = lines.next().unwrap();
    let _ = lines.next();
    let rules: Vec<Rule> = lines.map(|line| Rule::parse(line)).collect();

    //println!("{:?}", template);
    //println!("{:?}", rules);

    // Part 1
    for _ in 0..10 {
        template = apply(template, &rules);
    }
    let mm = max_min(&template);
    println!("{}-{}={:?}", mm.0, mm.1, mm.0 - mm.1);

    // Part 2
    for i in 11..=40 {
        println!("step {}", i);
        template = apply(template, &rules);
    }
    let mm = max_min(&template);
    println!("{}-{}={:?}", mm.0, mm.1, mm.0 - mm.1);
}
