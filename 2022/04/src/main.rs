use std::{
    collections::btree_map::Iter,
    io::{self, BufRead},
};

#[derive(Debug)]
struct Range(u32, u32);

impl Range {
    fn parse(st: &String) -> Result<Range, Box<dyn std::error::Error>> {
        let mut sp = st.split('-');
        Ok(Range(
            sp.next().ok_or("no low")?.parse()?,
            sp.next().ok_or("no hi")?.parse()?,
        ))
    }

    fn contains(&self, other: &Range) -> bool {
        self.0 <= other.0 && self.1 >= other.1
    }
}

#[derive(Debug)]
struct RangePair(Range, Range);

impl RangePair {
    fn parse(st: &String) -> Result<RangePair, Box<dyn std::error::Error>> {
        let mut sp = st.split(',');
        Ok(RangePair(
            Range::parse(&sp.next().ok_or("no low")?.to_string())?,
            Range::parse(&sp.next().ok_or("no hi")?.to_string())?,
        ))
    }

    fn fully_contained(&self) -> bool {
        self.0.contains(&self.1) || self.1.contains(&self.0)
    }
}

fn main() {
    let stdin = io::stdin();
    let lines: Vec<RangePair> = stdin
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .map(|line| RangePair::parse(&line).unwrap())
        .collect();

    let contains: Vec<bool> = lines.iter().map(|rp| rp.fully_contained()).collect();

    let n: u32 = contains.iter().map(|f| if *f { 1 } else { 0 }).sum();
    println!("{:?}", n);
}
