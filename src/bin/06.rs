use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        input
            .chars()
            .enumerate()
            .collect::<Vec<(usize, char)>>()
            .windows(4)
            .find(|chars| {
                let mut char_set: HashSet<char> = HashSet::new();

                chars.iter().all(|c| char_set.insert(c.1))
            })
            .unwrap()
            .last()
            .unwrap()
            .0
            + 1,
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(
        input
            .chars()
            .enumerate()
            .collect::<Vec<(usize, char)>>()
            .windows(14)
            .find(|chars| {
                let mut char_set: HashSet<char> = HashSet::new();

                chars.iter().all(|c| char_set.insert(c.1))
            })
            .unwrap()
            .last()
            .unwrap()
            .0
            + 1,
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(19));
    }
}
