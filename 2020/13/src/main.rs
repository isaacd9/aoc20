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
    //

    fn find_earliest_timestamp_by_id(&self, start_mult: u128) -> u128 {
        let equations: Vec<(u128, u128)> = self
            .busses
            .iter()
            .enumerate()
            .filter_map(|(i, bus_num)| bus_num.and_then(|n| Some((n as u128, i as u128))))
            .collect();

        let max = &equations.iter().fold(
            (0, 0),
            |acc, (n, i)| if *n > acc.0 { (*n, *i) } else { acc },
        );

        let start = if start_mult < max.1 {
            max.0
        } else {
            start_mult / max.0
        };

        //println("");
        for t in (start..) {
            let mult = max.0 * t;
            let tt = mult.checked_sub(max.1).unwrap();
            //println!("{}, {}", mult, tt);
            let mut all_found = true;
            for eq in &equations {
                all_found &= (tt + eq.1) % eq.0 == 0;
            }
            if all_found {
                return tt;
            }

            if t % 100000000 == 0 {
                println!("checkpoint: {}", t)
            }
        }
        //while !found {
        //    let mut inner_found = true;
        //    for (i, u) in self.busses.iter().enumerate() {
        //        u.and_then(|v| {
        //            let this_mod = v.checked_sub((i + 1) as u64);
        //            if this_mod.is_none() {
        //                return None;
        //            }
        //            if t % v as u128 != this_mod.unwrap() as u128 {
        //                inner_found = false;
        //                Some(())
        //            } else {
        //                None
        //            }
        //        });
        //    }
        //    found = inner_found;
        //    t += 1
        //}
        0
    }
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    let sched = read_input(lines).unwrap();

    println!("{:?}", sched.busses);
    let t = sched.find_earliest_timestamp_by_id(100000000000000);
    println!("{}", t);

    //println!("{} @ {}", times.0, times.1);
    //println!(
    //    "{} ",
    //    times.1.checked_sub(sched.timestamp).unwrap() * times.0,
    //);
}
