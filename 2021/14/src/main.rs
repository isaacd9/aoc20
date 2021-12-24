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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Pair {
    Standard(char, char),
}

impl Pair {
    fn to_string(&self) -> String {
        use Pair::*;
        match self {
            //Last(a, b) => iter::once(a).chain(iter::once(b)).collect(),
            Standard(a, b) => iter::once(a).chain(iter::once(b)).collect(),
        }
    }
}

fn apply_part_two(m: &HashMap<Pair, u64>, rules: &[Rule]) -> HashMap<Pair, u64> {
    use Pair::*;

    let mut r = HashMap::new();
    for (k, v) in m {
        let st: String = k.to_string();
        for rule in rules {
            if rule.pat == st {
                match k {
                    Standard(fst, snd) => {
                        *r.entry(Standard(*fst, rule.insertion)).or_insert(0) += v;
                        *r.entry(Standard(rule.insertion, *snd)).or_insert(0) += v;
                    }
                }
            }
        }
    }
    r
}

fn count_elements(template: &String) -> HashMap<char, u64> {
    let chs = template.chars();
    let mut m: HashMap<char, u64> = HashMap::new();
    for ch in chs {
        *m.entry(ch).or_default() += 1
    }
    m
}

fn max_min(template: &String) -> (u64, u64) {
    let mut min = std::u64::MAX;
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

fn max_min_2(m: &HashMap<Pair, u64>) -> (u64, u64) {
    let mut counts: HashMap<char, u64> = HashMap::new();
    for (k, v) in m {
        match k {
            &Pair::Standard(a, b) => {
                *counts.entry(a).or_default() += v;
                //*counts.entry(b).or_default() += v;
            }
        };
    }

    println!("{:?}", counts);
    (
        *counts.values().max().unwrap(),
        *counts.values().min().unwrap(),
    )
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|line| line.unwrap());

    let orig_template = lines.next().unwrap();
    let _ = lines.next();
    let rules: Vec<Rule> = lines.map(|line| Rule::parse(line)).collect();

    //println!("{:?}", template);
    //println!("{:?}", rules);

    let mut template = orig_template.clone();
    // Part 1
    for _ in 0..10 {
        template = apply(template, &rules);
        println!("{:?}", template.len());
    }
    let mm = max_min(&template);
    println!("linkedlist: {}-{}={:?}", mm.0, mm.1, mm.0 - mm.1);

    // Part 2
    let mut template_m = HashMap::new();
    for (i, window) in orig_template
        .chars()
        .collect::<Vec<char>>()
        .windows(2)
        .enumerate()
    {
        *template_m
            .entry(Pair::Standard(window[0], window[1]))
            .or_default() += 1;
    }
    println!("{:?}", template_m);

    for i in 0..40 {
        template_m = apply_part_two(&template_m, &rules);
        println!("{:?}", template_m.values().sum::<u64>() + 1);
    }
    //println!("{:?}", template_m);
    //println!("{:?}", template_m.values().sum::<u64>() + 1);
    let mm = max_min_2(&template_m);
    println!("map: {}-{}={:?}", mm.0, mm.1, mm.0 - mm.1);
}
