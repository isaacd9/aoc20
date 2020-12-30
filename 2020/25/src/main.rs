use std::collections::{HashMap, HashSet};
use std::fmt;
use std::io::{self, BufRead, Error, Lines, StdinLock};
use std::iter;
use std::iter::FromIterator;

fn transform_subject_number(subject_number: u64, loop_size: u64) -> u64 {
    let mut value = 1;
    for _ in 0..loop_size {
        value *= subject_number;
        value %= 20201227;
    }
    value
}

fn find_loop_size(subject_number: u64, public_key: u64) -> u64 {
    let mut value = 1;
    for loop_size in 0.. {
        //println!("trying loop size {}", loop_size);
        if value == public_key {
            return loop_size;
        }
        value *= subject_number;
        value %= 20201227;
    }

    0
}

fn main() {
    println!("{}", transform_subject_number(7, 8));
    println!("{}", transform_subject_number(7, 11));

    println!("{}", find_loop_size(7, 17807724));
    println!("{}", find_loop_size(7, 5764801));

    println!("{}", find_loop_size(7, 1965712));
    println!("{}", find_loop_size(7, 19072108));

    println!(
        "{}",
        transform_subject_number(19072108, find_loop_size(7, 1965712))
    );
    println!(
        "{}",
        transform_subject_number(1965712, find_loop_size(7, 19072108))
    );
}
