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
    rules: HashMap<usize, Disjunction>,
}

mod test {
    use super::*;

    //#[test]
    //fn test_matches_helper() {
    //    use crate::Rule::*;

    //    let g = Grammar {
    //        rules: vec![Disjunction(vec![Terminal('a')])],
    //    };

    //    assert_eq!(g.matches(&"a".to_string()), true);
    //    assert_eq!(g.matches(&"b".to_string()), false);
    //    assert_eq!(g.matches(&"ab".to_string()), false);
    //    assert_eq!(g.matches(&"ba".to_string()), false);
    //    assert_eq!(g.matches(&"aab".to_string()), false);
    //    assert_eq!(g.matches(&"bba".to_string()), false);

    //    let g = Grammar {
    //        rules: vec![
    //            Disjunction(vec![NonTerminal(vec![1, 2])]),
    //            Disjunction(vec![Terminal('a')]),
    //            Disjunction(vec![Terminal('b')]),
    //        ],
    //    };

    //    assert_eq!(g.matches(&"a".to_string()), false);
    //    assert_eq!(g.matches(&"b".to_string()), false);
    //    assert_eq!(g.matches(&"ab".to_string()), true);

    //    let g = Grammar {
    //        rules: vec![
    //            Disjunction(vec![NonTerminal(vec![1]), NonTerminal(vec![2])]),
    //            Disjunction(vec![Terminal('a')]),
    //            Disjunction(vec![Terminal('b')]),
    //        ],
    //    };

    //    assert_eq!(g.matches(&"a".to_string()), true);
    //    assert_eq!(g.matches(&"b".to_string()), true);
    //    assert_eq!(g.matches(&"ab".to_string()), false);
    //}
}

impl Grammar {
    fn matches_helper(&self, ss: &[char], i: usize) -> usize {
        use crate::Rule::*;

        let d = &self.rules[&i];

        //println!("trying dj {:?} ({:?}) on {:?}", i, d, ss);
        'outer: for (_, rule) in d.0.iter().enumerate() {
            //println!("trying rule {}-->{:?} from disjunction {:?}", i, rule, d.0);
            match rule {
                Terminal(c) => {
                    if ss.get(0) == Some(c) {
                        return 1;
                    }
                }
                NonTerminal(rule_refs) => {
                    let mut cur_ch: usize = 0;
                    for rule_ref in rule_refs {
                        //println!("{:?}", (i, rule_ref));
                        match (i, rule_ref) {
                            _ => {
                                let consumed = self.matches_helper(&ss[cur_ch..], *rule_ref);
                                if consumed == 0 {
                                    // Try next disjunction
                                    continue 'outer;
                                }
                                cur_ch += consumed;
                            }
                        }
                    }

                    //println!("done with rule_refs {:?}. consumed {:?}", rule_refs, cur_ch);
                    return cur_ch;
                }
            }
        }
        0
    }

    fn matches(&self, st: &String) -> bool {
        let chars = &st.chars().collect::<Vec<char>>();
        let mut cur_ch = 0;

        let mut done_w_forty_two = false;

        let mut forty_two_consumptions = 0;
        let mut thirty_one_consumptions = 0;

        loop {
            let consumed_forty_two = self.matches_helper(&chars[cur_ch..], 42);
            let consumed_thirty_one = self.matches_helper(&chars[cur_ch..], 31);

            //println!("{} {}", consumed_forty_two, consumed_thirty_one);
            if consumed_thirty_one > 0 {
                done_w_forty_two = true;
            }

            cur_ch += consumed_thirty_one;
            cur_ch += consumed_forty_two;

            if done_w_forty_two {
                if consumed_thirty_one == 0 {
                    break;
                }
                thirty_one_consumptions += 1;
            } else {
                if consumed_forty_two == 0 {
                    break;
                }
                forty_two_consumptions += 1;
            }
        }

        cur_ch == st.len() && forty_two_consumptions >= 2 && thirty_one_consumptions >= 1
    }
}

fn read_input(lines: Lines<StdinLock>) -> (Grammar, Vec<String>) {
    let mut unwrapped_lines = lines.map(|li| li.unwrap());

    let dj = unwrapped_lines
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
        .collect::<HashMap<usize, Disjunction>>();

    let lines: Vec<String> = unwrapped_lines.collect();
    (Grammar { rules: dj }, lines)
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    let res = read_input(lines);

    let grammar = res.0;
    let list = res.1;
    //println!("{} -> {:?}", st, grammar.matches(&st));

    let mut su = 0;
    for st in &list {
        let m = grammar.matches(&st);
        println!("{} -> {:?}", st, m);
        if m {
            su += 1;
        }
    }
    println!("{} / {}", su, list.len());
}
