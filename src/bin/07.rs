use itertools::Itertools;
use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.lines().collect::<Vec<&str>>();
    let mut dirs: Vec<String> = Vec::new();
    let mut hashed_dirs: HashMap<String, u32> = HashMap::new();

    for line in lines {
        let words: Vec<&str> = line.split(' ').collect_vec();
        if words[0] == "$" {
            if words[1] == "ls" {
                continue;
            } else {
                if words[2] == ".." {
                    let _ = dirs.pop();
                } else {
                    dirs.push(String::from(words[2]));
                }
            }
        } else if words[0] == "dir" {
            continue;
        } else {
            let num: u32 = line
                .split(' ')
                .flat_map(|c| c.parse::<u32>())
                .nth(0)
                .unwrap();
            for path in dirs.iter() {
                let entry = String::from(path);
                *hashed_dirs.entry(entry).or_insert(0) += num;
            }
        }
    }

    Some(hashed_dirs.values().filter(|&&c| c <= 100000).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.lines().collect::<Vec<&str>>();
    let mut dirs: Vec<String> = Vec::new();
    let mut hashed_dirs: HashMap<String, u32> = HashMap::new();

    for line in lines {
        let words: Vec<&str> = line.split(' ').collect_vec();
        if words[0] == "$" {
            if words[1] == "ls" {
                continue;
            } else {
                if words[2] == ".." {
                    let _ = dirs.pop();
                } else {
                    dirs.push(String::from(words[2]));
                }
            }
        } else if words[0] == "dir" {
            continue;
        } else {
            let num: u32 = line
                .split(' ')
                .flat_map(|c| c.parse::<u32>())
                .nth(0)
                .unwrap();
            for path in dirs.iter() {
                let entry = String::from(path);
                *hashed_dirs.entry(entry).or_insert(0) += num;
            }
        }
    }

    let max: u32 = *hashed_dirs.get("/").unwrap();
    let rest: u32 = 70000000 - max;

    Some(
        *hashed_dirs
            .values()
            .sorted()
            .filter(|&&val| rest + val >= 30000000)
            .into_iter()
            .nth(0)
            .unwrap(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
