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

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(|line| line.unwrap());
    let lines: Vec<String> = lines.collect();

    let invalid: Vec<Option<char>> = lines
        .iter()
        .map(|line| {
            let v = is_valid(line);
            println!("{}: {:?}", line, v);
            v
        })
        .collect();

    // Part 1
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

    println!("sum: {}", &su)
}
