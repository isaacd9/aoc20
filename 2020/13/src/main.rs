use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead, Error, Lines, StdinLock};
use std::time::SystemTime;

#[derive(Debug, PartialEq, Clone)]
struct Schedule {
    timestamp: u64,
    busses: Vec<Option<u64>>,
}

fn read_input(lines: Lines<StdinLock>) -> Result<Schedule, io::Error> {
    let mut lines_iter = lines.map(|li| li.unwrap());

    let timestamp = lines_iter.next().unwrap();
    let busses = lines_iter
        .next()
        .unwrap()
        .split(",")
        .map(|bus_no| {
            let parsed = bus_no.parse().ok();

            parsed
        })
        .collect();

    let s = Schedule {
        timestamp: timestamp.parse().unwrap(),
        busses: busses,
    };
    Ok(s)
}

impl Schedule {
    fn find_start_time_and_bus(&self) -> (u64, u64) {
        let mut start_time = self.timestamp;
        let mut found_time: Option<u64> = None;
        while found_time.is_none() {
            for u in &self.busses {
                match u {
                    Some(t) => {
                        if start_time % t == 0 {
                            found_time = Some(*t);
                        }
                    }
                    None => (),
                }
            }
            start_time += 1;
        }
        start_time -= 1;
        return (found_time.unwrap(), start_time - self.timestamp);
    }

    fn find_earliest_timestamp_by_id(&self) -> u128 {
        let equations: Vec<(u128, u128)> = self
            .busses
            .iter()
            .enumerate()
            .filter_map(|(i, bus_num)| {
                bus_num.and_then(|n| Some((n as u128, self.busses.len() as u128 - i as u128 - 1)))
            })
            .collect();

        let product: u128 = equations.iter().map(|eq| eq.0).product();

        let mut s: u128 = 0;
        for eq in equations.iter() {
            let m = product / eq.0;
            let coef = m % eq.0;

            let mut x = 0;
            for i in 0.. {
                if ((coef * i) % eq.0) == 1 {
                    x = i;
                    break;
                };
            }
            let term = eq.1 * m * x;
            s += term;
        }
        s % product - self.busses.len() as u128 + 1
    }
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    let sched = read_input(lines).unwrap();

    let time_and_bus = sched.find_start_time_and_bus();
    println!(
        "{}*{} = {}",
        time_and_bus.0,
        time_and_bus.1,
        time_and_bus.0 * time_and_bus.1
    );
    println!("{}", sched.find_earliest_timestamp_by_id());
}
