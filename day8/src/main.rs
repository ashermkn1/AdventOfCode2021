use std::fs::File;
use std::io::{BufRead, BufReader};
struct Display {
    patterns: Vec<String>,
    digits: Vec<String>,
}
impl Display {
    fn count_uniques(&self) -> u32 {
        let mut count = 0;
        for digit in &self.digits {
            match digit.len() {
                2 | 3 | 4 | 7 => count += 1,
                _ => (),
            }
        }
        count
    }
    fn decode_output(&self) -> u32 {
        let get_unique = |len| -> Vec<char> {
            self.patterns
                .iter()
                .filter(|s| s.len() == len)
                .next()
                .unwrap()
                .chars()
                .collect()
        };
        let get_several = |len| -> Vec<Vec<char>> {
            self.patterns
                .iter()
                .filter(|s| s.len() == len)
                .map(|s| s.clone().chars().collect())
                .collect()
        };

        //Determine which segment corresponds to the 'c' segment
        //Of the 6-segment digits, two use 'c', one doesn't
        //Compare to the ones digit, and both 'c' and 'f' segments can be determined
        let one_seg = get_unique(2);
        let six_segs = get_several(6);
        let (c_seg, f_seg) = {
            let test_char = one_seg[0];
            let count = six_segs.iter().filter(|s| s.contains(&test_char)).count();
            match count {
                2 => (test_char, one_seg[1]), //tested char is 'f', the other is 'c'
                _ => (one_seg[1], test_char), //opposite case
            }
        };
        //Determine which segment corresponds to the 'e' segment
        //Using the 'c' and 'f' segments, one can distinguish between 2, 3 and 5
        //Find the 3 and 2, then the 'e' segment is the one that is present in 2 but not 3
        let five_segs = get_several(5);
        let three_seg = five_segs
            .iter()
            .filter(|s| s.contains(&c_seg) && s.contains(&f_seg))
            .next()
            .unwrap();
        let two_seg = five_segs
            .iter()
            .filter(|s| s.contains(&c_seg) && !s.contains(&f_seg))
            .next()
            .unwrap();
        let &e_seg = two_seg
            .iter()
            .filter(|c| !three_seg.contains(&c))
            .next()
            .unwrap();

        //These three segments are enough to distinguish between the non-unique-length digits
        let decode_digit = |digit: &String| -> u32 {
            match digit.len() {
                2 => 1,
                3 => 7,
                4 => 4,
                7 => 8,
                5 => {
                    let contains_c_seg = digit.chars().collect::<Vec<char>>().contains(&c_seg);
                    let contains_f_seg = digit.chars().collect::<Vec<char>>().contains(&f_seg);
                    if contains_c_seg && contains_f_seg {
                        3
                    } else if contains_c_seg {
                        2
                    } else {
                        5
                    }
                }
                6 => {
                    let contains_c_seg = digit.chars().collect::<Vec<char>>().contains(&c_seg);
                    let contains_e_seg = digit.chars().collect::<Vec<char>>().contains(&e_seg);
                    if !contains_c_seg {
                        6
                    } else if contains_e_seg {
                        0
                    } else {
                        9
                    }
                }
                _ => panic!("borked input!!"),
            }
        };

        let mut vals = self.digits.iter().map(decode_digit).collect::<Vec<u32>>();
        vals[0] *= 1000;
        vals[1] *= 100;
        vals[2] *= 10;
        return vals.iter().sum();
    }
}
fn main() {
    let file = File::open("input.txt").expect("File wasn't found");
    let mut reader = BufReader::new(file);
    let input: Vec<Display> = reader
        .lines()
        .map(|line| {
            line.unwrap()
                .split(" | ")
                .map(|part| {
                    part.split_whitespace()
                        .map(str::to_string)
                        .collect::<Vec<String>>()
                })
                .collect()
        })
        .map(|line: Vec<Vec<String>>| Display {
            patterns: line[0].clone(),
            digits: line[1].clone(),
        })
        .collect();

    //println!("Part 1: {}", part1(&input));
    let mut total_sum = 0;
    for d in &input {
        total_sum += d.decode_output();
    }
    println!("Part 2: {}", total_sum);
}
fn part1(stuff: &Vec<Vec<Vec<String>>>) -> usize {
    stuff
        .iter()
        .map(|line| {
            line.iter()
                .nth(1)
                .unwrap()
                .iter()
                .filter(|digits| match digits.len() {
                    2 | 3 | 4 | 7 => true,
                    _ => false,
                })
                .count()
        })
        .sum::<usize>()
}
