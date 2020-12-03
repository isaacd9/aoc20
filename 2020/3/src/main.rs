use std::io::{self, BufRead, Error, Lines};
use std::iter::Iterator;

fn num_trees(
    lines: &mut dyn Iterator<Item = &Result<String, Error>>,
    slope: (usize, usize),
) -> u32 {
    let mut x = 0;
    let mut y = 0;
    let mut trees = 0;
    for line in lines.step_by(slope.1) {
        let li = line.as_ref().unwrap();
        let c = li.chars().nth(x % li.len());
        if c == Some('#') {
            trees += 1;
        }
        x += slope.0;
        y += slope.1;
        //println!("({},{})", x, y);
    }
    trees
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    let collected: Vec<Result<String, Error>> = lines.collect();
    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let mut total = 1;
    for slope in slopes {
        let num = num_trees(&mut collected.iter(), slope);
        println!("{}", num);
        total *= num;
    }
    println!("{}", total)
}
