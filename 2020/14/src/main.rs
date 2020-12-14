use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::io::{self, BufRead, Error, Lines, StdinLock};
use std::time::SystemTime;

#[derive(PartialEq, Clone, Copy)]
struct Bitmap(u64, u64);

impl fmt::Display for Bitmap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:#b}, {:#b})", self.0, self.1)
    }
}

impl fmt::Debug for Bitmap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Bitmap")
            .field("0", &format!("{:#b}", &self.0))
            .field("1", &format!("{:#b}", &self.1))
            .finish()
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Instruction {
    Mask(Bitmap),
    Mem { addr: u64, value: u64 },
}

fn read_input(lines: Lines<StdinLock>) -> Vec<Instruction> {
    use crate::Instruction::*;

    let mask_re = Regex::new(r"mask = (?P<mask>\S+)").unwrap();
    let mem_re = Regex::new(r"mem\[(?P<addr>\d+)\] = (?P<value>\d+)").unwrap();

    let inst = lines
        .map(|li| li.unwrap())
        .map(|li| {
            let mask_caps = mask_re.captures(&li);
            let mem_caps = mem_re.captures(&li);
            match (mask_caps, mem_caps) {
                (Some(caps), _) => {
                    let st = String::from(&caps["mask"]).replace("X", "1");
                    let and = u64::from_str_radix(st.as_str(), 2).unwrap();

                    let st = String::from(&caps["mask"]).replace("X", "0");
                    let or = u64::from_str_radix(st.as_str(), 2).unwrap();
                    Mask(Bitmap(and, or))
                }
                (_, Some(caps)) => Mem {
                    addr: caps["addr"].parse().unwrap(),
                    value: caps["value"].parse().unwrap(),
                },
                (None, None) => panic!("no captures found: {}", &li),
            }
        })
        .collect();
    inst
}

struct Arena([u64; 100000]);

impl Arena {
    fn new() -> Arena {
        Arena([0; 100000])
    }

    fn sum(&self) -> u64 {
        self.0.iter().sum()
    }

    fn eval_instructions(&mut self, instructions: &Vec<Instruction>) {
        use crate::Instruction::*;

        let mut cur_mask: Bitmap = Bitmap(std::u64::MAX, 0);
        for instruction in instructions {
            match instruction {
                Mask(m) => cur_mask = *m,
                Mem { addr, value } => {
                    let mut v = *value;
                    // Clear 0s
                    v &= cur_mask.0;
                    // Add 1s
                    v |= cur_mask.1;
                    //println!("{}={}", *addr, v);
                    (self.0)[*addr as usize] = v
                }
            }
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    let instructions = read_input(lines);

    let mut arena = Arena::new();
    arena.eval_instructions(&instructions);

    println!("{:?}", arena.sum());
}
