use std::cmp::Reverse;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
struct Monkey {
    items: VecDeque<u128>,
    worries: u128,
    operation: Operation,
    test: Test,
}

#[derive(Debug, Clone)]
struct Operation {
    operator: String,
    operand: Operand,
}

#[derive(Debug, Clone)]
enum Operand {
    Num(u128),
    Prev,
}

#[derive(Debug, Clone)]
struct Test {
    if_true: usize,
    if_false: usize,
    divisor: u128,
}

impl From<&str> for Monkey {
    fn from(monkey: &str) -> Self {
        let mut line = monkey.split('\n');
        let items: VecDeque<u128> = line
            .nth(1)
            .unwrap()
            .split(':')
            .nth(1)
            .unwrap()
            .split(", ")
            .map(|num| num.replace(' ', "").parse::<u128>().unwrap())
            .collect::<VecDeque<u128>>();

        let mut operation = line.next().unwrap().split(' ');
        let operator = String::from(operation.nth(6).unwrap());
        let operand: Operand = match operation.next().unwrap() {
            "old" => Operand::Prev,
            v => Operand::Num(v.parse().unwrap()),
        };

        let divisor = line
            .next()
            .unwrap()
            .split(' ')
            .nth(5)
            .unwrap()
            .parse::<u128>()
            .unwrap();

        let if_true = line
            .next()
            .unwrap()
            .split(' ')
            .nth(9)
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let if_false = line
            .next()
            .unwrap()
            .split(' ')
            .nth(9)
            .unwrap()
            .parse::<usize>()
            .unwrap();

        Monkey {
            items,
            worries: 0,
            operation: Operation { operator, operand },
            test: Test {
                if_true,
                if_false,
                divisor,
            },
        }
    }
}

pub fn part_one(input: &str) -> Option<u128> {
    let monkeys = input.split("\n\n");

    let mut monkeys: Vec<Monkey> = monkeys.map(|line| Monkey::from(line)).collect();

    for _ in 0..20 {
        for monkey_index in 0..monkeys.len() {
            loop {
                let mut result = 0;
                let mut monkey = monkeys[monkey_index].clone();
                let item = monkey.items.pop_front();
                match item {
                    Some(item) => {
                        let operand = match monkey.operation.operand {
                            Operand::Prev => item,
                            Operand::Num(x) => x,
                        };

                        if monkey.operation.operator == String::from("*") {
                            result = (operand * item) / 3;
                        } else {
                            result = (operand + item) / 3;
                        }
                        monkey.worries += 1;
                    }
                    None => break,
                }
                if result % monkey.test.divisor == 0 {
                    monkeys[monkey.test.if_true].items.push_back(result);
                } else {
                    monkeys[monkey.test.if_false].items.push_back(result);
                }

                monkeys[monkey_index] = monkey;
            }
        }
    }

    monkeys.sort_by_key(|x| Reverse(x.worries));

    Some(monkeys[0].worries * monkeys[1].worries)
}

pub fn part_two(input: &str) -> Option<u128> {
    let monkeys = input.split("\n\n");

    let mut monkeys: Vec<Monkey> = monkeys.map(|line| Monkey::from(line)).collect();

    let common: u128 = monkeys.iter().map(|monkey| monkey.test.divisor).product();

    for _ in 0..10000 {
        for monkey_index in 0..monkeys.len() {
            loop {
                let result;
                let mut monkey = monkeys[monkey_index].clone();
                let item = monkey.items.pop_front();
                match item {
                    Some(item) => {
                        let operand = match monkey.operation.operand {
                            Operand::Prev => item,
                            Operand::Num(x) => x,
                        };

                        if monkey.operation.operator == String::from("*") {
                            result = (operand * item) % common;
                        } else {
                            result = (operand + item) % common;
                        }
                        monkey.worries += 1;
                    }
                    None => break,
                }
                if result % monkey.test.divisor == 0 {
                    monkeys[monkey.test.if_true].items.push_back(result);
                } else {
                    monkeys[monkey.test.if_false].items.push_back(result);
                }

                monkeys[monkey_index] = monkey;
            }
        }
    }

    monkeys.sort_by_key(|x| Reverse(x.worries));

    Some(monkeys[0].worries * monkeys[1].worries)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), Some(2713310158));
    }
}
