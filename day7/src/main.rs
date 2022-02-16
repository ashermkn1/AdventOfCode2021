use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};

fn main() {
    let file = File::open("input.txt").expect("File wasn't found");
    let mut reader = BufReader::new(file);
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer);
    let crabs: Vec<u32> = buffer
        .split(',')
        .map(|num| num.parse::<u32>().unwrap())
        .collect();
    let median = median(&mut crabs.clone());
    let avg = crabs.iter().sum::<u32>() as f32 / crabs.len() as f32 + 0.5;

    println!("Median: {}", median);
    println!("Center of crab: {}", avg);
    println!(
        "Fuel needed for part 1: {}",
        crabs
            .iter()
            .map(|&crab| (median as i32 - crab as i32).abs() as u32)
            .sum::<u32>()
    );
    println!(
        "Fuel needed for part 2: {}",
        crabs.iter().map(|&crab| weird_sum(crab, 464)).sum::<u32>()
    )
}
fn weird_sum(start: u32, end: u32) -> u32 {
    let n = (start as i32 - end as i32).abs();
    ((n * (n + 1)) / 2) as u32
}
fn median(numbers: &mut Vec<u32>) -> u32 {
    numbers.sort();
    let mid = numbers.len() / 2;
    numbers[mid]
}
