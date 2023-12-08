use anyhow::{anyhow, bail, Context, Error, Result};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, Default)]
pub struct Map<'a> {
    directions: Vec<Direction>,
    nodes: HashMap<&'a str, (&'a str, &'a str)>,
}

impl<'a> Map<'a> {
    const FROM: &'a str = "AAA";
    const TO: &'a str = "ZZZ";

    pub fn parse(s: &str) -> Result<Map> {
        let mut lines = s.trim().lines();

        let directions = lines.next().with_context(|| "getting first line")?;
        let directions = directions
            .chars()
            .map(|c| match c {
                'L' => Ok(Direction::Left),
                'R' => Ok(Direction::Right),
                _ => Err(anyhow!("invalid direction: {c}")),
            })
            .collect::<Result<Vec<Direction>, Error>>()
            .with_context(|| "parsing directions")?;

        lines.next(); // skip the empty line

        let mut nodes = HashMap::new();
        for line in lines.rev() {
            if line.len() != 16 {
                bail!("invalid line length");
            }

            let name = &line[0..3];
            let left = &line[7..10];
            let right = &line[12..15];

            if nodes.insert(name, (left, right)).is_some() {
                bail!("duplicate node");
            }
        }
        Ok(Map { directions, nodes })
    }

    pub fn steps(&self) -> Result<usize> {
        let mut steps = 0;

        let mut current = Self::FROM;
        for dir in self.directions.iter().cycle() {
            steps += 1;
            let next = self
                .nodes
                .get(current)
                .with_context(|| "getting current node")?;
            current = match dir {
                Direction::Left => next.0,
                Direction::Right => next.1,
            };
            if current == Self::TO {
                break;
            }
        }

        Ok(steps)
    }
}

#[cfg(test)]
mod tests {
    use test_case::case;

    use super::*;

    const INPUT_ONE: &str = "
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
";

    const INPUT_TWO: &str = "
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
";

    #[case(INPUT_ONE, 2)]
    #[case(INPUT_TWO, 6)]
    fn test_map(input: &str, expected: usize) -> anyhow::Result<()> {
        let actual = Map::parse(input)
            .with_context(|| "parsing map")?
            .steps()
            .with_context(|| "counting steps")?;
        assert_eq!(actual, expected);
        Ok(())
    }
}
