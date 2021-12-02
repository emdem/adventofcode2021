use std::fs;
use std::vec::Vec;

fn main() {
    let mut depth = 0;
    let mut position = 0;
    //let input = fs::read_to_string("test_input.txt").expect("Error reading file");
    let input = fs::read_to_string("input.txt").expect("Error reading file");

    for line in input.lines() {
        let unsigned_val:u32 = line.chars().last().unwrap().to_digit(10).unwrap();
        let distance:i32 = unsigned_val as i32;

        if line.contains("down") {
            depth += distance;
        }
        if line.contains("up") {
            depth -= distance;
        }

        if line.contains("forward") {
            position += distance;
        } 
    }
    println!("answer: {}", depth * position);
}
