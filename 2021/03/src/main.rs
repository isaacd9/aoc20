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

    let mut oxygen_generator_considered: Vec<bool> = vec![true; report.len()];
    let mut co2_scrubber_considered: Vec<bool> = vec![true; report.len()];

    for i in 0..len {
        let mut p1_counts = vec![0, 0];

        let mut oxygen_generator_counts = vec![0, 0];
        let mut co2_scrubber_counts = vec![0, 0];
        for (n, num) in report.iter().enumerate() {
            let bit = num.chars().nth(i).unwrap();
            let digit = bit.to_digit(10).unwrap() as usize;
            p1_counts[digit] += 1;

            if oxygen_generator_considered[n] {
                oxygen_generator_counts[digit] += 1;
            }
            if co2_scrubber_considered[n] {
                co2_scrubber_counts[digit] += 1;
            }
        }

        // part 1
        gamma[i] = if p1_counts[0] > p1_counts[1] {
            '0'
        } else {
            '1'
        };
        epsilon[i] = if p1_counts[0] > p1_counts[1] {
            '1'
        } else {
            '0'
        };

        // Part 2
        let oxygen_generator_condition = if oxygen_generator_counts[0] > oxygen_generator_counts[1]
        {
            '0'
        } else {
            '1'
        };
        let co2_scrubber_condition = if co2_scrubber_counts[0] > co2_scrubber_counts[1] {
            '1'
        } else {
            '0'
        };
        //println!("oxy: {:?}", oxygen_generator_considered);
        //println!("co2: {:?}", co2_scrubber_considered);
        for (n, num) in report.iter().enumerate() {
            let bit = num.chars().nth(i).unwrap();

            if bit != oxygen_generator_condition
                && oxygen_generator_considered.iter().filter(|c| **c).count() > 1
            {
                oxygen_generator_considered[n] = false;
            }

            if bit != co2_scrubber_condition
                && co2_scrubber_considered.iter().filter(|c| **c).count() > 1
            {
                co2_scrubber_considered[n] = false;
            }
        }
    }

    let gamma_n = bin_from_vec(&gamma);
    let epsilon_n = bin_from_vec(&epsilon);

    // Part 1
    println!("{:?}={}", gamma, gamma_n);
    println!("{:?}={}", epsilon, epsilon_n);
    println!("{}*{}={}", gamma_n, epsilon_n, gamma_n * epsilon_n);

    // Part 2
    let mut oxygen_rating_i = 0;
    for (n, considered) in oxygen_generator_considered.iter().enumerate() {
        if *considered {
            oxygen_rating_i = n
        }
    }
    let mut co2_rating_i = 0;
    for (n, considered) in co2_scrubber_considered.iter().enumerate() {
        if *considered {
            co2_rating_i = n
        }
    }

    let oxygen_rating_n = bin_from_vec(&report[oxygen_rating_i].chars().collect::<Vec<char>>());
    let co2_rating_n = bin_from_vec(&report[co2_rating_i].chars().collect::<Vec<char>>());

    //println!(
    //    "{:?} ({})",
    //    oxygen_generator_considered, report[oxygen_rating_i]
    //);
    //println!("{:?} ({})", co2_scrubber_considered, report[co2_rating_i]);
    println!(
        "{}*{}={}",
        oxygen_rating_n,
        co2_rating_n,
        oxygen_rating_n * co2_rating_n
    );

    Ok(())
}
