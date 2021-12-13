use std::io::{self, BufRead};

fn is_valid(line: &String) -> Option<char> {
    let mut stack: Vec<char> = vec![];

    for ch in line.chars() {
        match ch {
            '(' | '[' | '{' | '<' => stack.push(ch),
            ')' => {
                let opener = stack.pop();
                if opener != Some('(') {
                    return Some(ch);
                }
            }
            ']' => {
                let opener = stack.pop();
                if opener != Some('[') {
                    return Some(ch);
                }
            }
            '}' => {
                let opener = stack.pop();
                if opener != Some('{') {
                    return Some(ch);
                }
            }
            '>' => {
                let opener = stack.pop();
                if opener != Some('<') {
                    return Some(ch);
                }
            }
            _ => panic!("unexpected ch: {}", ch),
        }
    }
    None
}

fn correct(line: &String) -> Vec<char> {
    let mut stack: Vec<char> = vec![];

    for ch in line.chars() {
        match ch {
            '(' | '[' | '{' | '<' => stack.push(ch),
            ')' | ']' | '}' | '>' => {
                stack.pop();
                ()
            }
            _ => panic!("unexpected ch: {}", ch),
        }
    }

    let mut correction = vec![];
    stack.reverse();
    for ch in stack {
        let p = match ch {
            '{' => '}',
            '[' => ']',
            '(' => ')',
            '<' => '>',
            _ => panic!("unexpected ch: {}", ch),
        };

        correction.push(p)
    }

    correction
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(|line| line.unwrap());
    let lines: Vec<String> = lines.collect();

    // Part 1
    let invalid: Vec<Option<char>> = lines
        .clone()
        .iter()
        .map(|line| {
            let v = is_valid(line);
            v
        })
        .collect();

    let su: u32 = invalid
        .iter()
        .map(|res| match res {
            Some(')') => 3,
            Some(']') => 57,
            Some('}') => 1197,
            Some('>') => 25137,
            _ => 0,
        })
        .sum();

    println!("sum: {}", &su);

    // Part 2
    let corrected: Vec<Vec<char>> = lines
        .clone()
        .iter()
        .filter(|line| is_valid(line).is_none())
        .map(|line| {
            let c = correct(line);
            c
        })
        .collect();

    let mut scores: Vec<u64> = corrected
        .iter()
        .map(|correction| {
            let mut score = 0;
            for ch in correction {
                score *= 5;
                score += match ch {
                    ')' => 1,
                    ']' => 2,
                    '}' => 3,
                    '>' => 4,
                    _ => 0,
                }
            }
            score
        })
        .collect();

    scores.sort();

    println!("score: {}", scores[scores.len() / 2])
}
