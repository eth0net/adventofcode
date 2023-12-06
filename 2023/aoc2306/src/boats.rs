use std::num::ParseIntError;

use anyhow::{Context, Result};

#[derive(Debug, Default)]
pub struct RaceList {
    races: Vec<Race>,
}

impl RaceList {
    fn new(races: Vec<Race>) -> RaceList {
        RaceList { races }
    }

    pub fn parse(s: &str) -> Result<RaceList> {
        let mut lines = s.lines();
        let times = lines
            .next()
            .with_context(|| "getting first line")?
            .strip_prefix("Time:")
            .with_context(|| "stripping time prefix")?
            .split_ascii_whitespace()
            .map(str::parse::<usize>)
            .collect::<Result<Vec<usize>, ParseIntError>>()
            .with_context(|| "parsing times")?;
        let distances = lines
            .next()
            .with_context(|| "getting second line")?
            .strip_prefix("Distance:")
            .with_context(|| "stripping distance prefix")?
            .split_ascii_whitespace()
            .map(str::parse::<usize>)
            .collect::<Result<Vec<usize>, ParseIntError>>()
            .with_context(|| "parsing times")?;

        let races = times
            .into_iter()
            .zip(distances)
            .map(|(time, distance)| Race::new(time, distance))
            .collect();

        Ok(RaceList::new(races))
    }

    pub fn win_score(&self) -> usize {
        self.races
            .iter()
            .map(|r| r.possible_wins())
            .reduce(|a, b| a * b)
            .unwrap_or_default()
    }
}

#[derive(Debug, Default)]
struct Race {
    time: usize,
    distance: usize,
}

impl Race {
    fn new(time: usize, distance: usize) -> Race {
        Race { time, distance }
    }

    fn possible_wins(&self) -> usize {
        (1..self.time)
            .filter(|t| {
                let speed = t;
                let time = self.time - t;
                let distance = speed * time;
                distance > self.distance
            })
            .count()
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;

    #[test_case(Race::new(7, 9), 4)]
    #[test_case(Race::new(15, 40), 8)]
    #[test_case(Race::new(30, 200), 9)]
    fn test_race_possible_wins(race: Race, expected: usize) {
        assert_eq!(race.possible_wins(), expected)
    }
}
