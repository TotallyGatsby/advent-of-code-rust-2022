use itertools::Itertools;
use json::JsonValue;

fn compare(lhs: &JsonValue, rhs: &JsonValue) -> i32 {
    if lhs.is_number() && rhs.is_number() {
        return (lhs.as_i32().unwrap() - rhs.as_i32().unwrap()).signum();
    } else if lhs.is_number() && rhs.is_array() {
        return compare(&JsonValue::Array(vec![lhs.clone()]), rhs);
    } else if lhs.is_array() && rhs.is_number() {
        return compare(lhs, &JsonValue::Array(vec![rhs.clone()]));
    } else if lhs.is_array() && rhs.is_array() {
        for (l, r) in lhs.members().zip(rhs.members()) {
            let result = compare(l, r);
            if result != 0 {
                return result;
            }
        }
        return (lhs.len() as i32 - rhs.len() as i32).signum();
    }

    9999
}
pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines().collect_vec();

    let mut iter = lines.iter();
    let mut score = 0;
    for i in 0..input.lines().step_by(3).count() {
        let left = json::parse(iter.next().unwrap()).unwrap();
        let right = json::parse(iter.next().unwrap()).unwrap();
        iter.next();

        if compare(&left, &right) == -1 {
            score += i as u32 + 1;
        }
    }

    Some(score)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut signals = Vec::new();

    signals.push(json::parse("[[2]]").unwrap());
    signals.push(json::parse("[[6]]").unwrap());

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        signals.push(json::parse(line).unwrap());
    }

    signals.sort_by(|l, r| compare(l, r).cmp(&0));

    let mut score = 1;
    for (i, signal) in signals.iter().enumerate() {
        if signal.to_string() == "[[2]]" || signal.to_string() == "[[6]]" {
            score *= i as u32 + 1;
        }
    }
    Some(score)
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
