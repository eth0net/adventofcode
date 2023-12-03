use anyhow::Result;
use aoc_client::{AocClient, PuzzleDay, PuzzleYear};

pub fn download(year: PuzzleYear, day: PuzzleDay) -> Result<()> {
    let client = AocClient::builder()
        .overwrite_files(true)
        .session_cookie_from_default_locations()?
        .year(year)?
        .day(day)?
        .build()?;

    client.save_input()?;
    client.save_puzzle_markdown()?;

    Ok(())
}
