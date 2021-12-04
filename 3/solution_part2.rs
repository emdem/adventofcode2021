use std::fs;
use std::vec::Vec;


fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading file");
    let samples = input.lines().collect::<Vec<&str>>();

    let o2 = get_rating(&samples, true);
    let co2 = get_rating(&samples, false);

    println!("{}", o2 * co2);
}

fn get_rating(samples: &Vec<&str>, o2: bool) -> u32 {
    let mut filtered = (*samples).clone();
    for i in 0..samples[0].len() {
        let num_ones = filtered.iter().filter(|s| {s.as_bytes()[i] as char == '1'}).count();
        let num_zeros = filtered.len() - num_ones;
        let keep_char: char;
        if num_zeros == num_ones {
            if o2 {
                keep_char = '1';
            } else {
                keep_char = '0';
            }
        }
        else {
            let most_common;
            if num_ones > num_zeros {
                most_common = 1;
            } else {
                most_common = 0;
            }
            keep_char = char::from_digit(if o2 {most_common} else { 1 - most_common}, 10).unwrap();
        }
        filtered = filtered.into_iter().filter(|s| {s.as_bytes()[i] as char == keep_char}).collect();
        if filtered.len() < 2 { break; }
    }
    let matching_sample = filtered[0];
    let base: u32 = 2;
    let mut result = 0;
    for i in 0..samples[0].len() {
        if matching_sample.as_bytes()[i] as char == '1' {
            let bit_value = base.pow((samples[0].len()-i-1) as u32);
            result += bit_value;
        }
    }

    result
}
