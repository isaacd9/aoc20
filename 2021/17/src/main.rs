use regex::Regex;
use std::result::Result::Err;

use std::{
    error,
    io::{self, Read},
};

#[derive(Debug)]
struct TargetArea {
    x: (i32, i32),
    y: (i32, i32),
}

impl TargetArea {
    fn parse(st: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let re = Regex::new(r"target area: x=([0-9\-]+)..([0-9\-]+), y=([0-9\-]+)..([0-9\-]+)")?;
        let caps = re.captures(&st);
        match caps {
            Some(c) => Ok(TargetArea {
                x: (c[1].parse()?, c[2].parse()?),
                y: (c[3].parse()?, c[4].parse()?),
            }),
            None => Result::Err("oh no".to_string().into()),
        }
    }

    fn contains(&self, pos: (i32, i32)) -> bool {
        (pos.0 >= self.x.0) && (pos.0 <= self.x.1) && (pos.1 >= self.y.0) && (pos.1 <= self.y.1)
    }
}

fn main() {
    let stdin = io::stdin();

    let mut st = String::new();
    stdin.lock().read_to_string(&mut st).unwrap();

    let ta = TargetArea::parse(&st.as_str()).unwrap();

    let mut outer_max_y = i32::MIN;
    let mut outer_max_v = None;

    for velocity_x in -200..200 {
        for velocity_y in -200..200 {
            let mut max_y = i32::MIN;
            let mut max_v = None;

            let mut position = (0, 0);
            let mut velocity = (velocity_x, velocity_y);
            for step in 0..100 {
                position = (position.0 + velocity.0, position.1 + velocity.1);

                if position.1 > max_y {
                    max_y = position.1;
                    max_v = Some((velocity_x, velocity_y))
                }

                if ta.contains(position) {
                    println!(
                        "({}, {}) contains ({}, {}) on step {}",
                        velocity_x, velocity_y, position.0, position.1, step
                    );

                    if max_y > outer_max_y {
                        outer_max_y = max_y;
                        outer_max_v = max_v;
                    }
                    break;
                }

                if velocity.0 > 0 {
                    velocity.0 -= 1;
                } else if velocity.1 < 0 {
                    velocity.0 += 1;
                }

                velocity.1 -= 1;
            }
        }
    }

    println!("{} {:?}", outer_max_y, outer_max_v);
}
