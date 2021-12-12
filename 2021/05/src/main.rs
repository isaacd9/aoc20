use std::io::{self, BufRead};

#[derive(Debug)]
struct LineSegment {
    source: (usize, usize),
    dest: (usize, usize),
}

#[derive(Debug)]
struct Board(Vec<Vec<usize>>);

impl Board {
    fn plot(&mut self, s: &LineSegment) {
        println!("plotting: {:?}", s);
        let x_range = if s.dest.0 < s.source.0 {
            s.dest.0..=s.source.0
        } else {
            s.source.0..=s.dest.0
        };

        let y_range = if s.dest.1 < s.source.1 {
            s.dest.1..=s.source.1
        } else {
            s.source.1..=s.dest.1
        };

        for x in x_range {
            for y in y_range.clone() {
                println!("plotting: ({}, {})", x, y);
                self.0[x][y] += 1
            }
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

    let mut board = Board(vec![vec![0; 1000]; 1000]);

    for seg in line_segments
        .iter()
        .filter(|seg| seg.source.0 == seg.dest.0 || seg.source.1 == seg.dest.1)
    {
        board.plot(seg)
    }

    let mut sum = 0;
    for row in board.0 {
        //println!("{:?}", row);
        for col in row {
            if col > 1 {
                sum += 1
            }
        }
    }

    println!("{:?}", sum);
}
