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

impl<'a> Iterator for Map<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

impl<'a> Map<'a> {
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

    pub fn nodes(&self) -> Vec<&&str> {
        self.nodes.keys().collect()
    }

    pub fn steps_from(&self, from: &str) -> Result<usize> {
        let mut steps = 0;

        let mut current = from;
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
            if current.ends_with('Z') {
                break;
            }
        }

        Ok(steps)
    }
}

#[cfg(test)]
mod tests {
    use num::Integer;
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

    const INPUT_THREE: &str = "
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
";

    #[case(INPUT_ONE, 2)]
    #[case(INPUT_TWO, 6)]
    fn test_map_aaa(input: &str, expected: usize) -> anyhow::Result<()> {
        let actual = Map::parse(input)
            .with_context(|| "parsing map")?
            .steps_from("AAA")
            .with_context(|| "counting steps")?;
        assert_eq!(actual, expected);
        Ok(())
    }

    #[case(INPUT_THREE, 6)]
    fn test_map_many(input: &str, expected: usize) -> anyhow::Result<()> {
        let map = Map::parse(input).with_context(|| "parsing map")?;
        let actual = map
            .nodes()
            .into_iter()
            .filter(|n| n.ends_with('A'))
            .map(|start| map.steps_from(start))
            .collect::<Result<Vec<usize>>>()
            .with_context(|| "counting steps")?
            .into_iter()
            .reduce(|a, b| a.lcm(&b))
            .with_context(|| "totalling steps")?;
        assert_eq!(actual, expected);
        Ok(())
    }
}
