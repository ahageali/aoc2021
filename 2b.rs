use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();

    let mut horizontal_pos = 0i32;
    let mut depth_pos = 0i32;
    let mut aim = 0i32;
    for line in stdin.lock().lines() {
        let unwrapped_line = line.unwrap();
        let split_line : Vec<&str> = unwrapped_line.split(" ").collect();
        let action_word = split_line[0];
        let units : i32 = split_line[1].parse().unwrap();
        eprintln!("parsed action word {} units {}", action_word, units);

        match action_word {
            "forward" => {
                horizontal_pos = horizontal_pos + units;
                depth_pos = depth_pos + (aim * units);
            },
            "up" => {
                aim = aim - units;
            },
            "down" => {
                aim = aim + units;
            },
            _ => {
                panic!("unknown value");
            }
        }
    }

    println!("{}", horizontal_pos * depth_pos);
}