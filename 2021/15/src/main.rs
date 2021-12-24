use std::{
    fmt::Display,
    io::{self, BufRead, Read},
};

#[derive(Debug)]
struct Cave(Vec<Vec<u32>>);

impl Display for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in self.0.iter() {
            writeln!(
                f,
                "{}",
                line.iter().map(|i| i.to_string()).collect::<String>()
            )?
        }
        Ok(())
    }
}

impl Cave {
    fn parse(st: &String) -> Self {
        let v: Vec<Vec<u32>> = st
            .lines()
            .map(|line| {
                line.chars()
                    .map(|ch| ch.to_string().parse().unwrap())
                    .collect()
            })
            .collect();
        Cave(v)
    }
}

fn main() {
    let stdin = io::stdin();
    //let mut lines = stdin.lock().lines().map(|line| line.unwrap());
    let mut st = String::new();
    stdin.lock().read_to_string(&mut st).unwrap();

    let cave = Cave::parse(&st);
    println!("{}", cave);
}
