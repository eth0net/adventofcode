use anyhow::{Context, Error, Result};

use crate::mirage::extrapolate_next;

pub fn extrapolate(s: &str) -> Result<isize> {
    let values = s
        .lines()
        .filter(|line| !line.is_empty())
        .map(extrapolate_next)
        .collect::<Result<Vec<isize>, Error>>()
        .with_context(|| "extrapolating sequences")?;
    Ok(values.into_iter().sum())
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use test_case::case;

    use super::*;

    const EXAMPLE: &str = include_str!("../example");

    #[case(EXAMPLE, 114 ; "example data")]
    fn test_extrapolate_sum(input: &str, expected: isize) -> Result<()> {
        assert_eq!(extrapolate(input)?, expected);
        Ok(())
    }
}
