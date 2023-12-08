use anyhow::{Context, Result};
use num::Integer;

use crate::maps::Map;

pub fn map_steps(s: &str) -> Result<usize> {
    let map = Map::parse(s).with_context(|| "parsing map")?;
    let starts = map.nodes().into_iter().filter(|n| n.ends_with('A'));

    starts
        .map(|start| map.steps_from(start))
        .collect::<Result<Vec<usize>>>()
        .with_context(|| "counting steps")?
        .into_iter()
        .reduce(|a, b| a.lcm(&b))
        .with_context(|| "totalling steps")
}
