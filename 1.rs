use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut prev_value = i32::MAX;
    let mut total_increases = 0;
    for line in stdin.lock().lines() {
        let parsed_value : i32 = line.unwrap().parse().unwrap();
        eprintln!("parsed {}", parsed_value);
        if parsed_value > prev_value {
            total_increases = total_increases + 1;
        }
        prev_value = parsed_value;
    }

    println!("{}", total_increases);
}