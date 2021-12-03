use std::io::{self, BufRead};

#[derive(Debug)]
enum Action {
    Forward(i32),
    Up(i32),
    Down(i32),
}

fn main() {
    use Action::*;
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(|line| line.unwrap());

    let actions: Vec<Action> = lines
        .map(|line| {
            let mut sp = line.split(" ");
            let cmd = sp.next().unwrap();
            let val = sp.next().unwrap().parse().unwrap();

            match cmd {
                "forward" => Forward(val),
                "up" => Up(val),
                "down" => Down(val),
                _ => panic!("unexpected instruction: {}", cmd),
            }
        })
        .collect();

    // Part 1
    let mut part_1_coords: Vec<i32> = vec![0, 0];
    for action in &actions {
        match action {
            Forward(x) => part_1_coords[0] += x,
            Up(y) => part_1_coords[1] -= y,
            Down(y) => part_1_coords[1] += y,
        }
    }
    println!(
        "{}*{}={}",
        part_1_coords[0],
        part_1_coords[1],
        part_1_coords[0] * part_1_coords[1]
    );

    // Part 2
    let mut part_2_coords: Vec<i32> = vec![0, 0];
    let mut part_2_aim: i32 = 0;
    for action in &actions {
        match action {
            Down(x) => part_2_aim += x,
            Up(x) => part_2_aim -= x,
            Forward(x) => {
                part_2_coords[0] += x;
                part_2_coords[1] += x * part_2_aim;
            }
        }
    }
    println!(
        "{}*{}={}",
        part_2_coords[0],
        part_2_coords[1],
        part_2_coords[0] * part_2_coords[1]
    );
}
