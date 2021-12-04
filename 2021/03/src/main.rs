use std::io::{self, BufRead};

fn bin_from_vec(v: &[char]) -> i32 {
    let st = &v.iter().collect::<String>();
    i32::from_str_radix(st, 2).unwrap()
}

fn main() -> Result<(), io::Error> {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(|line| line.unwrap());

    let report: Vec<String> = lines.collect();
    let len = report[0].len();

    let mut gamma = vec!['0'; len];
    let mut epsilon = vec!['0'; len];

    for i in 0..len {
        let mut counts = vec![0, 0];
        for num in &report {
            let bit = num.chars().nth(i).unwrap();
            counts[bit.to_digit(10).unwrap() as usize] += 1;
        }

        gamma[i] = if counts[0] > counts[1] { '0' } else { '1' };
        epsilon[i] = if counts[0] > counts[1] { '1' } else { '0' };
    }

    let gamma_n = bin_from_vec(&gamma);
    let epsilon_n = bin_from_vec(&epsilon);

    // Part 1
    println!("{:?}={}", gamma, gamma_n);
    println!("{:?}={}", epsilon, epsilon_n);
    println!("{}*{}={}", gamma_n, epsilon_n, gamma_n * epsilon_n);
    Ok(())
}
