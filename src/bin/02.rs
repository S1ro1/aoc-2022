pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                match line.as_bytes()[2] as char {
                    'X' => match line.as_bytes()[0] as char {
                        'A' => 4,
                        'B' => 1,
                        'C' => 7,
                        _ => 0,
                    },
                    'Y' => match line.as_bytes()[0] as char {
                        'A' => 8,
                        'B' => 5,
                        'C' => 2,
                        _ => 0,
                    },
                    'Z' => match line.as_bytes()[0] as char {
                        'A' => 3,
                        'B' => 9,
                        'C' => 6,
                        _ => 0,
                    },
                    _ => 0,
                }
            } as u32)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                match line.as_bytes()[2] as char {
                    'X' => match line.as_bytes()[0] as char {
                        'A' => 3,
                        'B' => 1,
                        'C' => 2,
                        _ => 0,
                    },
                    'Y' => match line.as_bytes()[0] as char {
                        'A' => 4,
                        'B' => 5,
                        'C' => 6,
                        _ => 0,
                    },
                    'Z' => match line.as_bytes()[0] as char {
                        'A' => 8,
                        'B' => 9,
                        'C' => 7,
                        _ => 0,
                    },
                    _ => 0,
                }
            } as u32)
            .sum(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
