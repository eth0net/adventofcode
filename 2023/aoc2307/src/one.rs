use anyhow::Result;

use crate::poker;

pub fn winnings(s: &str) -> Result<usize> {
    poker::play_without_joker(s)
}
