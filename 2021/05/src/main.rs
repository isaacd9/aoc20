use std::io::{self, BufRead};
use std::num;

#[derive(Debug)]
struct LineSegment {
    source: (usize, usize),
    dest: (usize, usize),
}

#[derive(Debug)]
struct Board(Vec<Vec<usize>>);

fn normalize(n: f64) -> i32 {
    if n < 0.0 {
        return -1;
    } else if n > 0.0 {
        return 1;
    } else if n == 0.0 {
        return 0;
    };
    panic!("impossible")
}

impl Board {
    fn plot(&mut self, s: &LineSegment) {
        //println!("plotting: {:?}", s);

        let a = s.dest.0 as i32 - s.source.0 as i32;
        let b = s.dest.1 as i32 - s.source.1 as i32;

        let magnitude = f64::abs(f64::sqrt(i32::pow(a, 2) as f64 + i32::pow(b, 2) as f64));
        //println!("{}", magnitude);
        let normal_a = normalize(a as f64 / magnitude);
        let normal_b = normalize(b as f64 / magnitude);

        //println!("{} {}", normal_a, normal_b);

        let mut pt = s.source;
        self.0[s.source.0][s.source.1] += 1;
        while pt != s.dest {
            pt = (
                (pt.0 as i32 + normal_a) as usize,
                (pt.1 as i32 + normal_b) as usize,
            );
            self.0[pt.0][pt.1] += 1;
            //println!("{:?}", pt);
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|line| line.unwrap());

    let line_segments: Vec<LineSegment> = lines
        .map(|line| {
            let mut points = line.split("->").map(|point| {
                let mut coords = point
                    .split(",")
                    .map(|coord| coord.trim().parse::<usize>().unwrap());

                (coords.next().unwrap(), coords.next().unwrap())
            });

            LineSegment {
                source: points.next().unwrap(),
                dest: points.next().unwrap(),
            }
        })
        .collect();

    let mut board_one = Board(vec![vec![0; 1000]; 1000]);

    for seg in line_segments
        .iter()
        .filter(|seg| seg.source.0 == seg.dest.0 || seg.source.1 == seg.dest.1)
    {
        board_one.plot(seg)
    }

    // Part 1
    let mut one_sum = 0;
    for row in board_one.0 {
        //println!("{:?}", row);
        for col in row {
            if col > 1 {
                one_sum += 1
            }
        }
    }

    println!("one_sum: {:?}", one_sum);

    // Part 2
    let mut board_two = Board(vec![vec![0; 1000]; 1000]);
    for seg in line_segments.iter().filter(|seg| {
        //println!(
        //    "{:?} {} {} {}",
        //    seg,
        //    i32::abs(seg.dest.0 as i32 - seg.source.0 as i32),
        //    i32::abs(seg.dest.1 as i32 - seg.source.1 as i32),
        //    i32::abs(seg.dest.0 as i32 - seg.source.0 as i32)
        //        == i32::abs(seg.dest.1 as i32 - seg.source.1 as i32)
        //);

        seg.source.0 == seg.dest.0
            || seg.source.1 == seg.dest.1
            || i32::abs(seg.dest.0 as i32 - seg.source.0 as i32)
                == i32::abs(seg.dest.1 as i32 - seg.source.1 as i32)
    }) {
        board_two.plot(seg)
    }

    let mut two_sum = 0;
    for row in board_two.0 {
        //println!("{:?}", row);
        for col in row {
            if col > 1 {
                two_sum += 1
            }
        }
    }

    println!("two_sum: {:?}", two_sum);
}
