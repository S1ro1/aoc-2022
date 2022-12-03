use itertools::Itertools;
use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
    let mut val: u32 = 0;
    for current in input.lines() {
        let middle = current.len() / 2;

        let first_compartment: HashSet<char> = current[..middle].chars().collect();
        let second_compartment: HashSet<char> = current[middle..].chars().collect();

        let shared = first_compartment
            .intersection(&second_compartment)
            .nth(0)
            .unwrap();
        val += match shared {
            'a'..='z' => *shared as u32 - 96,
            'A'..='Z' => *shared as u32 - 38,
            _ => 0,
        }
    }
    Some(val)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut val: u32 = 0;
    let chunks = input.lines().chunks(3);

    for chunk in chunks.into_iter() {
        let maps = chunk
            .map(|line| line.chars().collect::<HashSet<char>>())
            .collect::<Vec<HashSet<char>>>();

        let shared = maps[0]
            .iter()
            .filter(|b| maps[1..].iter().all(|set| set.contains(b)))
            .nth(0)
            .unwrap();

        val += match shared {
            'a'..='z' => *shared as u32 - 96,
            'A'..='Z' => *shared as u32 - 38,
            _ => 0,
        }
    }

    Some(val)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
