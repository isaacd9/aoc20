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
        fields
            .iter()
            .filter(|field| {
                let mut all_values_satisifed = true;
                for v in &values {
                    let mut any_rule_contains_value = false;

                    for r in &field.rules {
                        if r.contains(v) {
                            any_rule_contains_value = true;
                        }
                    }

                    all_values_satisifed &= any_rule_contains_value
                }
                all_values_satisifed
            })
            .map(|k| k.clone())
            .collect()
    }

    fn order_fields_helper(
        &self,
        tickets: &Vec<Ticket>,
        i: usize,
        used_field_names: &HashSet<String>,
        all_fields: &HashMap<usize, Vec<Field>>,
        mut memo_map: &mut HashMap<(usize, String), Option<Vec<Field>>>,
    ) -> Option<Vec<Field>> {
        let mut memo_key_v: Vec<String> = used_field_names.iter().cloned().collect();
        memo_key_v.sort();
        let memo_key: String = memo_key_v.join(" ");

        match memo_map.get(&(i, memo_key.clone())) {
            Some(k) => {
                //println!("hit memo case: {:?}", (i, &memo_key));
                return k.clone();
            }
            None => (),
        }

        let possible_fields: Vec<Field> = all_fields[&i]
            .iter()
            .filter(|field| !used_field_names.contains(&field.name))
            .cloned()
            .collect();

        let ret = if i == 0 {
            match possible_fields.len() {
                0 => None,
                _ => Some(vec![possible_fields[0].clone()]),
            }
        } else {
            for (field_i, field) in possible_fields.iter().enumerate() {
                if i == 19 {
                    println!(
                        "possible field {} for position {} (out of {})",
                        field_i,
                        i,
                        possible_fields.len()
                    );
                }
                let mut used_field_names = used_field_names.clone();
                used_field_names.insert(field.name.clone());

                match self.order_fields_helper(
                    tickets,
                    i - 1,
                    &used_field_names,
                    all_fields,
                    &mut memo_map,
                ) {
                    Some(rec) => {
                        let mut r = rec.clone();
                        r.push(field.clone());

                        let ret = Some(r);
                        memo_map.insert((i, memo_key), ret.clone());

                        return ret;
                    }
                    None => (),
                }
            }

            None
        };

        memo_map.insert((i, memo_key), ret.clone());

        ret
    }

    fn precompute_fields(
        &self,
        tickets: &Vec<Ticket>,
        fields: &Vec<Field>,
    ) -> HashMap<usize, Vec<Field>> {
        let mut m = HashMap::new();

        for i in 0..tickets[0].0.len() {
            let values: Vec<u64> = tickets.iter().map(|t| t.0[i]).collect();

            let fields_for_i = self.fields_matching_values(values, fields.clone());
            m.insert(i, fields_for_i);
        }

        m
    }

    fn order_fields(&self, t: &Vec<Ticket>) -> Vec<Field> {
        let mut fields_map = HashMap::new();
        for f in &self.0 {
            fields_map.insert(f.name.clone(), f.clone());
        }

        let precomputed = self.precompute_fields(t, &fields_map.values().cloned().collect());

        let r = self
            .order_fields_helper(
                t,
                t[0].0.len() - 1,
                &HashSet::new(),
                &precomputed,
                &mut HashMap::new(),
            )
            .unwrap();
        let k = r.clone();
        k
    }
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

        let possible_fields = f.fields_matching_values(
            vec![18, 5, 9],
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
            vec![99, 99, 99],
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

    let field_re = Regex::new(r"(?P<name>.*+): (?P<rule>.*+)").unwrap();
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
    let my_ticket = &res.1;
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

    let valid_tickets: Vec<Ticket> = other_tickets
        .iter()
        .filter(|t| fields.invalid_values(t).len() == 0)
        .cloned()
        .collect();

    println!(
        "{:?} out of {:?} are valid",
        valid_tickets.len(),
        other_tickets.len()
    );

    let order = fields.order_fields(&valid_tickets);
    println!(
        "{:?}",
        order
            .iter()
            .map(|f| f.name.clone())
            .collect::<Vec<String>>()
    );

    let mut mult = 1;
    for (i, val) in my_ticket.0.iter().enumerate() {
        let field = &order[i as usize];
        println!("{} -> {}", field.name, val);
        if field.name.contains("departure") {
            println!("multing {} {}", field.name, val);
            mult *= val;
        }
    }
    println!("{}", mult)
}
