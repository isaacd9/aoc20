use std::io::{self, BufRead};

#[derive(Debug)]
struct Range(u32, u32);

impl Range {
    fn parse(st: &str) -> Result<Range, Box<dyn std::error::Error>> {
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
    fn parse(st: &str) -> Result<RangePair, Box<dyn std::error::Error>> {
        let mut sp = st.split(',');
        Ok(RangePair(
            Range::parse(sp.next().ok_or("no low")?)?,
            Range::parse(sp.next().ok_or("no hi")?)?,
        ))
    }

    fn fully_contained(&self) -> bool {
        self.0.contains(&self.1) || self.1.contains(&self.0)
    }

    fn any_overlap(&self) -> bool {
        (self.0 .0 <= self.1 .1) && (self.0 .1 >= self.1 .0)
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
    let n_contains: u32 = contains.iter().map(|f| u32::from(*f)).sum();
    println!("{:?}", n_contains);

    let overlaps: Vec<bool> = lines.iter().map(|rp| rp.any_overlap()).collect();
    let n_overlaps: u32 = overlaps.iter().map(|f| u32::from(*f)).sum();
    println!("{:?}", n_overlaps);
}
