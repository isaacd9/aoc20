use std::io::{self, Read};

fn parse(st: &String) -> Result<Vec<u8>, std::num::ParseIntError> {
    let mut st_cloned = st.clone().trim().to_string();
    for _ in 0..(st_cloned.len() % 4) {
        st_cloned.push('0');
    }
    println!("cloned {}", st_cloned);
    (0..st_cloned.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&st_cloned[i..i + 2], 16))
        .collect()
}

fn main() {
    let stdin = io::stdin();

    let mut st = String::new();
    stdin.lock().read_to_string(&mut st).unwrap();

    println!("{:?}", st);
    println!("{:x?}", parse(&st).unwrap());
}
