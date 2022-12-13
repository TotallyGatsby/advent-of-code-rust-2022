use pathfinding::prelude::bfs;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

#[derive(Clone)]
struct Map {
    pub grid: Vec<Vec<u32>>,
    pub start_pos: Point,
    pub end_pos: Point,
}

impl Map {
    fn new(input: &str) -> Self {
        let width = input.lines().next().unwrap().len();
        let height = input.lines().collect::<Vec<_>>().len();

        let mut map = Map {
            grid: vec![vec![0; width]; height],
            start_pos: Point { x: 0, y: 0 },
            end_pos: Point { x: 0, y: 0 },
        };

        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                match c {
                    'S' => {
                        map.grid[y][x] = 1;
                        map.start_pos = Point { x, y }
                    }
                    'E' => {
                        map.grid[y][x] = 26;
                        map.end_pos = Point { x, y };
                    }
                    _ => {
                        map.grid[y][x] = c as u32 - 96;
                    }
                }
            }
        }

        map
    }

    fn print(self) {
        for y in 0..self.grid.len() {
            for x in 0..self.grid[0].len() {
                print!("{:0>2} ", self.grid[y][x]);
            }
            print!("\n");
        }
    }

    fn move_options(self, from: &Point) -> Vec<Point> {
        let mut options = Vec::new();

        //up
        if from.y > 0 && self.grid[from.y][from.x] >= self.grid[from.y - 1][from.x] - 1 {
            options.push(Point {
                x: from.x,
                y: from.y - 1,
            });
        }
        //left
        if from.x > 0 && self.grid[from.y][from.x] >= self.grid[from.y][from.x - 1] - 1 {
            options.push(Point {
                x: from.x - 1,
                y: from.y,
            });
        }
        //down
        if from.y < self.grid.len() - 1
            && self.grid[from.y][from.x] >= self.grid[from.y + 1][from.x] - 1
        {
            options.push(Point {
                x: from.x,
                y: from.y + 1,
            });
        }
        //right
        if from.x < self.grid[0].len() - 1
            && self.grid[from.y][from.x] >= self.grid[from.y][from.x + 1] - 1
        {
            options.push(Point {
                x: from.x + 1,
                y: from.y,
            });
        }

        options
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = Map::new(input);

    let result = bfs(
        &map.start_pos,
        |p| map.clone().move_options(p),
        |p| p.x == map.end_pos.x && p.y == map.end_pos.y,
    )
    .unwrap();

    Some(result.len() as u32 - 1)
}

pub fn part_two(input: &str) -> Option<u32> {
    // Let's just brute force it
    let map = Map::new(input);

    let mut shortest_path = u32::MAX;
    for y in 0..map.grid.len() {
        for x in 0..map.grid[0].len() {
            if map.grid[y][x] == 1 {
                let path = bfs(
                    &Point { x, y },
                    |p| map.clone().move_options(p),
                    |p| p.x == map.end_pos.x && p.y == map.end_pos.y,
                )
                .unwrap_or_default();

                if path.len() == 0 {
                    continue;
                }

                shortest_path = shortest_path.min(path.len() as u32);
            }
        }
    }

    Some(shortest_path - 1)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(31));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), Some(29));
    }
}
