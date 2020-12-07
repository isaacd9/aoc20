use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead, Error, Lines, StdinLock};

use std::cmp;

#[derive(Default, Debug)]
struct Bags {
    bag_types: HashMap<String, Vec<Rule>>,
}

#[derive(Default, Debug)]
struct Bag {
    name: String,
    contents: Vec<(u32, Bag)>,
}

impl Bag {
    fn find(&self, name: &String) -> bool {
        if self.name.eq(name) {
            return true;
        }

        for (_, bag) in &self.contents {
            if bag.find(name) {
                return true;
            }
        }
        false
    }

    fn count_bags_inside(&self) -> u32 {
        let mut count = 0;
        for (c, bag) in &self.contents {
            let inside = bag.count_bags_inside();
            count += c * inside;
            count += c;
        }
        count
    }
}

impl Bags {
    fn new(rules: &mut dyn Iterator<Item = &BagRules>) -> Bags {
        let mut bags = Bags {
            bag_types: HashMap::new(),
        };

        for ru in rules {
            bags.bag_types.insert(ru.bag.clone(), ru.contents.clone());
        }

        bags
    }

    fn resolve(&self, bag_name: &String) -> Bag {
        let rules = self.bag_types.get(bag_name).unwrap();
        let contents = rules
            .iter()
            .map(|rule| (rule.0, self.resolve(&rule.1)))
            .collect();

        Bag {
            name: bag_name.clone(),
            contents: contents,
        }
    }
}

#[derive(Default, Debug, Clone)]
struct Rule(u32, String);

#[derive(Default, Debug)]
struct BagRules {
    bag: String,
    contents: Vec<Rule>,
}

fn parse_bags(lines: Lines<StdinLock>) -> Result<Vec<BagRules>, io::Error> {
    let line_re = Regex::new(r"(?P<bag_type>.*) bags contain (?P<rules>.*)").unwrap();
    let rule_re = Regex::new(r"(?P<num>\d) (?P<type>.*) bag").unwrap();
    let no_regex = Regex::new(r"no other bags").unwrap();

    let rules = lines
        .map(|li| li.unwrap())
        .map(|li| {
            let caps = line_re.captures(li.as_str()).unwrap();
            let rules = caps["rules"].split(",");

            let r = rules
                .map(|rule| {
                    let caps = rule_re.captures(rule);
                    match caps {
                        Some(caps) => {
                            Some(Rule(caps["num"].parse().unwrap(), caps["type"].to_string()))
                        }
                        None => None,
                    }
                })
                .filter(|r| r.is_some())
                .map(|r| r.unwrap())
                .collect();

            BagRules {
                bag: caps["bag_type"].to_string(),
                contents: r,
            }
        })
        .collect();

    Ok(rules)
}

fn main() {
    let looking_for = "shiny gold".to_string();

    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    let bag_rules = parse_bags(lines).unwrap();
    let bags = Bags::new(&mut bag_rules.iter());
    let mut count = 0;
    for rule in bag_rules {
        if looking_for == rule.bag {
            continue;
        }

        let resolved = bags.resolve(&rule.bag);
        if resolved.find(&looking_for) {
            count += 1;
        }
    }
    println!("{}", count);
    println!(
        "{}",
        bags.resolve(&"shiny gold".to_string()).count_bags_inside()
    );
}
