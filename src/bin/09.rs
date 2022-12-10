use std::collections::HashSet;

use advent_of_code::helpers::normalize;
use itertools::Itertools;

#[derive(Clone)]
struct Agent {
    pub pos: (i32, i32),
}

impl Agent {
    pub fn new() -> Self {
        Self { pos: (0, 0) }
    }

    pub fn move_agent(&mut self, dir: &str) {
        match dir {
            "U" => {
                self.pos.1 += 1;
            }
            "R" => {
                self.pos.0 += 1;
            }
            "D" => {
                self.pos.1 -= 1;
            }
            "L" => {
                self.pos.0 -= 1;
            }
            _ => return,
        }
    }

    pub fn follow_agent(&mut self, agent: &Agent) {
        let x_dist = agent.pos.0 - self.pos.0;
        let y_dist = agent.pos.1 - self.pos.1;

        if y_dist == 0 {
            if x_dist > 1 {
                self.pos.0 += 1;
            } else if x_dist < -1 {
                self.pos.0 -= 1;
            }
        } else if x_dist == 0 {
            if y_dist > 1 {
                self.pos.1 += 1;
            } else if y_dist < -1 {
                self.pos.1 -= 1;
            }
        } else if x_dist.abs() > 1 || y_dist.abs() > 1 {
            self.pos.0 += normalize(x_dist);
            self.pos.1 += normalize(y_dist);
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut head = Agent::new();
    let mut tail = Agent::new();

    let mut path: HashSet<(i32, i32)> = HashSet::new();

    for line in input.lines() {
        let tokens = line.split_whitespace().collect_vec();
        for _ in 0..tokens[1].parse::<usize>().unwrap() {
            head.move_agent(tokens[0]);
            tail.follow_agent(&head);
            path.insert(tail.pos.clone());
        }
    }
    Some(path.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut snake_bits = vec![Agent { pos: (0, 0) }; 10];
    let mut path: HashSet<(i32, i32)> = HashSet::new();

    for line in input.lines() {
        let tokens = line.split_whitespace().collect_vec();
        for _ in 0..tokens[1].parse::<usize>().unwrap() {
            snake_bits[0].move_agent(tokens[0]);

            for j in 1..10 {
                let prev = &snake_bits[j - 1].clone();
                snake_bits[j].follow_agent(prev);
            }
            path.insert(snake_bits[9].pos.clone());
        }
    }

    Some(path.len() as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(1));
    }
}
