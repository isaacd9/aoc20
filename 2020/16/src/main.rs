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

    fn map_values_to_field(
        &self,
        mut values: Vec<u64>,
        unusued_fields: Vec<Field>,
    ) -> Option<Vec<Vec<Field>>> {
        if unusued_fields.len() == 1 {
            let val = values.remove(0);

            for (i, field) in unusued_fields.iter().enumerate() {
                for rule in &field.rules {
                    if rule.contains(&val) {
                        return Some(vec![vec![field.clone()]]);
                    }
                }
            }

            return None;
        } else {
            let mut possible_orderings = vec![];

            let mut can_find_ordering = false;

            let val = values.remove(0);

            for (i, field) in unusued_fields.iter().enumerate() {
                for rule in &field.rules {
                    if rule.contains(&val) {
                        can_find_ordering = true;

                        let mut remaining_fields = unusued_fields.clone();
                        remaining_fields.remove(i);

                        let r = self.map_values_to_field(values.clone(), remaining_fields);
                        match r {
                            Some(orderings) => {
                                for order in orderings {
                                    let mut ret = vec![field.clone()];
                                    ret.extend(order);
                                    possible_orderings.push(ret);
                                }
                            }
                            None => return None,
                        }
                    }
                }
            }
            if !can_find_ordering {
                println!("couldn't find place to put {:?}", val);
                return None;
            }
            return Some(possible_orderings);
        }
    }

    fn order_fields_helper(
        &self,
        tickets: &Vec<Ticket>,
        i: usize,
        unusued_fields: Vec<Field>,
    ) -> Option<Vec<Vec<Field>>> {
        let values = tickets.iter().map(|t| t.0[i]).collect();

        if unusued_fields.len() == 1 {
            self.map_values_to_field(values, unusued_fields)
        } else {
            match self.map_values_to_field(values, unusued_fields) {
                Some(fields) => None,
                None => None,
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
        let orderings = f.map_values_to_field(
            vec![3, 15, 5],
            vec![
                Field {
                    name: "row".to_string(),
                    rules: vec![(0..=3)],
                },
                Field {
                    name: "col".to_string(),
                    rules: vec![(4..=5)],
                },
                Field {
                    name: "whatever".to_string(),
                    rules: vec![(10..=15)],
                },
            ],
        );
        println!("{:?}", orderings);

        let orderings = f.map_values_to_field(
            vec![4, 4, 4],
            vec![
                Field {
                    name: "1".to_string(),
                    rules: vec![(4..=5)],
                },
                Field {
                    name: "2".to_string(),
                    rules: vec![(4..=5)],
                },
                Field {
                    name: "3".to_string(),
                    rules: vec![(4..=5)],
                },
            ],
        );
        println!("{:?}", orderings.len());
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
