use std::time::Instant;

fn main() {
    let input = include_str!("../../inputs/day01.txt");

    let start = Instant::now();
    let result1 = part1(input);
    let elapsed1 = start.elapsed();
    println!("Part 1: {} (took {:?})", result1, elapsed1);

    let start = Instant::now();
    let result2 = part2(input);
    let elapsed2 = start.elapsed();
    println!("Part 2: {} (took {:?})", result2, elapsed2);
}

fn parse_unlock(input: &str) -> Vec<i32> {
    input.lines().map(|line| {
        // separate first char from rest
        let direction = &line[0..1]; // "L" or "R"
        let number = &line[1..]; 

        let number: i32 = number.parse().unwrap();

        match direction {
            "L" => -number,
            "R" =>  number,
            _ => panic!("Unknown direction"),
        }
    }).collect()
}

fn part1(input: &str) -> i32 {
    // Your logic here
    let moves = parse_unlock(input);
    let mut count: i32 = 0;
    let mut position: i32 = 50;
    
    for rotation in moves {
        position += rotation;
        position = position % 100;
        if position == 0 {count += 1}
    }
    count
}

fn part2(input: &str) -> i32 {
    let moves = parse_unlock(input);
    let mut count: i32 = 0;
    let mut position: i32 = 50;
    let mut had_been_zero = false;
    
    for rotation in moves {
        position += rotation;
        if position > 0 {
            count += position / 100;
        } else {
            count += (position / 100).abs() + 1;
            if had_been_zero {count -= 1}
        }
        position = position.rem_euclid(100);
        if position == 0 {had_been_zero = true} else {had_been_zero = false}
    }
    
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let example = "R50\nL80";
        assert_eq!(part1(example), 1); // Replace 0 with example answer
    }

    #[test]
    fn test_part2() {
        let input = "L50\nL110\nR10";
        assert_eq!(part2(input), 3);
    }
}