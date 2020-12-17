use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead, Error, Lines, StdinLock};
use std::ops::RangeInclusive;

#[derive(PartialEq, Clone, Debug)]
struct Field {
    name: String,
    rules: Vec<RangeInclusive<u64>>,
}

#[derive(PartialEq, Clone, Debug)]
struct Fields(Vec<Field>);

#[derive(PartialEq, Clone, Debug)]
struct Ticket(Vec<u64>);

fn read_tickets(
    unwrapped_lines: &mut dyn Iterator<Item = String>,
) -> impl Iterator<Item = Ticket> + '_ {
    unwrapped_lines
        .take_while(|li| !li.eq(""))
        .skip(1)
        .map(|li| {
            Ticket(
                li.split(",")
                    .map(|num| num.parse::<u64>().unwrap())
                    .collect(),
            )
        })
}

fn read_input(lines: &mut Lines<StdinLock>) -> (Fields, Ticket, Vec<Ticket>) {
    let mut unwrapped_lines = lines.map(|li| li.unwrap());

    let field_re = Regex::new(r"(?P<name>\w+): (?P<rule>.*+)").unwrap();
    let range_re = Regex::new(r"(?P<low>\d+)-(?P<high>\d+)").unwrap();

    let fields = unwrapped_lines
        .by_ref()
        .take_while(|li| !li.eq(""))
        .map(|li| {
            let caps = field_re.captures(&li).unwrap();

            let rules = caps["rule"]
                .split("or")
                .map(|r| {
                    let range = range_re.captures(&r).unwrap();
                    RangeInclusive::new(
                        range["low"].parse().unwrap(),
                        range["high"].parse().unwrap(),
                    )
                })
                .collect();

            Field {
                name: caps["name"].to_string(),
                rules: rules,
            }
        })
        .collect();

    let my_ticket = read_tickets(&mut unwrapped_lines).nth(0).unwrap();
    let other_tickets = read_tickets(&mut unwrapped_lines.skip(1)).collect();

    (Fields(fields), my_ticket, other_tickets)
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let res = read_input(&mut lines);
    println!("{:?}", res);
}
