use itertools::Itertools;

struct Op {
    pub opcode: String,
    pub cycles: i32,
    pub operand: i32,
}

impl Op {
    pub fn tick(&mut self) -> bool {
        self.cycles -= 1;

        return if self.cycles == 0 { true } else { false };
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut cycle = 1;
    let mut score = 1;
    let mut signal = 0;
    let mut operations = Vec::new();

    for line in input.lines() {
        let tokens = line.split_whitespace().collect_vec();
        operations.push(Op {
            opcode: tokens[0].to_string(),
            operand: if tokens.len() == 1 {
                0
            } else {
                tokens[1].parse::<i32>().unwrap_or(0)
            },
            cycles: if tokens[0] == "noop" { 1 } else { 2 },
        })
    }

    for mut operation in operations {
        loop {
            if cycle % 40 == 20 {
                signal += cycle * score;
            }
            cycle += 1;
            if operation.tick() {
                break;
            }
        }
        match operation.opcode.as_str() {
            "addx" => {
                score += operation.operand;
            }
            _ => continue,
        }
    }
    Some(signal as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut cycle = 1;
    let mut register_x = 1;
    let mut pixel = 0;
    let mut operations = Vec::new();

    for line in input.lines() {
        let tokens = line.split_whitespace().collect_vec();
        operations.push(Op {
            opcode: tokens[0].to_string(),
            operand: if tokens.len() == 1 {
                0
            } else {
                tokens[1].parse::<i32>().unwrap_or(0)
            },
            cycles: if tokens[0] == "noop" { 1 } else { 2 },
        })
    }

    for mut operation in operations {
        loop {
            if register_x - 1 <= pixel && pixel <= register_x + 1 {
                print!("#");
            } else {
                print!(".");
            }
            pixel += 1;
            if cycle % 40 == 0 {
                pixel = 0;
                print!("\n");
            }
            cycle += 1;

            if operation.tick() {
                break;
            }
        }
        match operation.opcode.as_str() {
            "addx" => {
                register_x += operation.operand;
            }
            _ => continue,
        }
    }
    Some(0)
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
        assert_eq!(part_two(&input), Some(0));
    }
}
