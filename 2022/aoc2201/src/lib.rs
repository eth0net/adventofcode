pub fn most_calories(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.trim())
        .fold((0, 0), |(mut high, mut curr), line| {
            if line.is_empty() {
                return (high, 0);
            }

            if let Ok(val) = str::parse::<usize>(line) {
                curr += val
            }

            if high < curr {
                high = curr
            }

            (high, curr)
        })
        .0
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
    1000
    2000
    3000
    
    4000
    
    5000
    6000
    
    7000
    8000
    9000
    
    10000
    ";

    #[test]
    fn parse_input() {
        assert_eq!(most_calories(INPUT), 24000);
    }
}
