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

mod test {
    use super::*;

    #[test]
    fn test_matches_helper() {
        use crate::Rule::*;

        let g = Grammar {
            rules: vec![Disjunction(vec![Terminal('a')])],
        };

        assert_eq!(g.matches(&"a".to_string()), true);
        assert_eq!(g.matches(&"b".to_string()), false);
        assert_eq!(g.matches(&"ab".to_string()), false);
        assert_eq!(g.matches(&"ba".to_string()), false);
        assert_eq!(g.matches(&"aab".to_string()), false);
        assert_eq!(g.matches(&"bba".to_string()), false);

        let g = Grammar {
            rules: vec![
                Disjunction(vec![NonTerminal(vec![1, 2])]),
                Disjunction(vec![Terminal('a')]),
                Disjunction(vec![Terminal('b')]),
            ],
        };

        assert_eq!(g.matches(&"a".to_string()), false);
        assert_eq!(g.matches(&"b".to_string()), false);
        assert_eq!(g.matches(&"ab".to_string()), true);

        let g = Grammar {
            rules: vec![
                Disjunction(vec![NonTerminal(vec![1]), NonTerminal(vec![2])]),
                Disjunction(vec![Terminal('a')]),
                Disjunction(vec![Terminal('b')]),
            ],
        };

        assert_eq!(g.matches(&"a".to_string()), true);
        assert_eq!(g.matches(&"b".to_string()), true);
        assert_eq!(g.matches(&"ab".to_string()), false);
    }
}

impl Grammar {
    fn matches_helper(&self, ss: &[char], rules: &[Disjunction]) -> Option<usize> {
        use crate::Rule::*;

        let d = &rules[0];
        //println!("evaluating rule {:?} on {:?}", d, ss);
        'outer: for (i, rule) in d.0.iter().enumerate() {
            println!("trying rule {}-->{:?} from disjunction {:?}", i, rule, d.0);
            match rule {
                Terminal(c) => {
                    if ss.get(0) == Some(c) {
                        return Some(1);
                    }
                }
                NonTerminal(rule_refs) => {
                    let mut cur_ch: usize = 0;
                    for rule_ref in rule_refs {
                        let r = self.matches_helper(&ss[cur_ch..], &rules[*rule_ref..]);
                        match r {
                            Some(consumed) => cur_ch += consumed,
                            None => {
                                continue 'outer;
                            }
                        }
                    }

                    if cur_ch == ss.len() {
                        return Some(cur_ch);
                    }
                }
            }
        }
        None
    }

    fn matches(&self, st: &String) -> bool {
        self.matches_helper(&st.chars().collect::<Vec<char>>(), &self.rules[0..]) == Some(st.len())
    }
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
