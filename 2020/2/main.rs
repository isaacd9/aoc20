use std::io::{self, BufRead};

#[derive(Default)]
struct Policy {
    character: char,
    range: (u32, u32),
}

#[derive(Default)]
struct Password {
    policy: Policy,
    password: String,
}

fn parse_passwords() -> Result<Password, io::Error> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    for line in lines {
        println!("{}", line?)
    }

    Ok(Default::default())
}

fn main() {
    parse_passwords().unwrap();
}
