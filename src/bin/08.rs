pub fn part_one(input: &str) -> Option<u32> {
    let vals: Vec<Vec<u32>> = input
        .lines()
        .map(|l| l.chars().flat_map(|x| x.to_digit(10)).collect::<Vec<u32>>())
        .collect();

    let width: usize = vals[0].len();
    let height: usize = vals.len();

    let mut count: u32 = 0;

    for y in 0..height {
        for x in 0..width {
            let curr: u32 = vals[y][x];
            let mut highest_count: u32 = 4;

            // from the left
            for x_curr in 0..x {
                if vals[y][x_curr] >= curr {
                    highest_count -= 1;
                    break;
                }
            }

            // to the right
            for x_curr in x + 1..width {
                if vals[y][x_curr] >= curr {
                    highest_count -= 1;
                    break;
                }
            }

            // from the top
            for y_curr in 0..y {
                if vals[y_curr][x] >= curr {
                    highest_count -= 1;
                    break;
                }
            }

            // to the bottom
            for y_curr in y + 1..height {
                if vals[y_curr][x] >= curr {
                    highest_count -= 1;
                    break;
                }
            }
            if highest_count > 0 {
                count += 1;
            }
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let vals: Vec<Vec<u32>> = input
        .lines()
        .map(|l| l.chars().flat_map(|x| x.to_digit(10)).collect::<Vec<u32>>())
        .collect();

    let width: usize = vals[0].len();
    let height: usize = vals.len();

    let mut max: u32 = 0;

    for y in 0..height {
        for x in 0..width {
            let curr: u32 = vals[y][x];

            let mut x1: u32 = x as u32;

            // from the left
            for x_curr in 0..x {
                if vals[y][x_curr] >= curr {
                    x1 = (x - x_curr) as u32;
                }
            }

            // to the right
            let mut x2: u32 = (width - x - 1) as u32;
            for x_curr in x + 1..width {
                if vals[y][x_curr] >= curr {
                    x2 = (x_curr - x) as u32;
                    break;
                }
            }

            // from the top
            let mut y1: u32 = y as u32;
            for y_curr in 0..y {
                if vals[y_curr][x] >= curr {
                    y1 = (y - y_curr) as u32;
                }
            }

            // to the bottom
            let mut y2: u32 = (height - y - 1) as u32;
            for y_curr in y + 1..height {
                if vals[y_curr][x] >= curr {
                    y2 = (y_curr - y) as u32;
                    break;
                }
            }

            let highest: u32 = x1 * x2 * y1 * y2;
            if highest > max {
                max = highest;
            }
        }
    }
    Some(max)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
