use core::fmt;
use std::io::{self, BufRead};

#[derive(Debug, Clone)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn parse(st: &str) -> Self {
        let mut sp = st.split(',');

        Coord {
            x: sp.next().unwrap().parse().unwrap(),
            y: sp.next().unwrap().parse().unwrap(),
        }
    }
}

#[derive(Debug)]
enum Fold {
    X(usize),
    Y(usize),
}

impl Fold {
    fn parse(st: &str) -> Self {
        let trimmed = st.trim_start_matches("fold along ");
        let mut sp = trimmed.split('=');
        let axis = sp.next().unwrap();

        match axis {
            "x" => Fold::X(sp.next().unwrap().parse().unwrap()),
            "y" => Fold::Y(sp.next().unwrap().parse().unwrap()),
            _ => panic!("unexpected split: {}", axis),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Dot {
    On,
    Off,
}

impl fmt::Display for Dot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Dot::*;
        match self {
            On => write!(f, "{}", "#"),
            Off => write!(f, "{}", "."),
        }
    }
}

#[derive(Debug, Clone)]
struct Board(Vec<Vec<Dot>>);

impl Board {}

impl fmt::Display for Board {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut y = 0;
        for line in &self.0 {
            if y < 10 {
                let mut x = 0;
                for dot in line {
                    if x < 50 {
                        write!(f, "{}", dot)?;
                    }
                    x += 1;
                }
                write!(f, "\n")?;
                y += 1;
            }
        }
        Ok(())
    }
}

impl Board {
    fn plot(&mut self, c: &Coord) {
        self.0[c.y][c.x] = Dot::On;
    }

    fn visible(&self) -> u32 {
        let mut v = 0;
        for row in &self.0 {
            for col in row {
                v += match col {
                    Dot::Off => 0,
                    Dot::On => 1,
                }
            }
        }
        v
    }
}

fn fold(coords: &[Coord], fold: &Fold) -> Vec<Coord> {
    let mut r = vec![];

    for coord in coords {
        match fold {
            Fold::Y(u) => {
                if coord.y > *u {
                    let new_coord = Coord {
                        x: coord.x,
                        y: u - (coord.y - u),
                    };
                    r.push(new_coord)
                } else {
                    r.push(coord.clone())
                }
            }
            Fold::X(u) => {
                if coord.x > *u {
                    let new_coord = Coord {
                        x: u - (coord.x - u),
                        y: coord.y,
                    };
                    r.push(new_coord)
                } else {
                    r.push(coord.clone())
                }
            }
        }
    }

    r
}

fn main() {
    use Dot::*;

    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(|line| line.unwrap());

    let mut folds = vec![];
    let mut dots = vec![];
    for line in lines {
        if line.is_empty() {
            continue;
        } else if line.starts_with("fold along") {
            folds.push(Fold::parse(&line));
        } else {
            dots.push(Coord::parse(&line));
        }
    }

    // Original
    let mut original_board: Board = Board(vec![vec![Dot::Off; 2000]; 2000]);
    for dot in &dots {
        original_board.plot(dot)
    }
    //println!("{}", original_board);

    let mut first_fold_board: Board = Board(vec![vec![Dot::Off; 2000]; 2000]);
    let folded = fold(&dots, &folds[0]);
    for dot in folded {
        first_fold_board.plot(&dot)
    }
    //println!("{}", first_fold_board);

    // Part 1
    println!("{}", first_fold_board.visible());

    // Part 2
    let mut board: Board = Board(vec![vec![Dot::Off; 2000]; 2000]);
    let mut folded = dots.clone();

    for f in folds {
        let mut new_board: Board = Board(vec![vec![Dot::Off; 2000]; 2000]);
        folded = fold(&folded, &f);
        for dot in &folded {
            new_board.plot(dot)
        }
        board = new_board
    }

    println!("{}", board);
}
