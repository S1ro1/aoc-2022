use nom::*;
use std::cmp::Ordering;

#[derive(PartialEq, Eq, Debug, Clone)]
enum Item {
    Item(u32),
    List(Vec<Item>),
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Item::Item(a), Item::Item(b)) => a.cmp(b),
            (Item::List(a), Item::List(b)) => match a.iter().cmp(b) {
                Ordering::Greater => Ordering::Greater,
                Ordering::Less => Ordering::Less,
                _ => a.len().cmp(&b.len()),
            },
            (Item::Item(_), Item::List(b)) if b.len() == 1 => self.cmp(&b[0]),
            (Item::Item(a), Item::List(_)) => Item::List(vec![Item::Item(*a)]).cmp(other),
            (Item::List(_), Item::Item(_)) => other.cmp(self).reverse(),
        }
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .split("\n\n")
            .map(|p| pair(p.as_bytes()).unwrap().1)
            .enumerate()
            .filter(|(_, (a, b))| a.cmp(b) == Ordering::Less)
            .map(|(index, _)| index as u32 + 1)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let a = Item::List(vec![Item::List(vec![Item::Item(2)])]);
    let b = Item::List(vec![Item::List(vec![Item::Item(6)])]);

    let items: Vec<Item> = input
        .lines()
        .filter(|line| line.len() != 0)
        .map(|line| item(line.as_bytes()).unwrap().1)
        .filter(|i| i < &b)
        .collect();

    Some(((items.iter().filter(|i| *i < &a).count() + 1) * (items.len() + 2)) as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 13);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_two(&input), Some(140));
    }
}

named!(item<&[u8], Item>, alt!(map!(list, Item::List) | map!(num, Item::Item)));
named!(num<&[u8], u32>, map_opt!(nom::character::complete::digit1, atoi::atoi));
named!(list<&[u8], Vec<Item>>, delimited!(char!('['), separated_list0!(char!(','), item), char!(']')));

named!(pair<&[u8], (Item, Item)>, separated_pair!(item, tag!("\n"), item));
