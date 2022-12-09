use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
    let mut moves: Vec<((i32, i32), u32)> = Vec::new();

    for line in input.lines() {
        let (direction, distance) = line.split_once(' ').unwrap();
        let distance: u32 = distance.parse().unwrap();

        match direction {
            "L" => moves.push(((-1, 0), distance)),
            "R" => moves.push(((1, 0), distance)),
            "U" => moves.push(((0, 1), distance)),
            "D" => moves.push(((0, -1), distance)),
            _ => (),
        }
    }
    let mut xs: Vec<i32> = vec![0; 2];
    let mut ys: Vec<i32> = vec![0; 2];

    let mut visited: Vec<(i32, i32)> = vec![(xs[xs.len() - 1], ys[ys.len() - 1])];

    for ((cx, cy), distance) in moves {
        for _ in 0..distance {
            xs[0] += cx;
            ys[0] += cy;

            for i in 0..1 {
                let dx = xs[i + 1] - xs[i];
                let dy = ys[i + 1] - ys[i];

                if dx.abs() == 2 || dy.abs() == 2 {
                    xs[i + 1] = xs[i] + dx / 2;
                    ys[i + 1] = ys[i] + dy / 2;
                }
                visited.push((xs[xs.len() - 1], ys[ys.len() - 1]));
            }
        }
    }

    Some(HashSet::<&(i32, i32)>::from_iter(visited.iter()).len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut moves: Vec<((i32, i32), u32)> = Vec::new();

    for line in input.lines() {
        let (direction, distance) = line.split_once(' ').unwrap();
        let distance: u32 = distance.parse().unwrap();

        match direction {
            "L" => moves.push(((-1, 0), distance)),
            "R" => moves.push(((1, 0), distance)),
            "U" => moves.push(((0, 1), distance)),
            "D" => moves.push(((0, -1), distance)),
            _ => (),
        }
    }
    let mut xs: Vec<i32> = vec![0; 10];
    let mut ys: Vec<i32> = vec![0; 10];

    let mut visited: Vec<(i32, i32)> = vec![(xs[xs.len() - 1], ys[ys.len() - 1])];

    for ((cx, cy), distance) in moves {
        for _ in 0..distance {
            xs[0] += cx;
            ys[0] += cy;

            for i in 0..9 {
                let dx = xs[i + 1] - xs[i];
                let dy = ys[i + 1] - ys[i];

                if dx.abs() == 2 || dy.abs() == 2 {
                    xs[i + 1] = xs[i] + dx / 2;
                    ys[i + 1] = ys[i] + dy / 2;
                }
                visited.push((xs[xs.len() - 1], ys[ys.len() - 1]));
            }
        }
    }

    Some(HashSet::<&(i32, i32)>::from_iter(visited.iter()).len() as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), None);
    }
}
