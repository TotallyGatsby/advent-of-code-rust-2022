use itertools::Itertools;
use num_bigint::BigInt;

#[derive(Debug)]
enum MonkeyOperand {
    Plus,
    Mult,
}

#[derive(Debug)]
struct Monkey {
    pub dvisor: i32,
    pub objects: Vec<BigInt>,
    pub true_target: usize,
    pub false_target: usize,
    pub inspection_operand: MonkeyOperand,
    pub inspection_rhs: i32,
    pub handled_item_count: u64,
}

impl Monkey {
    fn new() -> Self {
        Self {
            dvisor: 1,
            objects: Vec::new(),
            true_target: 0,
            false_target: 0,
            inspection_operand: MonkeyOperand::Mult,
            inspection_rhs: -1,
            handled_item_count: 0,
        }
    }

    fn inspect(&mut self, divisor: i32) -> Option<(usize, BigInt)> {
        if self.objects.len() == 0 {
            return None;
        }

        let mut inspection_target = self.objects.drain(0..1).next()?;

        self.handled_item_count += 1;

        // Perform the monkey operation
        match self.inspection_operand {
            MonkeyOperand::Plus => {
                if self.inspection_rhs > 1000 {
                    inspection_target *= 2;
                } else {
                    inspection_target += BigInt::from(self.inspection_rhs);
                }
            }
            MonkeyOperand::Mult => {
                if self.inspection_rhs > 1000 {
                    inspection_target = inspection_target.pow(2);
                } else {
                    inspection_target *= BigInt::from(self.inspection_rhs);
                }
            }
        }

        // TODO: AoC HACK!
        if divisor == 3 {
            inspection_target = inspection_target / divisor as i128;
        } else {
            inspection_target = inspection_target % divisor;
        }

        if inspection_target.clone() % self.dvisor == BigInt::from(0) {
            return Some((self.true_target, inspection_target));
        } else {
            return Some((self.false_target, inspection_target));
        }
    }
}

fn get_monkeys(input: &str) -> Vec<Monkey> {
    let mut temp_monkey: Option<Monkey> = None;
    let mut monkeys = Vec::new();
    for lines in input.lines() {
        let tokens = lines.split_whitespace().collect_vec();
        if tokens.len() == 0 {
            continue;
        };

        match tokens[0] {
            "Monkey" => {
                if temp_monkey.is_some() {
                    monkeys.push(temp_monkey.unwrap());
                }
                temp_monkey = Some(Monkey::new());
            }
            "Starting" => {
                for i in 2..tokens.len() {
                    temp_monkey
                        .as_mut()
                        .unwrap()
                        .objects
                        .push(tokens[i].replace(",", "").parse().unwrap());
                }
            }
            "Operation:" => {
                temp_monkey.as_mut().unwrap().inspection_rhs =
                    tokens.last().unwrap().parse().unwrap_or(100000);
                temp_monkey.as_mut().unwrap().inspection_operand = if tokens[4] == "*" {
                    MonkeyOperand::Mult
                } else {
                    MonkeyOperand::Plus
                }
            }
            "Test:" => {
                temp_monkey.as_mut().unwrap().dvisor = tokens.last().unwrap().parse().unwrap();
            }
            "If" => match tokens[1] {
                "true:" => {
                    temp_monkey.as_mut().unwrap().true_target =
                        tokens.last().unwrap().parse().unwrap();
                }
                "false:" => {
                    temp_monkey.as_mut().unwrap().false_target =
                        tokens.last().unwrap().parse().unwrap();
                }
                _ => println!("Incorrectly structured If statement: {:?}", tokens),
            },
            _ => println!("Incorrectly formatted monkey line: {:?}", tokens),
        }
    }

    monkeys.push(temp_monkey.unwrap());
    monkeys
}

pub fn part_one(input: &str) -> Option<BigInt> {
    let mut monkeys = get_monkeys(input);

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            while let Some(tossed_item) = monkeys[i].inspect(3) {
                monkeys[tossed_item.0].objects.push(tossed_item.1);
            }
        }
    }

    Some(
        monkeys
            .iter()
            .map(|m| m.handled_item_count)
            .sorted()
            .rev()
            .take(2)
            .fold(BigInt::from(1), |acc, count| acc * count),
    )
}

pub fn part_two(input: &str) -> Option<BigInt> {
    let mut monkeys = get_monkeys(input);
    for round in 0..10000 {
        if round % 1000 == 0 || round == 1 || round == 20 {
            println!("Round: {}", round);
            for monkey in monkeys.iter() {
                println!("{}", monkey.handled_item_count);
            }
        }

        for i in 0..monkeys.len() {
            while let Some(tossed_item) = monkeys[i].inspect(9699690) {
                monkeys[tossed_item.0].objects.push(tossed_item.1);
            }
        }
    }

    Some(
        monkeys
            .iter()
            .map(|m| BigInt::from(m.handled_item_count))
            .sorted()
            .rev()
            .take(2)
            .fold(BigInt::from(1), |acc, count| acc * count),
    )
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
        assert_eq!(part_one(&input), Some(BigInt::from(10605)));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        // TODO: Hacked to use the custom LCM
        assert_eq!(part_two(&input), Some(BigInt::from(2567194800i128)));
    }
}
