use std::collections::{HashMap, HashSet};
use std::fmt;
use std::io::{self, BufRead, Error, Lines, StdinLock};
use std::ops::RangeInclusive;

#[derive(Debug, PartialEq, Clone)]
enum Rule {
    NonTerminal(Vec<usize>),
    Terminal(char),
}

#[derive(Debug, PartialEq, Clone)]
struct Disjunction(Vec<Rule>);

#[derive(Debug, PartialEq, Clone)]
struct Grammar {
    rules: Vec<Disjunction>,
}

fn read_input(lines: Lines<StdinLock>) -> (Grammar, Vec<String>) {
    let mut unwrapped_lines = lines.map(|li| li.unwrap());

    let mut dj = unwrapped_lines
        .by_ref()
        .take_while(|li| li != "")
        .map(|li| {
            let mut sp = li.split(":");
            let index = sp.nth(0).unwrap();
            let disjunctions: Vec<Rule> = sp
                .nth(0)
                .unwrap()
                .trim()
                .split("|")
                .map(|dis| {
                    let rule_refs = dis.trim().split(" ").collect::<Vec<&str>>();
                    match rule_refs[0] {
                        "\"a\"" => Rule::Terminal('a'),
                        "\"b\"" => Rule::Terminal('b'),
                        _ => {
                            let c = rule_refs
                                .iter()
                                .map(|rr| rr.trim())
                                .map(|rule_ref| rule_ref.parse::<usize>().unwrap())
                                .collect();
                            Rule::NonTerminal(c)
                        }
                    }
                })
                .collect();

            (index.parse().unwrap(), Disjunction(disjunctions))
        })
        .collect::<Vec<(usize, Disjunction)>>();

    dj.sort_by(|(a, _), (b, _)| a.cmp(b));
    let lines: Vec<String> = unwrapped_lines.collect();
    (
        Grammar {
            rules: dj.into_iter().map(|item| item.1).collect(),
        },
        lines,
    )
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    let exprs = read_input(lines);

    //println!("{:?}", exprs);
}
