use std::fs;
use std::vec::Vec;


fn to_decimal(bits: &mut Vec<i32>) -> i32 {
    let mut value = 0;
    for (index, bit) in bits.iter().enumerate() {
        value += (bit * i32::pow(2, ((bits.len() - (index+1)) as u32)));
    }
    return value;
}


fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading file");
    let mut bit_counts = vec![0; 0];
    let mut total_entries = 0;
    for line in input.lines() {
        //first iteration make da array
        if bit_counts.len() < line.chars().count() {
            bit_counts = vec![0; line.chars().count()];
        }
        total_entries += 1; //to calculate
        for (index, digit) in line.chars().enumerate() {
            if digit.to_digit(10) > Some(0) {
                bit_counts[index] += 1;
            }
        }
    }
    let mut gamma_bits = vec![0; bit_counts.len()];
    let mut epsilon_bits = vec![0; bit_counts.len()];

    let mut index = 0;
    for bit_count in bit_counts {
        let half = total_entries/2;
        if bit_count > half {
            gamma_bits[index] = 1;
            epsilon_bits[index] = 0;
        } else {
            gamma_bits[index] = 0;
            epsilon_bits[index] = 1;
        }
        index += 1;
    }
    let mut gamma = to_decimal(&mut gamma_bits);
    let mut epsilon = to_decimal(&mut epsilon_bits);
    println!("{}", epsilon*gamma);
}


