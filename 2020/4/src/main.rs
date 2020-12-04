use regex::Regex;
use std::collections::HashMap;
use std::io::{self, BufRead, Error, Lines, StdinLock};

#[derive(Default, Debug)]
struct Passport {
    fields: HashMap<String, String>,
}

const REQUIRED: [&'static str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

impl Passport {
    fn valid<T: Fn(String) -> bool>(&self, validators: &HashMap<String, Box<T>>) -> bool {
        for r in &REQUIRED {
            let data = self.fields.get(&r.to_string());

            match data {
                Some(d) => {
                    let v = validators.get(&r.to_string());
                    match v {
                        Some(validator) => println!("{}", validator(d.to_string())),
                        None => (),
                    }
                }
                None => {
                    return false;
                }
            }
        }
        true
    }
}

fn parse_passports(lines: Lines<StdinLock>) -> Result<Vec<Passport>, io::Error> {
    let re = Regex::new(r"(?P<key>\w+):(?P<value>\S+)").unwrap();

    let mut passports: Vec<Passport> = vec![];
    let mut peekable = lines.peekable();
    while peekable.peek().is_some() {
        let v: Vec<String> = peekable
            .by_ref()
            .map(|u| u.unwrap())
            .take_while(|line| line.ne(""))
            .collect();
        let st = v.join(" ");
        let mut passport: Passport = Default::default();

        for field in st.split_whitespace() {
            let caps = re.captures(field).unwrap();
            passport
                .fields
                .insert(caps["key"].to_string(), caps["value"].to_string());
        }

        passports.push(passport);
    }
    Ok(passports)
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    let passports = parse_passports(lines).unwrap();

    let mut validators: HashMap<String, Box<&dyn Fn(String) -> bool>> = HashMap::new();

    validators.insert(
        "byr".to_string(),
        Box::new(&|st: String| {
            println!("{}", st);
            let parsed = st.parse::<u32>().unwrap();
            parsed > 1920 && parsed < 2002
        }),
    );

    validators.insert(
        "iyr".to_string(),
        Box::new(&|st: String| {
            println!("{}", st);
            let parsed = st.parse::<u32>().unwrap();
            parsed > 1920 && parsed < 2002
        }),
    );

    let mut valid = 0;
    for passport in passports {
        if passport.valid(&validators) {
            valid += 1;
        }
    }
    println!("{:?}", valid)
}
