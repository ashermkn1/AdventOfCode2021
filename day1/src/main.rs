use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").expect("File wasn't found");
    let reader = BufReader::new(file);

    let numbers: Vec<i32> = reader
        .lines()
        .map(|line| line.unwrap().parse::<i32>().unwrap())
        .collect();
    println!("Count: {}", part2(numbers));
}
fn part1(items: Vec<i32>) -> i32 {
    let mut count = 0;
    for index in 1..items.len() {
        if items.get(index).unwrap() > items.get(index - 1).unwrap() {
            count += 1;
        }
    }
    count
}
fn part2(items: Vec<i32>) -> i32 {
    let mut count = 0;
    let mut prev_slice = &items[0..3];
    for slice in items.windows(3) {
        if slice.iter().sum::<i32>() > prev_slice.iter().sum::<i32>() {
            count += 1;
        }
        prev_slice = slice;
    }
    count
}
