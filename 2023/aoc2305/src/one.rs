use anyhow::{Context, Result};

use crate::farm::Almanac;

pub fn closest_seed_location(s: &str) -> Result<isize> {
    Almanac::parse(s)?.closest().with_context(|| "no closest")
}
