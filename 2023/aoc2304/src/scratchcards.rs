use std::{num::ParseIntError, usize};

use anyhow::{Context, Result};

#[derive(Debug, Default, Eq, PartialEq)]
pub struct Card {
    winning_numbers: Vec<u8>,
    scratch_numbers: Vec<u8>,
}

impl Card {
    pub fn parse(s: &str) -> Result<Card> {
        let mut card = Card::default();

        let (_, s) = s.split_once(": ").with_context(|| "splitting prefix")?;

        let (winning, scratch) = s.split_once(" | ").with_context(|| "splitting numbers")?;

        card.winning_numbers = parse_number_set(winning)?;
        card.scratch_numbers = parse_number_set(scratch)?;

        Ok(card)
    }

    pub fn score(&self) -> usize {
        let winning_count = self
            .winning_numbers
            .iter()
            .filter(|w| self.scratch_numbers.contains(w))
            .count();
        (1 << winning_count) >> 1
    }
}

fn parse_number_set(s: &str) -> Result<Vec<u8>> {
    s.split_ascii_whitespace()
        .map(str::parse::<u8>)
        .collect::<Result<Vec<u8>, ParseIntError>>()
        .with_context(|| "parsing number set")
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;

    #[test_case(
        "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
        Card{
            winning_numbers: vec![41, 48, 83, 86, 17],
            scratch_numbers: vec![83, 86, 6, 31, 17, 9, 48, 53],
        },
        8
        ; "card 1"
    )]
    #[test_case(
        "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
        Card{
            winning_numbers: vec![13, 32, 20, 16, 61],
            scratch_numbers: vec![61, 30, 68, 82, 17, 32, 24, 19],
        },
        2
        ; "card 2"
    )]
    #[test_case(
        "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
        Card{
            winning_numbers: vec![ 1, 21, 53, 59, 44],
            scratch_numbers: vec![69, 82, 63, 72, 16, 21, 14,  1],
        },
        2
        ; "card 3"
    )]
    #[test_case(
        "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
        Card{
            winning_numbers: vec![41, 92, 73, 84, 69],
            scratch_numbers: vec![59, 84, 76, 51, 58,  5, 54, 83],
        },
        1
        ; "card 4"
    )]
    #[test_case(
        "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
        Card{
            winning_numbers: vec![87, 83, 26, 28, 32],
            scratch_numbers: vec![88, 30, 70, 12, 93, 22, 82, 36],
        },
        0
        ; "card 5"
    )]
    #[test_case(
        "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        Card{
            winning_numbers: vec![31, 18, 13, 56, 72],
            scratch_numbers: vec![74, 77, 10, 23, 35, 67, 36, 11],
        },
        0
        ; "card 6"
    )]
    fn test_card_parse(input: &str, expected_card: Card, expected_score: usize) {
        let card = Card::parse(input).expect("card should parse");
        assert_eq!(card, expected_card, "card not equal to expected");
        assert_eq!(card.score(), expected_score, "score not equal to expected")
    }
}
