use anyhow::Result;
use aoc2305::{one, two};

const INPUT: &str = include_str!("../input");

fn main() -> Result<()> {
    let list = one::closest_list_location(INPUT)?;
    let range = two::closest_sequence_location(INPUT)?;

    println!("Closest with list: {list}");
    println!("Closest with range: {range}");

    Ok(())
}
