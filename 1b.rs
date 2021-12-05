use std::io::{self, BufRead};

fn sum(arr : [u32; 3]) -> u32 {
    arr[0] + arr[1] + arr[2]
}

fn main() {
    let stdin = io::stdin();
    let mut num_inputs = 0u32;

    let mut current_window = [0u32; 3];
    let mut index_to_replace = 0u32;

    let mut total_increases = 0u32;
    for line in stdin.lock().lines() {
        let parsed_value : u32 = line.unwrap().parse().unwrap();
        eprintln!("parsed {}", parsed_value);

        if num_inputs < 3 {
            current_window[num_inputs as usize] = parsed_value;
            num_inputs = num_inputs + 1;
        } else {
            let prev_sum = sum(current_window);
            current_window[index_to_replace as usize] = parsed_value;
            index_to_replace = (index_to_replace + 1) % 3;
            let new_sum = sum(current_window);
            if new_sum > prev_sum {
                total_increases = total_increases + 1
            }
        }
    }

    println!("{}", total_increases);
}