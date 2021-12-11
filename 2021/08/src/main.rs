use std::collections::{HashMap, HashSet};
use std::fs::OpenOptions;
use std::io::{self, BufRead};

fn missing(st: &String) -> Option<char> {
    for ch in ['a', 'b', 'c', 'd', 'e', 'f', 'g'] {
        if !st.contains(ch) {
            return Some(ch);
        }
    }
    None
}

fn contains(st: &String, sub: &String) -> bool {
    let st_map: HashSet<char> = st.chars().collect();
    let sub_map: HashSet<char> = sub.chars().collect();

    sub_map.is_subset(&st_map)
}

fn find_digits(key: &[String], digits: &[String]) -> Vec<Option<u32>> {
    //digits
    //    .iter()
    //    .map(|d| match d.len() {
    //        2 => Some(1),
    //        3 => Some(7),
    //        4 => Some(4),
    //        7 => Some(8),
    //        _ => None,
    //    })
    //    .collect()

    let mut m: HashMap<u32, String> = HashMap::new();
    for signal_pattern in key {
        let v = match signal_pattern.len() {
            2 => Some(1),
            3 => Some(7),
            4 => Some(4),
            7 => Some(8),
            _ => None,
        };

        if v.is_some() {
            m.insert(v.unwrap(), signal_pattern.to_string());
        }
    }

    // 6 digit
    let six = key
        .iter()
        .filter(|k| k.len() == 6 && !contains(k, &m[&1]))
        .nth(0);
    m.insert(6, six.unwrap().clone());

    let zero = key
        .iter()
        .filter(|k| {
            k.len() == 6 && *k != six.unwrap() && contains(&m[&4], &missing(k).unwrap().to_string())
        })
        .nth(0);
    m.insert(0, zero.unwrap().clone());

    let nine = key
        .iter()
        .filter(|k| k.len() == 6 && *k != six.unwrap() && *k != zero.unwrap())
        .nth(0);
    m.insert(9, nine.unwrap().clone());

    // 5 digit
    let three = key
        .iter()
        .filter(|k| k.len() == 5 && contains(k, &m[&1]))
        .nth(0);
    m.insert(3, three.unwrap().clone());

    let five = key
        .iter()
        .filter(|k| {
            let mut four_without_one = m[&4].clone();
            for c in m[&1].chars() {
                four_without_one = four_without_one.replace(&c.to_string(), "");
            }
            //println!("{:?}", k);
            k.len() == 5 && contains(k, &four_without_one)
        })
        .nth(0);
    m.insert(5, five.unwrap().clone());

    let two = key
        .iter()
        .filter(|k| k.len() == 5 && *k != three.unwrap() && *k != five.unwrap())
        .nth(0);
    m.insert(2, two.unwrap().clone());

    println!("{:?}", m);
    let mut str_to_n: HashMap<String, u32> = HashMap::new();
    for (k, v) in m {
        str_to_n.insert(v, k);
    }

    println!("{:?}", str_to_n);
    digits
        .iter()
        .map(|d| {
            println!("{:?}", d);
            Some(str_to_n[d])
        })
        .collect()
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(|line| line.unwrap());
    let input: Vec<Vec<Vec<String>>> = lines
        .map(|line| {
            line.split("|")
                .map(|half| {
                    half.trim()
                        .split(" ")
                        .map(|st| {
                            let mut chrs: Vec<char> = st.to_string().chars().collect();
                            chrs.sort();
                            chrs.iter().collect()
                        })
                        .collect()
                })
                .collect()
        })
        .collect();

    for line in &input {
        //println!("{:?}", line);
    }
    let digits: Vec<Vec<Option<u32>>> = input
        .iter()
        .map(|line| find_digits(&line[0], &line[1]))
        .collect();

    println!("{:?}", digits);
    // Part 1
    //let sum: usize = digits
    //    .iter()
    //    .map(|line| line.iter().filter(|o| o.is_some()).count())
    //    .sum();
    //println!("{:?}", sum)

    // Part 2
    let sum: usize = digits
        .iter()
        .map(|digits| digits.iter().fold(0, |acc, elem| acc * 10 + elem.unwrap()))
        .sum::<u32>()
        .try_into()
        .unwrap();
    println!("{:?}", sum)
}
