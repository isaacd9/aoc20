use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::io::{self, BufRead, Error, Lines, StdinLock};
use std::time::SystemTime;

#[derive(PartialEq, Clone, Copy)]
struct Bitmap(u64, u64);

impl Bitmap {
    fn floating_iterator(&self) -> impl Iterator<Item = Bitmap> + '_ {
        let xor = self.0 ^ self.1;

        let x_s = xor.count_ones();
        (0..2_u64.pow(x_s)).map(move |mut v| {
            let mut x = xor;
            for _ in 0..64 {
                if x & 1 == 1 {
                    x &= std::u64::MAX << 1;
                    x |= v & 1;
                    v = v.rotate_right(1);
                }
                x = x.rotate_right(1);
            }

            Bitmap(!xor, x)
        })
    }
}

impl From<&str> for Bitmap {
    fn from(in_s: &str) -> Self {
        let st = String::from(in_s).replace("X", "1");
        let and = u64::from_str_radix(st.as_str(), 2).unwrap();

        let st = String::from(in_s).replace("X", "0");
        let or = u64::from_str_radix(st.as_str(), 2).unwrap();
        Bitmap(and, or)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let b = Bitmap::from("00000000000000000000000000000000X0XX");
        assert_eq!(b, Bitmap(0b1011, 0b0));
    }

    #[test]
    fn test_floating_iter() {
        let b = Bitmap::from("00000000000000000000000000000000X0XX");
        let v: Vec<Bitmap> = b.floating_iterator().collect();
        for i in v {
            println!("{:?}", i);
        }
        ()
    }
}

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
                (Some(caps), _) => Mask(Bitmap::from(&caps["mask"])),
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

struct Arena(HashMap<u64, u64>);

impl Arena {
    fn new() -> Arena {
        Arena(HashMap::new())
    }

    fn sum(&self) -> u64 {
        self.0.values().sum()
    }

    fn eval_instructions_part_one(&mut self, instructions: &Vec<Instruction>) {
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
                    *(self.0).entry(*addr).or_insert(0) = v;
                }
            }
        }
    }

    fn eval_instructions_part_two(&mut self, instructions: &Vec<Instruction>) {
        use crate::Instruction::*;

        let mut cur_mask: Bitmap = Bitmap(std::u64::MAX, 0);
        for instruction in instructions {
            match instruction {
                Mask(m) => cur_mask = *m,
                Mem { addr, value } => {
                    let mut a = *addr;
                    // Add 1s
                    a |= cur_mask.1;

                    //println!("before: {:#b} {}", a, cur_mask);
                    for m in cur_mask.floating_iterator() {
                        // Clear 0s
                        a &= m.0;
                        // Set 1s
                        a |= m.1;
                        *(self.0).entry(a).or_insert(0) = *value;
                    }
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
    arena.eval_instructions_part_one(&instructions);
    println!("{:?}", arena.sum());

    let mut arena = Arena::new();
    arena.eval_instructions_part_two(&instructions);
    println!("{:?}", arena.sum());
}
