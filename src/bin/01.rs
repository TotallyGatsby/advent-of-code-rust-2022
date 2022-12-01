pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines();

    let mut largest_calories = 0u32;
    let mut current_calories = 0;

    for line in lines {
        if line.is_empty() {
            // Time to check the total calories for this elf
            if current_calories > largest_calories {
                largest_calories = current_calories;
            }
            current_calories = 0;
        } else {
            current_calories += line.parse::<u32>().unwrap();
        }
    }
    if current_calories > largest_calories {
        largest_calories = current_calories;
    }

    Some(largest_calories)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines();

    let mut calories = Vec::new();
    let mut current_calories = 0;

    for line in lines {
        if line.is_empty() {
            // Time to check the total calories for this elf
            calories.push(current_calories);
            current_calories = 0;
        } else {
            current_calories += line.parse::<u32>().unwrap();
        }
    }

    calories.push(current_calories);

    calories.sort_by(|a, b| b.cmp(a));

    Some(calories[0] + calories[1] + calories[2])
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
