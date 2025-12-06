use std::time::Instant;

fn main() {
    let input = include_str!("../../inputs/day02.txt");

    let start = Instant::now();
    let result1 = part1(input);
    let elapsed1 = start.elapsed();
    println!("Part 1: {} (took {:?})", result1, elapsed1);

    let start = Instant::now();
    let result2 = part2(input);
    let elapsed2 = start.elapsed();
    println!("Part 2: {} (took {:?})", result2, elapsed2);
}

fn parse_input(input: &str) -> Vec<[u64; 2]> {
    let ranges = input.split(',');

    let ranges = ranges.filter_map(|s| {
        s.split_once('-')})
        .filter_map(|pair| {
            Some([(pair.0).parse::<u64>().ok()?, (pair.1).parse::<u64>().ok()?])
        })
        .collect();
    
    ranges
}

fn is_invalid_id_str_part1(id_str: String) -> bool {
    let id_str_len = id_str.len();
    if id_str_len % 2 != 0 {
        // something's gone wrong...
        println!("going wrong... with string {}", id_str);
    }
    assert!(id_str.len() % 2 == 0);
    
    let first_half  = &id_str[..(id_str_len/2)];
    let second_half = &id_str[(id_str_len/2)..];

    first_half == second_half
}

fn sum_invalid_ids_in_range_part1(range: [u64; 2]) -> u64 {
    let mut sum = 0;
    let mut i = range[0];

    while i <= range[1] {
        // 1. Get the string once
        let s = i.to_string();
        let len = s.len() as u32;

        if len % 2 != 0 {
            // Fix: Jump to the next power of 10 using the length
            // If len is 3 (100), we want 1000 (10^3).
            i = 10_u64.pow(len);
        } else {
            // 2. Pass the string we already have to the checker
            if is_invalid_id_str_part1(s) {
                sum += i;
            }
            i += 1;
        }
    }
    sum
}

fn part1(input: &str) -> u64 {
    let ranges = parse_input(input);
    let mut sum = 0;
    for range in ranges {
        sum += sum_invalid_ids_in_range_part1(range);
    }
    sum
}

fn part2(input: &str) -> u64 {
    let ranges = parse_input(input);
    let mut sum = 0;
    for range in ranges {
        sum += sum_invalid_ids_in_range_part2(range);
    }
    sum
}

fn sum_invalid_ids_in_range_part2(range: [u64; 2]) -> u64 {
    let mut sum = 0;

    for i in range[0]..=range[1] {
        // 1. Get the string once
        let s = i.to_string();
        if is_invalid_id_str_part2(s) {sum += i}
    }
    sum
}


fn is_invalid_id_str_part2(id_str: String) -> bool {
    let mut sub_len: usize = 1;

    while sub_len <= id_str.len() / 2 {
        if is_invalid_id_with_length(&id_str, sub_len) {return true}
        sub_len += 1;
    }
    false
}

// this function checks whether a string is invalid specifically with
// length of repeated substring equal to sub_len.
// core bit of is_invalid_id_str_part2
fn is_invalid_id_with_length(id_str: &str, sub_len: usize) -> bool {
    let full_length = id_str.len();
    if full_length % sub_len != 0 {return false}

    let ref_str = &id_str[0..sub_len];

    let mut i = sub_len;
    while i < full_length {
        if &id_str[i..i+sub_len] != ref_str {return false}
        i += sub_len;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input = "other_stuff";
        assert_eq!(part2(input), 3);
    }

    #[test]
    fn test_parse() {
        let input = "100-102,200-202";
        assert_eq!(parse_input(input), vec![[100, 102], [200, 202]]);
    }

    #[test]
    fn test_part_1() {
        let input = "99-1010,55-1010";
        assert_eq!(part1(input), 8u64);
    }

    #[test]
    fn test_is_invalid_id_with_length() {
        assert_eq!(is_invalid_id_with_length("123123123", 3), true);
        assert_eq!(is_invalid_id_with_length("123123123", 4), false);
    }
}