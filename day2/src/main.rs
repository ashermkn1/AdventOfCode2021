use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").expect("File wasn't found");
    let reader = BufReader::new(file);

    let numbers: Vec<i32> = reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let mut parts = line.split_whitespace();
            (
                String::from(parts.next().unwrap()),
                parts.next().unwrap().parse::<i32>().unwrap(),
            )
        })
        .collect();

    println!("Final: {}", part2(numbers));
}

fn part1(commands: Vec<(String, i32)>) -> i32 {
    let mut depth = 0;
    let mut pos = 0;
    for command in commands {
        match command.0.as_str() {
            "forward" => pos += command.1,
            "up" => depth -= command.1,
            "down" => depth += command.1,
            _ => {}
        }
    }
    depth * pos
}
fn part2(commands: Vec<(String, i32)>) -> i32 {
    let mut depth = 0;
    let mut pos = 0;
    let mut aim = 0;
    for command in commands {
        let x = command.1;
        match command.0.as_str() {
            "up" => aim += x,
            "down" => aim -= x,
            "forward" => {
                pos += x;
                depth += aim * x;
            }
            _ => {}
        }
    }
    depth * pos
}
