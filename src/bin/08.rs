#[allow(dead_code)]
fn print_vec(cells: &Vec<Vec<i32>>) {
    for row in cells {
        let row_str = row
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ");
        println!("{}", row_str);
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();

    let mut cells = vec![vec![0i32; width]; height];
    let mut from_left = vec![vec![0i32; width]; height];
    let mut from_top = vec![vec![0i32; width]; height];
    let mut from_right = vec![vec![0i32; width]; height];
    let mut from_bottom = vec![vec![0i32; width]; height];

    for (i, line) in input.lines().enumerate() {
        let row = cells.get_mut(i).unwrap();

        for (j, c) in line.chars().enumerate() {
            *(*row).get_mut(j).unwrap() = c.to_digit(10).unwrap() as i32;
        }
    }

    let mut prev_height;

    for (y, row) in cells.iter().enumerate() {
        prev_height = -1;
        for (x, cell) in row.iter().enumerate() {
            if *cell > prev_height {
                from_left[y][x] = 1;
                prev_height = *cell;
            }
        }
    }

    for (y, row) in cells.iter().enumerate() {
        prev_height = -1;
        for (x, cell) in row.iter().rev().enumerate() {
            if *cell > prev_height {
                from_right[y][width - x - 1] = 1;
                prev_height = *cell;
            }
        }
    }

    for x in 0..width {
        prev_height = -1;
        for y in 0..height {
            let tree_height = cells[y][x];
            if tree_height > prev_height {
                from_top[y][x] = 1;
                prev_height = tree_height;
            }
        }
    }

    for x in 0..width {
        prev_height = -1;
        for y in 0..height {
            let tree_height = cells[height - 1 - y][x];
            if tree_height > prev_height {
                from_bottom[height - 1 - y][x] = 1;
                prev_height = tree_height;
            }
        }
    }

    let mut count = 0;
    for x in 0..width {
        for y in 0..height {
            count += from_top[y][x] | from_right[y][x] | from_bottom[y][x] | from_left[y][x];
        }
    }

    Some(count as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();

    let mut cells = vec![vec![0i32; width]; height];
    let mut view_scores = vec![vec![0i32; width]; height];

    for (i, line) in input.lines().enumerate() {
        let row = cells.get_mut(i).unwrap();

        for (j, c) in line.chars().enumerate() {
            *(*row).get_mut(j).unwrap() = c.to_digit(10).unwrap() as i32;
        }
    }

    for (y, row) in cells.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            let mut view_distance = 0;
            // look left
            for j in (0..x).rev() {
                if row[j] < *cell {
                    view_distance += 1;
                } else {
                    view_distance += 1;
                    break;
                }
            }
            view_scores[y][x] = view_distance;
            view_distance = 0;
            // Look up
            for j in (0..y).rev() {
                if cells[j][x] < *cell {
                    view_distance += 1;
                } else {
                    view_distance += 1;
                    break;
                }
            }
            view_scores[y][x] = view_scores[y][x] * view_distance;
            // Look right
            view_distance = 0;
            for j in (x + 1)..width {
                if cells[y][j] < *cell {
                    view_distance += 1;
                } else {
                    view_distance += 1;
                    break;
                }
            }
            view_scores[y][x] = view_scores[y][x] * view_distance;
            // Look down
            view_distance = 0;
            for j in y + 1..height {
                if cells[j][x] < *cell {
                    view_distance += 1;
                } else {
                    view_distance += 1;
                    break;
                }
            }
            view_scores[y][x] = view_scores[y][x] * view_distance;
        }
    }

    Some(*view_scores.iter().flatten().max().unwrap() as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
