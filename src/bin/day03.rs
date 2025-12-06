fn main() {
    let input = include_str!("../../inputs/day03.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> u32 {
    let total_joltage: u32 = input.lines().map(|bank_str| bank_joltage(bank_str)).sum();
    total_joltage
}

fn part2(input: &str) -> u64 {
    let total_joltage: u64 = input.lines().map(|bank_str| super_bank_joltage(bank_str, 12)).sum();
    total_joltage
}

// take bank string slice, return chosen tens digit and its index in the bank
fn choose_tens(bank_str: &str) -> (u32, u32) {
    let mut incumbent_idx: u32 = 0;
    let mut incumbent_digit: u32 = 0;

    let char_count = bank_str.chars().count();

    // note cannot choose last digit for tens
    for (i, c) in bank_str.char_indices().take(char_count - 1) {
        let challenger_digit: u32 = c.to_digit(10).expect("Expected to read digit character!");
        if challenger_digit > incumbent_digit {
            incumbent_digit = challenger_digit;
            incumbent_idx = i as u32;
        }
    }

    (incumbent_idx, incumbent_digit)
}

// scans bank string past the tens index to find the highest digit,
// chooses units. no need to return index for part 1
fn choose_units(bank_str: &str, tens_idx: u32) -> u32 {
    let mut incumbent_digit: u32 = 0;
    for c in bank_str.chars().skip((tens_idx + 1) as usize) {
        let challenger_digit: u32 = c.to_digit(10).expect("Expected to read digit character!");
        if challenger_digit > incumbent_digit {incumbent_digit = challenger_digit}
    }
    incumbent_digit
}

fn bank_joltage(bank_str: &str) -> u32 {
    let (tens_idx, tens_digit) = choose_tens(bank_str);
    let units_digit = choose_units(bank_str, tens_idx);
    let joltage: u32 = 10 * tens_digit + units_digit;
    joltage
}

// generalising functionality of part 1.
// this function chooses the ith digit, counting from the right, 0-indexed.
// eg, units are 0th digit, tens are 1st, hundreds are 2nd, and so on.
// (this corresponds to exponent of 10 in scientific notation, hence
// argument name digit_exp).
// analogously to choose_tens above, it returns the index of the winning digit
// along with the winning digit.
// the next call requires the previous winning digit in order to know where
// it may begin its search (that index + 1)
fn choose_digit(bank_str: &str, digit_exp: usize, search_start_idx: usize) -> (u32, u32) {
    let mut incumbent_digit: u32 = 0;
    let mut incumbent_idx: u32 = 0;
    let char_count = bank_str.chars().count();
    for (i, c) in bank_str.char_indices().skip(search_start_idx).take(char_count - search_start_idx - digit_exp) {
        // println!("INNER Processing index {}", i);
        let challenger_digit: u32 = c.to_digit(10).expect("Expected to read digit character!");
        if challenger_digit > incumbent_digit {
            incumbent_digit = challenger_digit;
            incumbent_idx = i as u32;
        }
    }
    (incumbent_idx, incumbent_digit)
}

fn super_bank_joltage(bank_str: &str, no_digits: usize) -> u64 {
    let mut joltage: u64 = 0;
    let mut search_start_idx: usize = 0;
    for digit_exp in (0..no_digits).rev() {
        // println!("Processing exponent {}", digit_exp);
        // println!("Will search from index {} to length - {}", search_start_idx, digit_exp);
        let (digit_idx, digit) = choose_digit(bank_str, digit_exp, search_start_idx);
        // println!("Chose digit {} at index {}", digit, digit_idx);
        if digit_exp == 0 {
            joltage += digit as u64; // special case for units, breaks pow
        } else {
            joltage += (digit as u64) * (10u64).pow(digit_exp as u32);
        }
        search_start_idx = (digit_idx as usize) + 1;
    }
    joltage
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

    #[test]
    fn test_super_joltage() {
        let input = "818181911112111";
        assert_eq!(super_bank_joltage(input, 12), 888911112111);
    }
}