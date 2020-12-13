use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead, Error, Lines, StdinLock};

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
    //fn find_start_time_and_bus(&self) -> (u64, u64) {
    //    let mut start_time = self.timestamp;
    //    let mut found_time: Option<u64> = None;
    //    while found_time.is_none() {
    //        for u in &self.busses {
    //            if start_time % u == 0 {
    //                found_time = Some(*u);
    //            }
    //        }
    //        start_time += 1;
    //    }
    //    start_time -= 1;
    //    return (found_time.unwrap(), start_time);
    //}

    fn find_earliest_timestamp_by_id(&self) -> u128 {
        let mut t: u128 = 0;
        let mut found = false;
        while !found {
            let mut inner_found = true;
            for (i, u) in self.busses.iter().enumerate() {
                u.and_then(|v| {
                    let this_mod = v.checked_sub((i + 1) as u64);
                    if this_mod.is_none() {
                        return None;
                    }
                    if t % v as u128 != this_mod.unwrap() as u128 {
                        inner_found = false;
                        Some(())
                    } else {
                        None
                    }
                });
            }
            found = inner_found;
            t += 1
        }
        t
    }
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    let sched = read_input(lines).unwrap();

    println!("{:?}", sched.busses);
    let t = sched.find_earliest_timestamp_by_id();
    println!("{}", t);

    //println!("{} @ {}", times.0, times.1);
    //println!(
    //    "{} ",
    //    times.1.checked_sub(sched.timestamp).unwrap() * times.0,
    //);
}
