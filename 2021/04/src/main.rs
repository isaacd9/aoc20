use std::collections::HashSet;
use std::fmt;
use std::io::{self, BufRead};

#[derive(Clone)]
struct Board {
    nums: Vec<Vec<u32>>,
    marked: HashSet<(usize, usize)>,
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for line in self.nums.iter() {
            let joined = line
                .iter()
                .map(|i| i.to_string())
                .collect::<Vec<String>>()
                .join("\t");

            writeln!(f, "{}", joined)?;
        }
        Ok(())
    }
}

impl Board {
    fn mark(&mut self, n: u32) {
        for (row, row_vec) in self.nums.iter().enumerate() {
            for (col, num) in row_vec.iter().enumerate() {
                if *num == n {
                    self.marked.insert((row, col));
                }
            }
        }
    }

    fn check_win(&self) -> bool {
        let num_rows = self.nums.len();
        let num_cols = self.nums[0].len();

        for r in 0..num_rows {
            let mut complete = true;
            for c in 0..num_cols {
                complete &= self.marked.contains(&(r, c));
            }

            if complete {
                //println!("winning row {}", r);
                return true;
            }
        }

        for c in 0..num_cols {
            let mut complete = true;
            for r in 0..num_rows {
                complete &= self.marked.contains(&(r, c));
            }

            if complete {
                //println!("winning col {}", c);
                return true;
            }
        }

        false
    }

    fn sum_unmarked(&self) -> u32 {
        let mut sum = 0;
        for (row, row_vec) in self.nums.iter().enumerate() {
            for (col, num) in row_vec.iter().enumerate() {
                if !self.marked.contains(&(row, col)) {
                    sum += num
                }
            }
        }
        sum
    }
}

fn find_first_winner(numbers: &Vec<u32>, boards: &mut Vec<Board>) -> Option<(u32, usize)> {
    for marked in numbers {
        println!("m: {}", marked);
        for (board_n, board) in boards.iter_mut().enumerate() {
            board.mark(*marked);
            if board.check_win() {
                return Some((*marked, board_n));
            }
        }
    }

    None
}

fn find_last_winner(numbers: &Vec<u32>, boards: &mut Vec<Board>) -> Option<(u32, usize)> {
    let len = boards.len();
    let mut won_boards: HashSet<usize> = HashSet::new();
    for marked in numbers {
        println!("m: {}", marked);
        for (board_n, board) in boards.iter_mut().enumerate() {
            board.mark(*marked);
            if board.check_win() {
                //println!("board {} won!", board_n);
                won_boards.insert(board_n);
            }

            if won_boards.len() == len {
                return Some((*marked, board_n));
            }
        }
    }

    None
}

fn main() -> Result<(), io::Error> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|line| line.unwrap());

    let numbers: Vec<u32> = lines
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();

    let mut boards: Vec<Board> = vec![];
    let mut cur_board: Board = Board {
        nums: vec![],
        marked: HashSet::default(),
    };

    for line in lines {
        if line == "" {
            if cur_board.nums.len() > 0 {
                boards.push(cur_board);
                cur_board = Board {
                    nums: vec![],
                    marked: HashSet::default(),
                };
            }
        } else {
            let int_line: Vec<u32> = line
                .split_ascii_whitespace()
                .map(|c| c.parse().unwrap())
                .collect();
            cur_board.nums.push(int_line);
        }
    }

    // Part 1
    let mut boards_one = boards.clone();
    let (m, w) = find_first_winner(&numbers, &mut boards_one).unwrap();
    println!(
        "Winner on {} was board {}. Sum unmarked: {}. Solution {}",
        m,
        w,
        boards_one[w].sum_unmarked(),
        m * boards_one[w].sum_unmarked(),
    );

    // Part 2
    let mut boards_two = boards.clone();
    let (m, w) = find_last_winner(&numbers, &mut boards_two).unwrap();
    println!(
        "Winner on {} was board {}. Sum unmarked: {}. Solution {}",
        m,
        w,
        boards_two[w].sum_unmarked(),
        m * boards_two[w].sum_unmarked(),
    );

    Ok(())
}
