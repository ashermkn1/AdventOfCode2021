use std::borrow::BorrowMut;
use std::cmp::{Eq, Ord, Reverse};
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").expect("File wasn't found");
    let reader = BufReader::new(file);

    let digits: Vec<Vec<u32>> = reader
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|ch| ch.to_digit(10).unwrap() as u32)
                .collect()
        })
        .collect();
    println!("Power Consumption: {}", part1(&digits));
    println!("Life Support: {}", part2(&digits));
}

fn part1(digits: &Vec<Vec<u32>>) -> u32 {
    let mut gamma: u32 = 0;
    let mut epsilon: u32 = 0;
    for i in 0..digits[0].len() {
        let most_common = most_frequent_bit(nth(digits, i)).unwrap_or(0);
        gamma = (gamma << 1) + most_common;
        // invert bit: 1->0 and 0->1
        let least_common = !most_common & 1;
        epsilon = (epsilon << 1) + least_common;
    }
    epsilon * gamma
}
fn nth(digits: &Vec<Vec<u32>>, n: usize) -> Vec<u32> {
    digits.iter().map(|v| v[n]).collect()
}
fn part2(digits: &Vec<Vec<u32>>) -> u32 {
    let oxygen = dbg!(oxygen_rating(digits));
    let co2 = dbg!(co2_scrubber(digits));
    co2 * oxygen
}
fn oxygen_rating(digits: &Vec<Vec<u32>>) -> u32 {
    let mut digits = digits.clone();
    for i in 0..digits[0].len() {
        let bit = most_frequent_bit(nth(&digits, i)).unwrap_or(1);

        digits.retain(|v| v[i] == bit);
        println!("{:?}", digits);
        if digits.iter().count() == 1 {
            // concat vec of digits into integer
            return digits
                .iter()
                .next()
                .unwrap()
                .iter()
                .fold(0, |acc: u32, elem: &u32| (acc << 1) + elem);
        }
    }
    1
}
fn co2_scrubber(digits: &Vec<Vec<u32>>) -> u32 {
    let mut digits = digits.clone();
    for i in 0..digits[0].len() {
        let bit = most_frequent_bit(nth(&digits, i)).unwrap_or(1);

        digits.retain(|v| v[i] != bit);

        if digits.iter().count() == 1 {
            // concat vec of digits into integer
            return digits
                .iter()
                .next()
                .unwrap()
                .iter()
                .fold(0, |acc: u32, elem: &u32| (acc << 1) + elem);
        }
    }
    1
}
fn most_frequent_bit(nums: Vec<u32>) -> Option<u32> {
    let count_of_one = nums.iter().filter(|x| **x == 1).count();
    if count_of_one == nums.len() / 2 && nums.len() % 2 == 0 {
        None
    } else if count_of_one > (nums.len() / 2) {
        Some(1)
    } else {
        Some(0)
    }
}
