fn parse_blocks<'a>(lines: impl Iterator<Item = &'a str>) -> Option<Vec<Vec<char>>> {
    // Iterate over the lines and extract the characters at the corresponding
    // column index, adding them to the corresponding stack
    let mut stacks: Vec<Vec<char>> = Vec::new();
    for _ in 0..9 {
        let stack: Vec<char> = Vec::new();
        stacks.push(stack);
    }

    for line in lines {
        for (stack_id, col) in (1..36).step_by(4).enumerate() {
            let stack = &mut stacks[stack_id];
            if line.len() > col {
                // Check if the column is empty
                let ch = line.chars().nth(col);
                if ch.is_some() && ch.unwrap() != ' ' {
                    stack.push(ch.unwrap());
                }
            }
        }
    }

    for i in 0..9 {
        stacks[i].reverse();
    }

    Some(stacks)
}

#[derive(Debug)]
struct MoveInstruction {
    amount: usize,
    from_column: usize,
    to_column: usize,
}

impl MoveInstruction {
    pub fn new(line: &str) -> Self {
        let mut tokens = line.split_whitespace().filter_map(|s| s.parse().ok());

        Self {
            amount: tokens.next().unwrap(),
            from_column: tokens.next().unwrap() - 1,
            to_column: tokens.next().unwrap() - 1,
        }
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let (blocks_str, instructions_str) = input.split_once("\n\n").unwrap();

    let mut block_lines = blocks_str
        .lines()
        .into_iter()
        .filter(|line| line.contains('['));
    let mut blocks = parse_blocks(block_lines.by_ref()).unwrap();

    let mut instructions: Vec<MoveInstruction> = Vec::new();

    for line in instructions_str.lines() {
        instructions.push(MoveInstruction::new(line));
    }

    for inst in instructions {
        for _ in 0..inst.amount {
            let temp = blocks[inst.from_column].pop().unwrap();
            blocks[inst.to_column].push(temp);
        }
    }

    let result: String = blocks
        .iter()
        .map(|b| b.last().unwrap_or(&' '))
        .collect::<String>()
        .trim()
        .to_string();

    Some(result)
}

pub fn part_two(input: &str) -> Option<String> {
    let (blocks_str, instructions_str) = input.split_once("\n\n").unwrap();

    let mut block_lines = blocks_str
        .lines()
        .into_iter()
        .filter(|line| line.contains('['));
    let mut blocks = parse_blocks(block_lines.by_ref()).unwrap();

    let mut instructions: Vec<MoveInstruction> = Vec::new();

    for line in instructions_str.lines() {
        instructions.push(MoveInstruction::new(line));
    }

    for inst in instructions {
        let mut temp_stack = Vec::new();
        for _ in 0..inst.amount {
            temp_stack.push(blocks[inst.from_column].pop().unwrap());
        }
        for _ in 0..inst.amount {
            blocks[inst.to_column].push(temp_stack.pop().unwrap());
        }
    }

    let result: String = blocks
        .iter()
        .map(|b| b.last().unwrap_or(&' '))
        .collect::<String>()
        .trim()
        .to_string();

    Some(result)
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
        assert_eq!(part_one(&input), Some("CMZ".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".to_string()));
    }
}
