use anyhow::{Context, Result};

use crate::farm::Almanac;

pub fn closest_sequence_location(s: &str) -> Result<isize> {
    Almanac::with_seed_ranges(s)?
        .closest()
        .with_context(|| "no closest")
}
