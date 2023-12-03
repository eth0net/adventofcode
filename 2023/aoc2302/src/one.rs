use anyhow::{Context, Result};

use crate::cubes::{Game, Set};

pub fn sum_possible(s: &str) -> Result<usize> {
    let mut count = 0;

    let bag = Set::new(12, 13, 14);

    for line in s.lines() {
        let game = Game::parse(line).with_context(|| "parsing game")?;

        if game.possible(&bag) {
            count += game.id
        }
    }

    Ok(count)
}
