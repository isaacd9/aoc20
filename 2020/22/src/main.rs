use std::collections::{HashMap, HashSet};
use std::fmt;
use std::io::{self, BufRead, Error, Lines, StdinLock};
use std::iter::FromIterator;

#[derive(Debug, PartialEq, Clone)]
struct Game {
    deck_one: Vec<u64>,
    deck_two: Vec<u64>,
}

impl Game {
    fn play_one_round(&mut self) {
        let one_card = self.deck_one.remove(0);
        let two_card = self.deck_two.remove(0);

        if one_card > two_card {
            self.deck_one.push(one_card);
            self.deck_one.push(two_card);
        } else {
            self.deck_two.push(two_card);
            self.deck_two.push(one_card);
        }
    }

    fn calculate_score(&self) -> u64 {
        let winner = if self.deck_one.len() > 0 {
            &self.deck_one
        } else {
            &self.deck_two
        };

        winner
            .iter()
            .rev()
            .enumerate()
            .fold(0, |acc, (index, card)| acc + card * (index as u64 + 1))
    }

    fn play(&mut self) -> u64 {
        let mut rounds = 0;
        while self.deck_one.len() > 0 && self.deck_two.len() > 0 {
            self.play_one_round();
            rounds += 1;
        }
        //println!("payed {:?} rounds", rounds);
        self.calculate_score()
    }
}

fn read_input(lines: Lines<StdinLock>) -> Game {
    let mut li = lines.map(|li| li.unwrap());

    let mut deck_one: Vec<u64> = li
        .by_ref()
        .take_while(|li| li != "")
        .skip(1)
        .map(|nu| nu.parse().unwrap())
        .collect();

    let mut deck_two: Vec<u64> = li.by_ref().skip(1).map(|nu| nu.parse().unwrap()).collect();

    Game { deck_one, deck_two }
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    let mut game = read_input(lines);

    let score = game.play();
    println!("{:?}", score);
}
