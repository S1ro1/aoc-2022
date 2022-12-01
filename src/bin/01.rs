pub fn part_one(input: &str) -> Option<u32> {
    let kek = input
        .split("\n\n")
        .map(|elf| elf.split('\n').map(|num| num.parse::<u32>().unwrap_or(0)))
        .map(|line| line.sum())
        .max();
    
    println!("{:?}", kek);

    kek
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut arr: Vec<u32> = input
        .split("\n\n")
        .map(|elf| elf.split('\n').map(|num| num.parse().unwrap_or(0)))
        .map(|line| line.sum())
        .collect();

    arr.sort();

    arr.windows(3).map(|c| c.into_iter().sum()).max()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
