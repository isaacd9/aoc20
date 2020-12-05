use regex::Regex;
use std::collections::HashMap;
use std::io::{self, BufRead, Error, Lines, StdinLock};

#[derive(Default, Debug)]
struct Passport {
    fields: HashMap<String, String>,
}

const REQUIRED: [&'static str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
const EYE_COLORS: [&'static str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

impl Passport {
    fn valid(&self, validators: &HashMap<String, Box<dyn Fn(String) -> bool>>) -> bool {
        for r in &REQUIRED {
            let data = self.fields.get(&r.to_string());

            match data {
                Some(d) => {
                    let v = validators.get(&r.to_string());
                    match v {
                        Some(validator) => {
                            let res = validator(d.to_string());
                            if !res {
                                return false;
                            }
                            ()
                        }
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

    let mut validators: HashMap<String, Box<dyn Fn(String) -> bool>> = HashMap::new();

    validators.insert(
        "byr".to_string(),
        Box::new(|st: String| {
            let parsed = st.parse::<u32>().unwrap();
            parsed >= 1920 && parsed <= 2002
        }),
    );

    validators.insert(
        "iyr".to_string(),
        Box::new(|st: String| {
            let parsed = st.parse::<u32>().unwrap();
            parsed >= 2010 && parsed <= 2020
        }),
    );

    validators.insert(
        "eyr".to_string(),
        Box::new(|st: String| {
            let parsed = st.parse::<u32>().unwrap();
            parsed >= 2020 && parsed <= 2030
        }),
    );

    validators.insert(
        "hgt".to_string(),
        Box::new(|st: String| {
            let height_re = Regex::new(r"(?P<num>\d+)(?P<unit>in|cm)").unwrap();
            let matched = height_re.captures(st.as_str());
            match matched {
                Some(caps) => {
                    let parsed_num = caps["num"].parse::<u32>().unwrap();
                    match &caps["unit"] {
                        "cm" => parsed_num >= 150 && parsed_num <= 193,
                        "in" => parsed_num >= 59 && parsed_num <= 76,
                        _ => false,
                    }
                }
                None => false,
            }
        }),
    );

    validators.insert(
        "hcl".to_string(),
        Box::new(|st: String| {
            let hcl_re = Regex::new(r"#(?P<hcl>[0-9a-f]+)").unwrap();
            let matched = hcl_re.captures(st.as_str());

            match matched {
                Some(caps) => caps["hcl"].len() == 6,
                None => false,
            }
        }),
    );

    validators.insert(
        "ecl".to_string(),
        Box::new(|st: String| EYE_COLORS.contains(&st.as_str())),
    );

    validators.insert(
        "pid".to_string(),
        Box::new(|st: String| {
            let pid_re = Regex::new(r"(?P<pid>\d+)").unwrap();
            let matched = pid_re.captures(st.as_str());

            match matched {
                Some(caps) => caps["pid"].len() == 9,
                None => false,
            }
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
