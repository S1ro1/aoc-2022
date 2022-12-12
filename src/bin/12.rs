use std::collections::VecDeque;

#[derive(Debug, Clone, Copy)]
pub struct Node {
    val: char,
    visited: bool,
    x: usize,
    y: usize,
}

const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

pub fn is_node_valid(x: i32, y: i32, width: usize, height: usize, grid: &Vec<Vec<Node>>) -> bool {
    if x < 0 || x > width as i32 - 1 || y < 0 || y > height as i32 - 1 {
        return false;
    }
    if grid[y as usize][x as usize].visited == true {
        return false;
    }

    true
}

pub fn is_val_valid(old: char, new: char) -> bool {
    new as i32 - old as i32 <= 1
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid: Vec<Vec<Node>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| Node {
                    val: c,
                    visited: false,
                    x: 0,
                    y: 0,
                })
                .collect::<Vec<Node>>()
        })
        .collect();

    let mut start: (i32, i32) = (0, 0);
    let mut end: (i32, i32) = (0, 0);

    let width = grid[0].len();
    let height = grid.len();

    for y in 0..height {
        for x in 0..width {
            if grid[y][x].val == 'S' {
                start = (y as i32, x as i32);
                grid[y][x].val = 'a';
                grid[y][x].visited = true;
            }
            if grid[y][x].val == 'E' {
                end = (y as i32, x as i32);
                grid[y][x].val = 'z';
            }
            grid[y][x].x = x;
            grid[y][x].y = y;
        }
    }
    let mut queue: VecDeque<(Node, u32)> = VecDeque::new();
    queue.push_back((grid[start.0 as usize][start.1 as usize], 0));

    while !queue.is_empty() {
        let (curr, steps) = queue.pop_front().unwrap();
        if (curr.y as i32, curr.x as i32) == end {
            return Some(steps);
        }

        for (y, x) in DIRECTIONS {
            let new_val: (i32, i32) = (y + curr.y as i32, x + curr.x as i32);

            if !is_node_valid(new_val.1, new_val.0, width, height, &grid) {
                continue;
            }
            if !is_val_valid(curr.val, grid[new_val.0 as usize][new_val.1 as usize].val) {
                continue;
            }

            queue.push_back((grid[new_val.0 as usize][new_val.1 as usize], steps + 1));
            grid[new_val.0 as usize][new_val.1 as usize].visited = true;
        }
    }

    None
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut grid: Vec<Vec<Node>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| Node {
                    val: c,
                    visited: false,
                    x: 0,
                    y: 0,
                })
                .collect::<Vec<Node>>()
        })
        .collect();

    let mut starts: Vec<(i32, i32)> = Vec::new();
    let mut end: (i32, i32) = (0, 0);

    let width = grid[0].len();
    let height = grid.len();

    for y in 0..height {
        for x in 0..width {
            if grid[y][x].val == 'S' || grid[y][x].val == 'a' {
                starts.push((y as i32, x as i32));
                grid[y][x].val = 'a';
                grid[y][x].visited = true;
            }
            if grid[y][x].val == 'E' {
                end = (y as i32, x as i32);
                grid[y][x].val = 'z';
            }
            grid[y][x].x = x;
            grid[y][x].y = y;
        }
    }

    let mut min: u32 = u32::MAX;
    for start in starts {
        let mut queue: VecDeque<(Node, u32)> = VecDeque::new();
        queue.push_back((grid[start.0 as usize][start.1 as usize], 0));

        for y in 0..height {
            for x in 0..width {
                grid[y][x].visited = false;
            }
        }

        while !queue.is_empty() {
            let (curr, steps) = queue.pop_front().unwrap();
            if (curr.y as i32, curr.x as i32) == end {
                if steps < min {
                    min = steps;
                    break;
                }
            }

            for (y, x) in DIRECTIONS {
                let new_val: (i32, i32) = (y + curr.y as i32, x + curr.x as i32);

                if !is_node_valid(new_val.1, new_val.0, width, height, &grid) {
                    continue;
                }
                if !is_val_valid(curr.val, grid[new_val.0 as usize][new_val.1 as usize].val) {
                    continue;
                }

                queue.push_back((grid[new_val.0 as usize][new_val.1 as usize], steps + 1));
                grid[new_val.0 as usize][new_val.1 as usize].visited = true;
            }
        }
    }

    Some(min)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(31));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), Some(29));
    }
}
