use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::{BufReader, Read};
#[derive(Debug)]
struct Lanternfish {
    timer: u32,
}
impl Lanternfish {
    fn new() -> Lanternfish {
        Lanternfish { timer: 8 }
    }
    fn from_timer(timer: u32) -> Lanternfish {
        Lanternfish { timer }
    }
    fn advance(&mut self) -> Option<Lanternfish> {
        if self.timer == 0 {
            self.timer = 6;
            return Some(Lanternfish::new());
        }
        self.timer -= 1;

        None
    }
}
struct BigSchool {
    fish_map: HashMap<u64, u64>,
}
struct School {
    fish_vec: Vec<Lanternfish>,
}
impl School {
    fn new() -> School {
        School {
            fish_vec: Vec::new(),
        }
    }
    fn from_vec(fish: Vec<Lanternfish>) -> School {
        School { fish_vec: fish }
    }
    // advance one day
    fn advance(&mut self) {
        let mut new_fish: Vec<Lanternfish> = Vec::new();
        for mut fish in &mut self.fish_vec {
            if let Some(x) = fish.advance() {
                new_fish.push(x);
            }
        }
        self.fish_vec.append(&mut new_fish);
    }
    fn count(&self) -> usize {
        self.fish_vec.len()
    }
    fn simulate(&mut self, days: u32) {
        println!("Current State: {:?}", self.fish_vec);

        for day in 1..days + 1 {
            self.advance();
            //println!("Number of fish after day {}: {}", day, self.count())
        }
    }
}
impl BigSchool {
    fn new() -> BigSchool {
        BigSchool {
            fish_map: HashMap::new(),
        }
    }
    fn from_vec(fish: Vec<u32>) -> BigSchool {
        let mut map: HashMap<u64, u64> = HashMap::new();
        for item in fish {
            *map.entry(item as u64).or_default() += 1;
        }

        BigSchool { fish_map: map }
    }
    fn advance(&mut self) {
        let mut new_map: HashMap<u64, u64> = HashMap::new();
        for (&key, &value) in &self.fish_map {
            if key == 0 {
                *new_map.entry(6).or_default() += value;
                *new_map.entry(8).or_default() += value;
            } else {
                *new_map.entry(key - 1).or_default() += value;
            }
        }
        self.fish_map = new_map;
    }
    fn count(&self) -> u64 {
        self.fish_map.values().sum()
    }
    fn simulate(&mut self, days: u32) -> u64 {
        println!("Current count: {}", self.count());
        for day in 0..days {
            self.advance();
            println!("Count after day {}: {}", day + 1, self.count());
        }
        self.count()
    }
}

fn main() {
    let file = File::open("input.txt").expect("File wasn't found");
    let mut reader = BufReader::new(file);

    let mut population = String::new();
    reader.read_to_string(&mut population);

    let mut population = BigSchool::from_vec(
        population
            .split(',')
            .map(|item| item.parse().unwrap())
            .collect(),
    );

    population.simulate(256);
    println!("{}", population.count());
}
