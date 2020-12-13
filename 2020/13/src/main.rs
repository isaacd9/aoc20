use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead, Error, Lines, StdinLock};

#[derive(Debug, PartialEq, Clone)]
struct Schedule {
    timestamp: u64,
    busses: Vec<u64>,
}

fn read_input(lines: Lines<StdinLock>) -> Result<Schedule, io::Error> {
    let mut lines_iter = lines.map(|li| li.unwrap());

    let timestamp = lines_iter.next().unwrap();
    let busses = lines_iter
        .next()
        .unwrap()
        .split(",")
        .filter_map(|bus_no| {
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

fn find_start_time_and_bus(sched: &Schedule) -> (u64, u64) {
    let mut start_time = sched.timestamp;
    let mut found_time: Option<u64> = None;
    while found_time.is_none() {
        for u in &sched.busses {
            if start_time % u == 0 {
                found_time = Some(*u);
            }
        }
        start_time += 1;
    }
    start_time -= 1;
    return (found_time.unwrap(), start_time);
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    let sched = read_input(lines).unwrap();

    let times = find_start_time_and_bus(&sched);

    println!("{} @ {}", times.0, times.1);
    println!(
        "{} ",
        times.1.checked_sub(sched.timestamp).unwrap() * times.0,
    );
}
