fn main() {
    let input = include_str!("../../inputs/day04.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

struct Grid {
    data: Vec<bool>,
    width: usize,
    height: usize,
}

impl Grid {
    fn get(&self, row: usize, col: usize) -> bool {
        self.data[row * self.width + col]
    }

    fn set(&mut self, row: usize, col: usize, value: bool) {
        self.data[row * self.width + col] = value;
    }

    fn check_index(&self, row: isize, col: isize) -> bool {
        row >= 0 && col >= 0 && (row as usize) <= (self.height - 1) && (col as usize) <= (self.width - 1)
    }

    fn count_neighbours(&self, row: usize, col: usize) -> usize {
        assert!(self.get(row, col), "({}, {}) is not the location of a roll!", row, col);
        let mut candidate_idx;
        let mut neighbour_count = 0;
        for i in -1isize..=1 {
            for j in -1isize..=1 {
                if i == 0 && j == 0 {continue}
                candidate_idx = [(row as isize) + i, (col as isize) + j];
                if self.check_index(candidate_idx[0], candidate_idx[1]) && self.get(candidate_idx[0] as usize, candidate_idx[1] as usize) {
                    neighbour_count += 1
                }
            }
        }
        neighbour_count
    }
}

fn parse_input_to_grid(input: &str) -> Grid {
    let data = input.chars().filter_map(|c| {
        if c == '@' {
            Some(true)
        } else if c == '.' {
            Some(false)
        } else {
            None
        }
    }).collect();

    let height = input.lines().count();
    let width = input.lines().next().unwrap_or("").len();

    Grid {
        data,
        width,
        height,
    }
}

fn part1(input: &str) -> usize {
    let mut count = 0;
    let map_grid = parse_input_to_grid(input);
    for row in 0..map_grid.height {
        for col in 0..map_grid.width {
            if map_grid.get(row, col) && map_grid.count_neighbours(row, col) < 4 {
                count += 1;
            }
        }
    }
    count
}

fn part2(input: &str) -> i32 {
    let mut total_removed = 0;
    let mut map_grid = parse_input_to_grid(input);
    loop {
        let mut removed_this_pass = 0;
        for row in 0..map_grid.height {
            for col in 0..map_grid.width {
                if map_grid.get(row, col) && map_grid.count_neighbours(row, col) < 4 {
                    map_grid.set(row, col, false);
                    removed_this_pass += 1;
                    total_removed += 1;
                }
            }
        }
        if removed_this_pass == 0 {break}
    }
    total_removed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = "@..\n.@.";
        let grid = parse_to_grid(input);
        println!("{:?}", grid.data);
        println!("Width is {}", grid.width);
        println!("Height is {}", grid.height);
    }

    #[test]
    fn test_part1() {
        let input = "stuff";
        assert_eq!(part1(input), 1); // Replace 0 with example answer
    }

    #[test]
    fn test_part2() {
        let input = "other_stuff";
        assert_eq!(part2(input), 3);
    }
}