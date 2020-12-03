use regex::Regex;
use std::io::{self, BufRead};

#[derive(Default, Debug)]
struct Policy {
    character: char,
    range: (u32, u32),
}

#[derive(Default, Debug)]
struct Password {
    policy: Policy,
    password: String,
}

impl Policy {
    fn matches_first_policy(&self, password: &String) -> bool {
        let c = password.chars().filter(|c| *c == self.character).count();
        let ok = c >= self.range.0 as usize && c <= self.range.1 as usize;
        //println!("range: {:?}, count: {}, ok: {}", self.range, c, ok);
        ok
    }

    fn matches_second_policy(&self, password: &String) -> bool {
        let mut count = 0;
        let first_c = password.chars().nth(self.range.0 as usize - 1);
        if first_c == Some(self.character) {
            count += 1;
        }

        let second_c = password.chars().nth(self.range.1 as usize - 1);
        if second_c == Some(self.character) {
            count += 1;
        }

        println!(
            "first_c ({:?}): {:?}, second_c ({:?}): {:?}, pw: {:?}",
            first_c, self.range.0, second_c, self.range.1, password,
        );
        count == 1
    }
}

impl Password {
    fn valid(&self) -> bool {
        self.policy.matches_second_policy(&self.password)
    }
}

fn parse_passwords() -> Result<Vec<Password>, io::Error> {
    let re = Regex::new(r"(?P<min>\d+)-(?P<max>\d+) (?P<ch>\w): (?P<pw>\w+)").unwrap();

    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let passwords: Vec<Password> = lines
        .map(|line| {
            let li = line.unwrap();
            let caps = re.captures(li.as_str()).unwrap();
            Password {
                policy: Policy {
                    character: caps["ch"].chars().nth(0).unwrap(),
                    range: (caps["min"].parse().unwrap(), caps["max"].parse().unwrap()),
                },
                password: caps["pw"].to_string(),
            }
        })
        .collect();

    Ok(passwords)
}

fn main() {
    let parsed_passwords = parse_passwords().unwrap();
    let mut count_valid = 0;
    for pw in parsed_passwords {
        if pw.valid() {
            count_valid += 1;
        }
    }

    println!("{}", count_valid)
}
