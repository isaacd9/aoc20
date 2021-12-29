use regex::Regex;
use std::result::Result::Err;

use std::{
    error,
    io::{self, Read},
};

#[derive(Debug)]
struct TargetArea {
    x: (i32, i32),
    y: (i32, i32),
}

impl TargetArea {
    fn parse(st: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let re = Regex::new(r"target area: x=([0-9\-]+)..([0-9\-]+), y=([0-9\-]+)..([0-9\-]+)")?;
        let caps = re.captures(&st);
        match caps {
            Some(c) => Ok(TargetArea {
                x: (c[1].parse()?, c[2].parse()?),
                y: (c[3].parse()?, c[4].parse()?),
            }),
            None => Result::Err("oh no".to_string().into()),
        }
    }
}

fn main() {
    let stdin = io::stdin();

    let mut st = String::new();
    stdin.lock().read_to_string(&mut st).unwrap();

    println!("{:?}", TargetArea::parse(&st.as_str()).unwrap());
}
