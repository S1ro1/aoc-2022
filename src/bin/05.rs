use std::collections::VecDeque;

pub fn part_one(input: &str) -> Option<String> {
    let (head, tail) = input.split_once("\n\n").unwrap();

    let mut stacks: Vec<VecDeque<char>> = vec![VecDeque::new(); 9];

    for line in head.lines() {
        let len = line.len();
        let mut curr: usize = 1;
        while curr < len {
            match line.chars().nth(curr) {
                None => (),
                Some(c) => {
                    if c.is_alphabetic() {
                        stacks[curr / 4].push_back(c);
                    } else {
                        ();
                    }
                }
            }
            curr += 4;
        }
    }

    for line in tail.lines() {
        let mut splits = line.split(' ').into_iter();

        let _ = splits.next().unwrap();
        let count = splits.next().unwrap().parse::<usize>().unwrap();
        let _ = splits.next().unwrap();
        let src = splits.next().unwrap().parse::<usize>().unwrap();
        let _ = splits.next().unwrap();
        let dst = splits.next().unwrap().parse::<usize>().unwrap();

        for _ in 0..count {
            let tmp = stacks[src - 1].pop_front().unwrap();
            stacks[dst - 1].push_front(tmp);
        }
    }
    Some(
        stacks
            .iter()
            .flat_map(|stack| stack.front())
            .collect::<String>(),
    )
}

pub fn part_two(input: &str) -> Option<String> {
    let (head, tail) = input.split_once("\n\n").unwrap();

    let mut stacks: Vec<VecDeque<char>> = vec![VecDeque::new(); 9];

    for line in head.lines() {
        let len = line.len();
        let mut curr: usize = 1;
        while curr < len {
            match line.chars().nth(curr) {
                None => (),
                Some(c) => {
                    if c.is_alphabetic() {
                        stacks[curr / 4].push_back(c);
                    } else {
                        ();
                    }
                }
            }
            curr += 4;
        }
    }

    for line in tail.lines() {
        let mut splits = line.split(' ').into_iter();

        let _ = splits.next().unwrap();
        let count = splits.next().unwrap().parse::<usize>().unwrap();
        let _ = splits.next().unwrap();
        let src = splits.next().unwrap().parse::<usize>().unwrap();
        let _ = splits.next().unwrap();
        let dst = splits.next().unwrap().parse::<usize>().unwrap();

        let mut tmp_deq: VecDeque<char> = VecDeque::new();

        for _ in 0..count {
            let tmp = stacks[src - 1].pop_front().unwrap();
            tmp_deq.push_back(tmp);
        }

        for _ in 0..count {
            let tmp = tmp_deq.pop_back().unwrap();
            stacks[dst - 1].push_front(tmp);
        }
    }
    Some(
        stacks
            .iter()
            .flat_map(|stack| stack.front())
            .collect::<String>(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some(String::from("CMZ")));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some(String::from("MCD")));
    }
}
