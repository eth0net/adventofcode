use crate::boats::RaceList;
use anyhow::Result;

pub fn race_result(s: &str) -> Result<usize> {
    Ok(RaceList::parse(s)?.win_score())
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_race_result() -> Result<()> {
        let input = "
Time:      7  15   30
Distance:  9  40  200

"
        .trim_start();
        let expect = 288;
        assert_eq!(race_result(input)?, expect);
        Ok(())
    }
}
