use anyhow::{Context, Result};

use crate::maps::Map;

pub fn map_steps(s: &str) -> Result<usize> {
    Map::parse(s)
        .with_context(|| "parsing map")?
        .steps_from("AAA")
        .with_context(|| "counting steps")
}
