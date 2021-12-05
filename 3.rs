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

// fn dec_to_binary(input: u8) -> u32 {
//     let mut rolling = input;

// }

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

    
    let mut gamma_num = 0u64;
    let mut epsilon_num = 0u64;
    for i in 0..max_bits {
        let multiple = 1 << (max_bits-i-1);
        let mut num_zeroes = 0;
        for num in &input_numbers {
            if num & multiple == 0 {
                num_zeroes = num_zeroes + 1;
            }
        }

        if num_zeroes < input_numbers.len() / 2 {
            gamma_num = gamma_num + multiple;
        } else {
            epsilon_num = epsilon_num + multiple;
        }
    }


    println!("{}", gamma_num * epsilon_num);
}