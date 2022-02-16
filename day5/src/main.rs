use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::{Add, Sub};

#[derive(Copy, Clone, Debug, PartialEq, Hash, Eq)]
struct Point {
    x: i32,
    y: i32,
}
impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl<'a, 'b> Add<&'b Point> for &'a Point {
    type Output = Point;

    fn add(self, other: &'b Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl<'a, 'b> Sub<&'b Point> for &'a Point {
    type Output = Point;

    fn sub(self, other: &'b Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
impl Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Point {
    fn from_tuple(coord: (i32, i32)) -> Point {
        Point {
            x: coord.0,
            y: coord.1,
        }
    }
    fn from_vec(coord: Vec<i32>) -> Point {
        assert_eq!(coord.len(), 2);
        Point {
            x: coord[0],
            y: coord[1],
        }
    }
    // give a unit vector in the direction of the other point
    fn direction_to(&self, other: &Point) -> Point {
        let difference: Point = other - self;
        // signum
        Point {
            x: match difference.x {
                num if num < 0 => -1,
                0 => 0,
                _ => 1,
            },
            y: match difference.y {
                num if num < 0 => -1,
                0 => 0,
                _ => 1,
            },
        }
    }
}
#[derive(Debug)]
struct Pair {
    p1: Point,
    p2: Point,
}
impl Pair {
    fn diagonal(&self) -> bool {
        self.p1.x != self.p2.x && self.p1.y != self.p2.y
    }
    // return vector of all points that the line between p1 and p2 crosses
    fn crosses(&self) -> Vec<Point> {
        let mut results = vec![self.p1];
        let direction = self.p1.direction_to(&self.p2);
        let mut current = self.p1;
        loop {
            let next = current + direction;
            results.push(next);

            if next == self.p2 {
                break;
            }

            current = next;
        }
        results
    }
}
fn main() {
    let file = File::open("input.txt").expect("File wasn't found");
    let reader = BufReader::new(file);

    let coords: Vec<Pair> = reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let (first, second) = line.split_once(" -> ").unwrap();
            Pair {
                p1: Point::from_vec(
                    first
                        .split(",")
                        .map(|num| num.parse::<i32>().unwrap())
                        .collect(),
                ),
                p2: Point::from_vec(
                    second
                        .split(",")
                        .map(|num| num.parse::<i32>().unwrap())
                        .collect(),
                ),
            }
        })
        .collect();
    println!("Part 1: {}", part1(&coords))
}
fn part1(coords: &Vec<Pair>) -> usize {
    let mut grid: HashMap<Point, u32> = HashMap::new();

    for pair in coords {
        // if pair.diagonal() {
        //     continue;
        // }
        for point in pair.crosses() {
            *grid.entry(point).or_default() += 1;
        }
    }
    grid.into_iter().filter(|entry| entry.1 >= 2).count()
}
