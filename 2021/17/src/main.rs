use regex::Regex;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::hash::Hash;
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
    println!("{:?}", ta);

    // Part 1
    let mut velocities: HashSet<(i32, i32)> = HashSet::new();

    let mut outer_max_y = i32::MIN;
    let mut outer_max_v = None;

    for velocity_x in -1000..1000 {
        for velocity_y in -1000..1000 {
            //println!("initial {:?}", (velocity_x, velocity_y));

            let mut max_y = i32::MIN;
            let mut max_v = None;

            let mut position = (0, 0);
            let mut velocity = (velocity_x, velocity_y);
            for step in 0..1000 {
                position = (position.0 + velocity.0, position.1 + velocity.1);
                //println!("{:?}", position);

                if position.1 > max_y {
                    max_y = position.1;
                    max_v = Some((velocity_x, velocity_y))
                }

                if ta.contains(position) {
                    /*
                    println!(
                        "({}, {}) contains ({}, {}) on step {}",
                        velocity_x, velocity_y, position.0, position.1, step
                    );
                    */
                    if max_y >= outer_max_y {
                        println!("updating max to {}", max_y);
                        outer_max_y = max_y;
                        outer_max_v = max_v;
                    }

                    velocities.insert((velocity_x, velocity_y));
                    break;
                }

                velocity.0 += match velocity.0.cmp(&0) {
                    Ordering::Greater => -1,
                    Ordering::Less => 1,
                    Ordering::Equal => 0,
                };

                velocity.1 -= 1;
            }
        }
    }

    println!("{} {:?}", outer_max_y, outer_max_v);

    /*
    // Part 2
    let mut velocities: HashSet<(i32, i32)> = HashSet::new();
    for velocity_x in -100..100 {
        for velocity_y in -100..100 {
            // println!("initial v {:?}", (velocity_x, velocity_y));

            for t in 0..100 {
                let x_acceleration = if velocity_x > 0 {
                    -1
                } else if velocity_x >= 0 {
                    0
                } else {
                    1
                };
                let position = (
                    (x_acceleration * i32::pow(t, 2) / 2) + (velocity_x * t),
                    ((-1 * i32::pow(t, 2)) / 2) + (velocity_y * t),
                );

                //println!("position: {:?}", position);

                if ta.contains(position) {
                    println!(
                        "({}, {}) contains ({}, {}) on step {}",
                        velocity_x, velocity_y, position.0, position.1, t,
                    );
                    velocities.insert((velocity_x, velocity_y));
                }
            }
        }
    }
    println!("{}", velocities.len());
    */

    let expected = vec![
        (23, -10),
        (25, -7),
        (8, 0),
        (26, -10),
        (20, -8),
        (25, -6),
        (25, -10),
        (8, 1),
        (24, -10),
        (7, 5),
        (23, -5),
        (27, -10),
        (8, -2),
        (25, -9),
        (26, -6),
        (30, -6),
        (7, -1),
        (13, -2),
        (15, -4),
        (7, 8),
        (22, -8),
        (23, -8),
        (23, -6),
        (24, -8),
        (7, 2),
        (27, -8),
        (27, -5),
        (25, -5),
        (29, -8),
        (7, 7),
        (7, 3),
        (9, -2),
        (11, -3),
        (13, -4),
        (30, -8),
        (28, -10),
        (27, -9),
        (30, -9),
        (30, -5),
        (29, -6),
        (6, 8),
        (20, -10),
        (8, -1),
        (28, -8),
        (15, -2),
        (26, -7),
        (7, 6),
        (7, 0),
        (10, -2),
        (30, -7),
        (21, -8),
        (24, -7),
        (22, -6),
        (11, -2),
        (6, 7),
        (21, -9),
        (29, -9),
        (12, -2),
        (7, 1),
        (28, -6),
        (9, -1),
        (11, -1),
        (28, -5),
        (22, -7),
        (21, -7),
        (20, -5),
        (6, 4),
        (6, 2),
        (15, -3),
        (28, -9),
        (23, -9),
        (11, -4),
        (10, -1),
        (20, -9),
        (21, -10),
        (24, -9),
        (9, 0),
        (29, -10),
        (6, 1),
        (20, -7),
        (22, -5),
        (12, -3),
        (6, 0),
        (12, -4),
        (26, -5),
        (14, -2),
        (7, 9),
        (20, -6),
        (27, -7),
        (6, 3),
        (14, -4),
        (30, -10),
        (26, -8),
        (24, -6),
        (22, -10),
        (26, -9),
        (22, -9),
        (29, -7),
        (6, 6),
        (6, 9),
        (24, -5),
        (28, -7),
        (21, -6),
        (14, -3),
        (25, -8),
        (23, -7),
        (27, -6),
        (7, 4),
        (6, 5),
        (13, -3),
        (21, -5),
        (29, -5),
    ];

    for e in expected {
        if !velocities.contains(&e) {
            println!("missing {:?}", e);
        }
        //velocities.remove(&e);
    }
    println!("{:?}", velocities.len());
}
