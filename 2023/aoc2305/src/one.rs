use anyhow::{Context, Result};

use crate::farm::Almanac;

pub fn closest_list_location(s: &str) -> Result<isize> {
    Almanac::with_seed_list(s)?
        .closest()
        .with_context(|| "no closest")
}
