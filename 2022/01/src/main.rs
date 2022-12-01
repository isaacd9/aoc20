use std::{
    io::{self, Read},
    vec,
};

fn main() {
    let mut stdin = io::stdin();
    let mut buf = String::new();
    stdin.read_to_string(&mut buf).unwrap();

    let mut v: Vec<u32> = vec![];

    let sp = buf.split('\n');

    let mut sum = 0;
    for s in sp {
        if s.eq("") {
            v.push(sum);
            sum = 0;
        } else {
            let parsed = s.parse::<u32>().unwrap();
            sum += parsed;
        }
    }
    v.sort_unstable();
    v.reverse();
    let three = v[0] + v[1] + v[2];
    println!("{} {}", v[0], three);
}
