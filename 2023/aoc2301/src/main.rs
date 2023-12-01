use anyhow::{Context, Result};

const INPUT: &str = include_str!("../input");

fn main() -> Result<()> {
    let mut sum = 0;

    for line in INPUT.lines() {
        sum += evaluate_line(line)?;
    }

    println!("Calibration value: {sum}");

    Ok(())
}

fn evaluate_line(line: &str) -> Result<u32> {
    let numbers = find_numbers(line)?;

    let first = numbers.first().with_context(|| "no first digit found")?;
    let last = numbers.last().with_context(|| "no first digit found")?;

    let value = format!("{first}{last}")
        .parse::<u32>()
        .with_context(|| "parsing calibration value")?;

    Ok(value)
}

fn find_numbers(line: &str) -> Result<Vec<u32>> {
    let mut numbers = Vec::new();

    for (idx, char) in line.char_indices() {
        match char {
            '0'..='9' => numbers.push(char.to_digit(10).unwrap()),
            'o' if check_range(line, idx, "one") => {
                numbers.push(1);
            }
            't' if check_range(line, idx, "two") => {
                numbers.push(2);
            }
            't' if check_range(line, idx, "three") => {
                numbers.push(3);
            }
            'f' if check_range(line, idx, "four") => {
                numbers.push(4);
            }
            'f' if check_range(line, idx, "five") => {
                numbers.push(5);
            }
            's' if check_range(line, idx, "six") => {
                numbers.push(6);
            }
            's' if check_range(line, idx, "seven") => {
                numbers.push(7);
            }
            'e' if check_range(line, idx, "eight") => {
                numbers.push(8);
            }
            'n' if check_range(line, idx, "nine") => {
                numbers.push(9);
            }
            _ => {}
        }
    }

    Ok(numbers)
}

fn check_range(target: &str, start: usize, expected: &str) -> bool {
    let end = start + expected.len();

    match target.get(start..end) {
        Some(range) => range == expected,
        None => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("two1nine", 29 ; "two1nine")]
    #[test_case("eightwothree", 83 ; "eightwothree")]
    #[test_case("abcone2threexyz", 13 ; "abcone2threexyz")]
    #[test_case("xtwone3four", 24 ; "xtwone3four")]
    #[test_case("4nineeightseven2", 42 ; "4nineeightseven2")]
    #[test_case("zoneight234", 14 ; "zoneight234")]
    #[test_case("7pqrstsixteen", 76 ; "7pqrstsixteen")]
    fn test_evaluate_line(input: &str, expected: u32) {
        let actual = evaluate_line(input).unwrap();
        assert_eq!(actual, expected)
    }

    #[test_case("two1nine", vec![2, 1, 9] ; "two1nine")]
    #[test_case("eightwothree", vec![8, 2, 3] ; "eightwothree")]
    #[test_case("abcone2threexyz", vec![1, 2, 3] ; "abcone2threexyz")]
    #[test_case("xtwone3four", vec![2, 1, 3, 4] ; "xtwone3four")]
    #[test_case("4nineeightseven2", vec![4, 9, 8, 7, 2] ; "4nineeightseven2")]
    #[test_case("zoneight234", vec![1, 8, 2, 3, 4] ; "zoneight234")]
    #[test_case("7pqrstsixteen", vec![7, 6] ; "7pqrstsixteen")]
    fn test_find_numbers(input: &str, expected: Vec<u32>) {
        let actual = find_numbers(input).unwrap();
        assert_eq!(actual, expected)
    }

    #[test_case("two1nine", 0, "two", true ; "two1nine")]
    #[test_case("eightwothree", 4, "two", true ; "eightwothree")]
    #[test_case("abcone2threexyz", 3, "one", true ; "abcone2threexyz")]
    #[test_case("xtwone3four", 1, "two", true ; "xtwone3four")]
    #[test_case("4nineeightseven2", 5, "eight", true ; "4nineeightseven2")]
    #[test_case("zoneight234", 3, "eight", true ; "zoneight234")]
    #[test_case("7pqrstsixteen", 6, "six", true ; "7pqrstsixteen")]
    #[test_case("abc", 0, "xyz", false ; "abc")]
    fn test_check_range(input: &str, start: usize, expected: &str, expected_result: bool) {
        let actual = check_range(input, start, expected);
        assert_eq!(actual, expected_result)
    }
}
