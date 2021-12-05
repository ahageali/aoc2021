use std::io::{self, BufRead};

fn binary_to_dec(input : u64) -> u64 {
    let mut rolling: u64 = input;
    let mut multiple = 1;
    let mut converted = 0u64;
    
    while rolling != 0 {
        if rolling % 2 == 1 { 
            converted = converted + multiple;
        }

        multiple = multiple << 1;
        rolling = rolling / 10;
        eprintln!("converted: {} multiple: {}, rolling: {}", converted, multiple, rolling);
    }

    converted
}

fn cnt_num_bits(num: u64) -> u64 {
    let mut rolling = num;
    let mut count = 0;
    while rolling != 0 {
        rolling = rolling >> 1;
        count = count + 1;
    }

    count
}

fn search(input_nums : Vec<u64>, max_bits : u64, least_common: bool) -> u64 {
    let mut input_numbers = input_nums;
    for i in 0..max_bits {
        if input_numbers.len() == 1 {
            return input_numbers[0];
        }

        let multiple = 1 << (max_bits-i-1);

        eprintln!("input numbers: {:?}, multiple: {}", input_numbers, multiple);
        let mut num_zeroes = 0;
        for num in &input_numbers {
            if num & multiple == 0 {
                num_zeroes = num_zeroes + 1;
            }
        }

        let mut condition = num_zeroes > input_numbers.len() / 2;
        if least_common {
            condition = !condition;
        }

        let filtered_values;
        if condition {
            filtered_values = input_numbers.into_iter().filter(|num| (num & multiple) == 0).collect();
        } else {
            filtered_values = input_numbers.into_iter().filter(|num| (num & multiple) != 0).collect();
        }
        input_numbers = filtered_values;
    }

    assert_eq!(input_numbers.len(), 1);
    input_numbers[0]
}

fn main() {
    let stdin = io::stdin();

    let mut input_numbers : Vec<u64> = vec![];
    let mut max_bits = 0;
    for line in stdin.lock().lines() {
        let parsed_line : u64 = line.unwrap().parse().unwrap();
        let converted_num = binary_to_dec(parsed_line);

        input_numbers.push(converted_num);
        max_bits = std::cmp::max(max_bits, cnt_num_bits(converted_num));
    }

    let oxygen_rating = search(input_numbers.to_vec(), max_bits, false);
    let co2_rating = search(input_numbers.to_vec(), max_bits, true);
    eprintln!("oxygen rating: {}, co2 rating: {}", oxygen_rating, co2_rating);

    println!("{}", oxygen_rating * co2_rating);
}