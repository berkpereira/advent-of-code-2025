fn main() {
    let input = include_str!("../../inputs/day--.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> i32 {
    0
}

fn part2(input: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

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