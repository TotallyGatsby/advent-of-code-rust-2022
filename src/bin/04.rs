use advent_of_code::helpers::split_to_ints;

pub fn part_one(input: &str) -> Option<u32> {
    input.lines().try_fold(0, |score, line| {
        // Example input is 3-5,4-9
        // So we split out some vars, first by the comma to get (3-5, 4-9)
        let (elf1, elf2) = line.split_once(",")?;
        let (elf1_min, elf1_max) = split_to_ints(elf1, "-")?;
        let (elf2_min, elf2_max) = split_to_ints(elf2, "-")?;

        // Then into individual values (3,5), (4,9)

        // If both elves share one boundary, one must be the subset of the other
        if elf1_min == elf2_min || elf1_max == elf2_max {
            return Some(score + 1);
        }

        // Store which elf holds the min, and which holds the max
        let min_elf = if elf1_min <= elf2_min { 1 } else { 2 };
        let max_elf = if elf1_max >= elf2_max { 1 } else { 2 };

        // If one elf has the min and the max, it has a range that encompases the other elf
        if min_elf == max_elf {
            return Some(score + 1);
        }

        // If any input is invalid or if there's not a match, do not increment the accumulator
        Some(score)
    })
}

pub fn part_two(input: &str) -> Option<u32> {
    input.lines().try_fold(0, |score, line| {
        // Example input is 3-5,4-9
        // So we split out some vars, first by the comma to get (3-5, 4-9)
        let (elf1, elf2) = line.split_once(",")?;
        let (elf1_min, elf1_max) = split_to_ints(elf1, "-")?;
        let (elf2_min, elf2_max) = split_to_ints(elf2, "-")?;

        if elf1_min == elf2_min
            || elf1_max == elf2_max
            || (elf1_max >= elf2_min && elf1_min <= elf2_max)
        {
            return Some(score + 1);
        }
        Some(score)
    })
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(4));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(6));
    }
}
