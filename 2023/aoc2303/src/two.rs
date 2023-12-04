use anyhow::Result;

use crate::engine::Engine;

pub fn sum_gear_ratios(s: &str) -> Result<usize> {
    let engine = Engine::parse(s)?;

    Ok(engine.sum_gear_ratios())
}
