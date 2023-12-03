use anyhow::{Context, Result};

use crate::cubes::{Game, Set};

pub fn sum_sets_power(s: &str) -> Result<usize> {
    let mut count = 0;

    for line in s.lines() {
        let mut bag = Set::default();
        let game = Game::parse(line).with_context(|| "parsing game")?;
        
        for round in game.rounds {
            if round.red > bag.red {
                bag.red = round.red
            }
            if round.green > bag.green {
                bag.green = round.green
            }
            if round.blue > bag.blue {
                bag.blue = round.blue
            }
        }
        
        count += bag.power()
    }

    Ok(count)
}
