pub fn most_calories(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.trim())
        .map(str::parse::<usize>)
        .fold((0, 0), |(mut high, mut curr), line| match line {
            Ok(val) => {
                curr += val;
                if high < curr {
                    high = curr
                };
                (high, curr)
            }
            Err(_) => (high, 0),
        })
        .0
}

// pub fn top_three(input: &str) -> usize {
//     let mut stack = vec![0];
//     let mut idx = 0;

//     input
//         .lines()
//         .map(|line| line.trim())
//         .map(|line| str::parse::<usize>(line))
//         .for_each(|line| {});
// }

#[cfg(test)]
mod tests {
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
    fn most_calories() {
        assert_eq!(super::most_calories(INPUT), 24000);
    }

    // #[test]
    // fn top_three() {
    //     assert_eq!(super::top_three(INPUT), 45000);
    // }
}
