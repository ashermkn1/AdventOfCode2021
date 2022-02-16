use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct HeightMap {
    map: Vec<Vec<u32>>,
}
impl HeightMap {
    fn basins(&self) -> u32 {
        let mut sizes: Vec<u32> = Vec::new();
        // breadth first search
        for (start_x, start_y) in self.low_points_coord() {
            println!("Basin for ({}, {})", start_x, start_y);
            let mut size = 1;
            let mut visited: Vec<(u32, u32)> = vec![(start_x, start_y)];
            let mut frontier: VecDeque<(u32, u32)> = VecDeque::from(
                self.adjacent(start_x, start_y)
                    .iter()
                    .filter(|(x, y)| self.map[*x as usize][*y as usize] != 9)
                    .copied()
                    .collect::<Vec<(u32, u32)>>(),
            );
            while let Some((i, j)) = frontier.pop_front() {
                visited.push((i, j));
                size += 1;
                // add adjacent tiles that haven't been visited yet, aren't == 9,
                // and are greater than the origin
                frontier.append(&mut VecDeque::from(
                    self.adjacent(i, j)
                        .iter()
                        .filter(|(x, y)| {
                            self.map[*x as usize][*y as usize] != 9
                                && !visited.contains(&(*x, *y))
                                && !frontier.contains(&(*x, *y))
                                && self.map[*x as usize][*y as usize]
                                    > self.map[i as usize][j as usize]
                        })
                        .copied()
                        .collect::<Vec<(u32, u32)>>(),
                ))
            }
            sizes.push(size);
        }
        sizes.sort_by(|a, b| b.cmp(a));
        sizes
            .iter()
            .take(3)
            .copied()
            .reduce(|acc, elem| acc * elem)
            .unwrap()
    }
    fn adjacent(&self, x: u32, y: u32) -> Vec<(u32, u32)> {
        let x = x as i32;
        let y = y as i32;
        vec![(x - 1, y), (x, y - 1), (x + 1, y), (x, y + 1)]
            .iter()
            .filter(|(i, j)| *i >= 0 && *j >= 0)
            .copied()
            .filter(|(i, j)| (*i as usize) < self.map.len() && (*j as usize) < self.map[0].len())
            .map(|(i, j)| (i as u32, j as u32))
            .collect()
    }
    fn low_points_coord(&self) -> Vec<(u32, u32)> {
        let mut res = Vec::new();
        for (i, row) in self.map.iter().enumerate() {
            for (j, _) in row.iter().enumerate() {
                if self.lowest_of_neighbours(i, j) {
                    res.push((i as u32, j as u32));
                }
            }
        }

        res
    }
    fn low_points_value(&self) -> Vec<u32> {
        let mut res = Vec::new();
        for (i, row) in self.map.iter().enumerate() {
            for (j, &element) in row.iter().enumerate() {
                if self.lowest_of_neighbours(i, j) {
                    res.push(element);
                }
            }
        }

        res
    }
    fn risk_levels(&self) -> u32 {
        let low = self.low_points_value();
        low.iter().sum::<u32>() + low.len() as u32
    }

    fn lowest_of_neighbours(&self, x: usize, y: usize) -> bool {
        let element = self.map[x][y];

        let x = x as i32;
        let y = y as i32;
        for i in x - 1..x + 2 {
            if i < 0 || i as usize >= self.map.len() {
                continue;
            }
            for j in y - 1..y + 2 {
                if j < 0 || j as usize >= self.map[0].len() {
                    continue;
                }
                if i != x && j != y {
                    continue;
                }
                if i == x && j == y {
                    continue;
                }
                if self.map[i as usize][j as usize] <= element {
                    return false;
                }
            }
        }
        true
    }
}
fn main() {
    let file = File::open("input.txt").expect("File wasn't found");
    let reader = BufReader::new(file);

    let map: HeightMap = HeightMap {
        map: reader
            .lines()
            .map(|line| {
                line.unwrap()
                    .split("")
                    .filter(|x| !x.is_empty())
                    .map(|digit| digit.parse::<u32>().unwrap())
                    .collect()
            })
            .collect(),
    };
    println!("Part 1: {}", map.risk_levels());
    println!("Part 2: {}", map.basins());
}
