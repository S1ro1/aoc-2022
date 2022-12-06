use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
    let count: u32 = 4;
    Some(
        input
            .chars()
            .collect::<Vec<char>>()
            .windows(count as usize)
            .map(|window| {
                let ch: HashSet<&char> = HashSet::from_iter(window);
                if ch.len() == count as usize {
                    1
                } else {
                    0
                }
            })
            .position(|c| c == 1)
            .unwrap() as u32
            + count,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let count: u32 = 14;
    Some(
        input
            .chars()
            .collect::<Vec<char>>()
            .windows(count as usize)
            .map(|window| {
                let ch: HashSet<&char> = HashSet::from_iter(window);
                if ch.len() == count as usize {
                    1
                } else {
                    0
                }
            })
            .position(|c| c == 1)
            .unwrap() as u32
            + count,
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(19));
    }
}
