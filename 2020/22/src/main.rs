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

    fn calculate_score(&self) -> (u64, bool) {
        let mut winner = true;
        let winner_deck = if self.deck_one.len() > 0 {
            winner = true;
            &self.deck_one
        } else {
            winner = false;
            &self.deck_two
        };

        (
            winner_deck
                .iter()
                .rev()
                .enumerate()
                .fold(0, |acc, (index, card)| acc + card * (index as u64 + 1)),
            winner,
        )
    }

    fn play_one_round_recursive(&mut self) {
        //println!("{:?} {:?}", self.deck_one, self.deck_two);
        let one_card = self.deck_one.remove(0);
        let two_card = self.deck_two.remove(0);

        //println!("{} {}", one_card, two_card);
        let player_one_wins = if one_card as usize <= self.deck_one.len()
            && two_card as usize <= self.deck_two.len()
        {
            // Recursive Combat
            //println!("RECURSIVE COMBAT!!!");
            let mut new_game = self.clone();
            new_game.deck_one.drain(one_card as usize..);
            new_game.deck_two.drain(two_card as usize..);
            new_game.play_recursive().1
        } else {
            // Regular Combat
            if one_card > two_card {
                true
            } else {
                false
            }
        };

        if player_one_wins {
            self.deck_one.push(one_card);
            self.deck_one.push(two_card);
        } else {
            self.deck_two.push(two_card);
            self.deck_two.push(one_card);
        }
    }

    fn serialize_game(&self) -> String {
        format!(
            "{}-{}",
            self.deck_one
                .iter()
                .map(|num| format!("{}", num))
                .collect::<String>(),
            self.deck_two
                .iter()
                .map(|num| format!("{}", num))
                .collect::<String>(),
        )
    }

    fn play_recursive(&mut self) -> (u64, bool) {
        let mut rounds = 0;
        let mut prev_positions = HashSet::<String>::new();

        while self.deck_one.len() > 0 && self.deck_two.len() > 0 {
            //println!("playing round {:?}", rounds);
            self.play_one_round_recursive();
            if prev_positions.contains(&self.serialize_game()) {
                return (0, true);
            }
            rounds += 1;
            prev_positions.insert(self.serialize_game());
        }
        //println!("game over. played {:?} rounds", rounds,);
        self.calculate_score()
    }

    fn play(&mut self) -> (u64, bool) {
        while self.deck_one.len() > 0 && self.deck_two.len() > 0 {
            self.play_one_round();
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

    let mut game_one = game.clone();
    let mut game_two = game.clone();

    //let score = game_one.play();
    //println!("{:?}", score);

    let score = game_two.play_recursive();
    println!("{:?}", score);
}
