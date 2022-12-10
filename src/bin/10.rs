pub fn part_one(input: &str) -> Option<i32> {
    let mut cycle: i32 = 0;
    let mut x: i32 = 1;
    let mut total: i32 = 0;

    for line in input.lines() {
        let parts: Vec<&str> = line.split(' ').collect();

        for _ in 0..parts.len() {
            cycle += 1;
            if cycle % 40 == 20 {
                total += x * cycle;
            }
        }

        if parts.len() == 2 {
            x += parts[1].parse::<i32>().unwrap();
        }
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<String> {
    let mut cycle: i32 = 0;
    let mut x: i32 = 1;

    let mut output: String = String::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split(' ').collect();

        for _ in 0..parts.len() {
            let check: i32 = ((cycle % 40) - x).abs();
            cycle += 1;
            if check < 2 {
                output.push('#');
            } else {
                output.push('.');
            }
            if cycle % 40 == 0 {
                output.push('\n');
            }
        }

        if parts.len() == 2 {
            x += parts[1].parse::<i32>().unwrap();
        }
    }
    Some(output)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(
            part_two(&input),
            Some(String::from(
                "##..##..##..##..##..##..##..##..##..##..\n\
                 ###...###...###...###...###...###...###.\n\
                 ####....####....####....####....####....\n\
                 #####.....#####.....#####.....#####.....\n\
                 ######......######......######......####\n\
                 #######.......#######.......#######.....\n"
            ))
        );
    }
}
