use anyhow::Result;

use crate::engine::Engine;

pub fn sum_missing_parts(s: &str) -> Result<usize> {
    let engine = Engine::parse(s)?;

    Ok(engine.sum_missing_parts())
}
