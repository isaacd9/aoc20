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

#[allow(dead_code)]
impl Fields {
    fn invalid_values(&self, t: &Ticket) -> Vec<u64> {
        t.0.iter()
            .filter(|value| {
                for f in &self.0 {
                    for r in &f.rules {
                        if r.contains(value) {
                            return false;
                        }
                    }
                }
                return true;
            })
            .map(|k| *k)
            .collect()
    }

    fn fields_matching_values(&self, values: Vec<u64>, fields: Vec<Field>) -> Vec<Field> {
        let mut possible_fields = vec![];
        for field in fields.iter() {
            let mut all_matched = true;
            for val in &values {
                let mut some_matched = false;
                for rule in &field.rules {
                    if rule.contains(&val) {
                        some_matched = true;
                    }
                }

                all_matched &= some_matched;
            }

            if all_matched {
                possible_fields.push(field.clone());
            }
        }
        return possible_fields;
    }

    fn order_fields_helper(
        &self,
        tickets: &Vec<Ticket>,
        i: usize,
        unusued_fields: Vec<Field>,
    ) -> Option<Vec<Field>> {
        let values = tickets.iter().map(|t| t.0[i]).collect();

        if unusued_fields.len() == 1 {
            self.fields_matching_values(values, unusued_fields);
            None
        } else {
            let possible_fields = self.fields_matching_values(values, unusued_fields);
            match possible_fields.len() {
                0 => None,
                _ => None,
            }
        }
    }

    //fn order_fields(&self, t: &Vec<Ticket>) -> Vec<Field> {}
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    #[test]
    fn test_map_values_to_field() {
        let f = Fields(vec![]);

        // Base case
        let possible_fields = f.fields_matching_values(
            vec![3, 15, 5],
            vec![
                Field {
                    name: "row".to_string(),
                    rules: vec![(0..=5), (8..=19)],
                },
                Field {
                    name: "class".to_string(),
                    rules: vec![(0..=1), (4..=19)],
                },
                Field {
                    name: "seat".to_string(),
                    rules: vec![(0..=13), (16..=19)],
                },
            ],
        );
        println!("{:?}", possible_fields);

        let possible_fields = f.fields_matching_values(
            vec![9, 1, 14],
            vec![
                Field {
                    name: "row".to_string(),
                    rules: vec![(0..=5), (8..=19)],
                },
                Field {
                    name: "class".to_string(),
                    rules: vec![(0..=1), (4..=19)],
                },
                Field {
                    name: "seat".to_string(),
                    rules: vec![(0..=13), (16..=19)],
                },
            ],
        );
        println!("{:?}", possible_fields);
    }
}

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
    let fields = &res.0;
    let other_tickets = &res.2;
    let mut sum = 0;

    {
        for ticket in other_tickets {
            let iv = fields.invalid_values(&ticket);
            for v in &iv {
                sum += v
            }
        }
        println!("{:?}", sum);
    }

    let valid_tickets: Vec<&Ticket> = other_tickets
        .iter()
        .filter(|t| fields.invalid_values(t).len() == 0)
        .collect();

    println!("{:?}", valid_tickets);
}
