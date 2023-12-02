use anyhow::{Context, Result};

use crate::cubes::Game;

pub fn sum_possible(input: &str) -> Result<usize> {
    let mut count = 0;

    for line in input.lines() {
        let game = Game::parse(line).with_context(|| "parsing game")?;

        if game.possible(12, 13, 14) {
            count += game.id
        }
    }

    Ok(count)
}
