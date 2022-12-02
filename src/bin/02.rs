pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines().fold(0u32, |score, line| match line {
        "A X" => score + 4,
        "A Y" => score + 8,
        "A Z" => score + 3,
        "B X" => score + 1,
        "B Y" => score + 5,
        "B Z" => score + 9,
        "C X" => score + 7,
        "C Y" => score + 2,
        "C Z" => score + 6,
        &_ => {
            println!("Unknown result: {}", line);
            score
        }
    }))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(input.lines().fold(0u32, |score, line| match line {
        "A X" => score + 3,
        "A Y" => score + 4,
        "A Z" => score + 8,
        "B X" => score + 1,
        "B Y" => score + 5,
        "B Z" => score + 9,
        "C X" => score + 2,
        "C Y" => score + 6,
        "C Z" => score + 7,
        &_ => {
            println!("Unknown result: {}", line);
            score
        }
    }))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
