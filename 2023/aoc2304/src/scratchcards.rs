use std::{collections::HashMap, num::ParseIntError};

use anyhow::{Context, Result};

#[derive(Debug, Default)]
pub struct Table {
    cards: HashMap<usize, (Card, usize)>,
}

impl Table {
    pub fn parse(s: &str) -> Result<Table> {
        let mut table = Table::default();

        for line in s.lines() {
            if line.is_empty() {
                continue;
            }
            table.add_card(&Card::parse(line)?)
        }

        Ok(table)
    }

    fn add_card(&mut self, card: &Card) {
        self.cards
            .entry(card.id)
            .and_modify(|(_, c)| *c += 1)
            .or_insert((card.clone(), 1));
    }

    pub fn count_scratchcards(&self) -> usize {
        let mut total_count = 0;

        let mut cards = self.cards.clone();

        let mut keys: Vec<&usize> = self.cards.keys().collect();
        keys.sort_unstable();

        for id in keys {
            let (card, count) = cards.get(id).unwrap().clone();

            total_count += count;

            for i in 1..=card.winning_count() {
                cards.entry(id + i).and_modify(|(_, c)| *c += count);
            }
        }

        total_count
    }

    pub fn score(&self) -> usize {
        self.cards
            .iter()
            .fold(0, |acc, (_, (card, _))| acc + card.score())
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Card {
    id: usize,
    winning_numbers: Vec<usize>,
    scratch_numbers: Vec<usize>,
}

impl Card {
    pub fn parse(s: &str) -> Result<Card> {
        let mut card = Card::default();

        let (id, s) = s.split_once(": ").with_context(|| "splitting prefix")?;

        card.id = id
            .strip_prefix("Card")
            .with_context(|| "stripping prefix")?
            .trim()
            .parse()
            .with_context(|| "parsing id")?;

        let (winning, scratch) = s.split_once(" | ").with_context(|| "splitting numbers")?;

        card.winning_numbers = parse_number_set(winning)?;
        card.scratch_numbers = parse_number_set(scratch)?;

        Ok(card)
    }

    pub fn winning_count(&self) -> usize {
        self.winning_numbers
            .iter()
            .filter(|w| self.scratch_numbers.contains(w))
            .count()
    }

    pub fn score(&self) -> usize {
        (1 << self.winning_count()) >> 1
    }
}

fn parse_number_set(s: &str) -> Result<Vec<usize>> {
    s.split_ascii_whitespace()
        .map(str::parse::<usize>)
        .collect::<Result<Vec<usize>, ParseIntError>>()
        .with_context(|| "parsing number set")
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;

    #[test]
    fn test_table_parse() {
        let input = "
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11

";
        let table = Table::parse(input).expect("table should parse");
        assert_eq!(table.count_scratchcards(), 30);
    }

    #[test_case(
        "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
        Card{
            id: 1,
            winning_numbers: vec![41, 48, 83, 86, 17],
            scratch_numbers: vec![83, 86, 6, 31, 17, 9, 48, 53],
        },
        8
        ; "card 1"
    )]
    #[test_case(
        "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
        Card{
            id: 2,
            winning_numbers: vec![13, 32, 20, 16, 61],
            scratch_numbers: vec![61, 30, 68, 82, 17, 32, 24, 19],
        },
        2
        ; "card 2"
    )]
    #[test_case(
        "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
        Card{
            id: 3,
            winning_numbers: vec![ 1, 21, 53, 59, 44],
            scratch_numbers: vec![69, 82, 63, 72, 16, 21, 14,  1],
        },
        2
        ; "card 3"
    )]
    #[test_case(
        "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
        Card{
            id: 4,
            winning_numbers: vec![41, 92, 73, 84, 69],
            scratch_numbers: vec![59, 84, 76, 51, 58,  5, 54, 83],
        },
        1
        ; "card 4"
    )]
    #[test_case(
        "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
        Card{
            id: 5,
            winning_numbers: vec![87, 83, 26, 28, 32],
            scratch_numbers: vec![88, 30, 70, 12, 93, 22, 82, 36],
        },
        0
        ; "card 5"
    )]
    #[test_case(
        "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        Card{
            id: 6,
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
