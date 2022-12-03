use itertools::Itertools;
use std::collections::HashSet;

fn score_item(item: &char) -> u32 {
    if item.is_ascii_lowercase() {
        (*item as u32) - 96
    } else {
        (*item as u32) - 38
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines().fold(0, |score, line| {
        let (left, right) = line.split_at(line.len() / 2);

        let left_sack: HashSet<char> = left.chars().collect();
        let right_sack: HashSet<char> = right.chars().collect();

        let intersect = left_sack.intersection(&right_sack).next().unwrap();

        score_item(intersect) + score
    }))
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines();

    let mut score = 0;

    //while let Ok(elves) = lines.next_chunk::<3>() {
    for mut elves in &lines.chunks(3) {
        let hash_one: HashSet<char> = elves.next().unwrap().chars().collect();
        let hash_two: HashSet<char> = elves.next().unwrap().chars().collect();
        let mut hash_three: HashSet<char> = elves.next().unwrap().chars().collect();

        hash_three.retain(|k| hash_one.contains(k) && hash_two.contains(k));

        score += score_item(&hash_three.into_iter().next().unwrap());
    }

    Some(score)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
