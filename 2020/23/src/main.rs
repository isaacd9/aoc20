use std::io::{self, BufRead, Error, Lines, Read, StdinLock};

#[derive(Debug, PartialEq, Clone)]
struct Game {
    current_cup: usize,
    cups: Vec<u64>,
}

impl Game {
    fn index(&self, i: usize) -> usize {
        if i > self.cups.len() {
            //println!("oh no {} {}", i, self.cups.len())
        }
        i % self.cups.len()
    }

    fn find_destination(&self, d: u64) -> usize {
        let min = self.cups.iter().min().unwrap();
        let mut desired = d - 1;

        while desired >= *min {
            match self.cups.iter().position(|c| *c == desired) {
                Some(i) => return i,
                None => desired -= 1,
            }
        }
        self.cups
            .iter()
            .enumerate()
            .max_by_key(|(_, label)| *label)
            .unwrap()
            .0
    }

    fn play_round(&mut self) {
        let cur = self.cups[self.current_cup];

        let first = self.cups[self.index(self.current_cup + 1)];
        let second = self.cups[self.index(self.current_cup + 2)];
        let third = self.cups[self.index(self.current_cup + 3)];
        self.cups
            .remove(self.cups.iter().position(|c| *c == first).unwrap());
        self.cups
            .remove(self.cups.iter().position(|c| *c == second).unwrap());
        self.cups
            .remove(self.cups.iter().position(|c| *c == third).unwrap());

        println!("pick up: {},{},{}", first, second, third);

        let dest = self.find_destination(cur);
        println!("destination: {} ({})", dest, self.cups[dest]);

        self.cups.insert(dest + 1, first);
        self.cups.insert(dest + 2, second);
        self.cups.insert(dest + 3, third);

        let new_cur = self.cups.iter().position(|c| *c == cur).unwrap();
        //println!("cur moved {} places", new_cur - self.current_cup);
        if dest < self.current_cup {
            self.cups.rotate_left(new_cur - self.current_cup)
        }
    }

    fn play(&mut self, rounds: u64) {
        for i in 0..rounds {
            println!("move {:?}", i);
            println!("{:?}", self);
            self.play_round();
            self.current_cup = (self.current_cup + 1) % (self.cups.len());
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut buffer = String::new();
    stdin.lock().read_to_string(&mut buffer);

    let cups: Vec<u64> = buffer
        .trim()
        .chars()
        .map(|c| c.to_string().parse().unwrap())
        .collect();

    let mut g = Game {
        current_cup: 0,
        cups: cups,
    };

    g.play(100);
    println!("{:?}", g);
}
