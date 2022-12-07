use std::collections::HashMap;

use itertools::Itertools;

fn get_disk_use(input: &str) -> HashMap<String, u32> {
    let mut disk_use: HashMap<String, u32> = HashMap::new();
    disk_use.insert("/".to_string(), 0);

    let mut dir_stack = Vec::new();
    dir_stack.push("/");

    for line in input.lines() {
        let tokens = line.split_whitespace().collect_vec();
        match tokens[0] {
            "$" => match tokens[1] {
                "ls" => continue,
                "cd" => match tokens[2] {
                    ".." => {
                        dir_stack.pop();
                    }
                    "/" => {
                        dir_stack.clear();
                        dir_stack.push("/");
                    }
                    _ => {
                        dir_stack.push(tokens[2]);
                        if !disk_use.contains_key(&dir_stack.join("/")) {
                            disk_use.insert(dir_stack.join("/"), 0);
                        }
                    }
                },
                _ => {
                    println!("Unknown command: {}", line);
                }
            },
            "dir" => continue,
            _ => {
                for i in 1..dir_stack.len() + 1 {
                    let dir = dir_stack.iter().take(i).join("/");
                    disk_use
                        .entry(dir)
                        .and_modify(|e| *e += tokens[0].parse::<u32>().unwrap());
                }
            }
        }
    }
    disk_use
}

pub fn part_one(input: &str) -> Option<u32> {
    let disk_use = get_disk_use(input);

    Some(disk_use.iter().fold(0, |size, dir| {
        if *dir.1 <= 100000u32 {
            return size + *dir.1;
        }
        size
    }))
}

pub fn part_two(input: &str) -> Option<u32> {
    let disk_use = get_disk_use(input);

    let free_space = 70000000 - disk_use["/"];

    println!("Free Space: {}", free_space);
    Some(
        *disk_use
            .iter()
            .filter(|dir| *dir.1 > (30000000 - free_space))
            .sorted()
            .last()
            .unwrap()
            .1,
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
