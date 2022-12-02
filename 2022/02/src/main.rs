use std::{
    io::{self, BufRead, Read},
    vec,
};

enum RockPaperScissors {
    Rock,
    Paper,
    Scissors,
}

enum Outcome {
    Loss = 0,
    Draw = 3,
    Win = 6,
}

fn op_play(st: &str) -> RockPaperScissors {
    match st {
        "A" => RockPaperScissors::Rock,
        "B" => RockPaperScissors::Paper,
        "C" => RockPaperScissors::Scissors,
        _ => panic!("unknown!"),
    }
}

fn you_play_1(st: &str) -> RockPaperScissors {
    match st {
        "X" => RockPaperScissors::Rock,
        "Y" => RockPaperScissors::Paper,
        "Z" => RockPaperScissors::Scissors,
        _ => panic!("unknown!"),
    }
}

fn you_play_2(st: &str, op_play: &RockPaperScissors) -> RockPaperScissors {
    use Outcome::*;
    use RockPaperScissors::*;

    let outcome = match st {
        "X" => Loss,
        "Y" => Draw,
        "Z" => Win,
        _ => panic!("unknown outcome"),
    };

    match (outcome, op_play) {
        // Rock
        (Loss, Rock) => Scissors,
        (Draw, Rock) => Rock,
        (Win, Rock) => Paper,

        // Paper
        (Loss, Paper) => Rock,
        (Draw, Paper) => Paper,
        (Win, Paper) => Scissors,

        // Scissors
        (Loss, Scissors) => Paper,
        (Draw, Scissors) => Scissors,
        (Win, Scissors) => Rock,
    }
}

fn outcome(you_p: &RockPaperScissors, op_p: &RockPaperScissors) -> Outcome {
    use Outcome::*;
    use RockPaperScissors::*;

    match (you_p, op_p) {
        // Rock
        (Rock, Rock) => Draw,
        (Rock, Paper) => Loss,
        (Rock, Scissors) => Win,

        // Paper
        (Paper, Rock) => Win,
        (Paper, Paper) => Draw,
        (Paper, Scissors) => Loss,

        // Scissors
        (Scissors, Rock) => Loss,
        (Scissors, Paper) => Win,
        (Scissors, Scissors) => Draw,
    }
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(|line| line.unwrap());

    let score: i32 = lines
        .map(|line| {
            let mut sp = line.split_ascii_whitespace();
            let opponent = sp.next().unwrap();
            let you = sp.next().unwrap();

            let op_p = op_play(opponent);
            let you_p = you_play_2(you, &op_p);

            let outcome_score = outcome(&you_p, &op_p) as i32;
            let shape_score = match you_p {
                RockPaperScissors::Rock => 1,
                RockPaperScissors::Paper => 2,
                RockPaperScissors::Scissors => 3,
            };
            outcome_score + shape_score
        })
        .sum();

    println!("{}", score)
}
