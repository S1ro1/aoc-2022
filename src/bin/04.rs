use std::ops::Deref;

use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let mut line_nums = line
                    .split([',', '-'])
                    .map(|num| num.parse::<u32>().unwrap());
                let first = line_nums.next().unwrap();
                let second = line_nums.next().unwrap();
                let third = line_nums.next().unwrap();
                let fourth = line_nums.next().unwrap();

                ((first >= third && second <= fourth) || (third >= first && fourth <= second))
                    as u32
            })
            .sum::<u32>(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let mut line_nums = line
                    .split([',', '-'])
                    .map(|num| num.parse::<u32>().unwrap());
                let first = line_nums.next().unwrap();
                let second = line_nums.next().unwrap();
                let third = line_nums.next().unwrap();
                let fourth = line_nums.next().unwrap();

                (((first <= third) && (second >= third)) || ((first >= third) && (fourth >= first)))
                    as u32
            })
            .sum::<u32>(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
