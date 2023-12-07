use std::{cmp::Ordering, collections::HashMap};

use anyhow::{bail, Context, Result};

pub fn play(s: &str) -> Result<usize> {
    let mut sets = s
        .lines()
        .filter(|l| !l.is_empty())
        .map(Set::parse)
        .collect::<Result<Vec<Set>>>()
        .with_context(|| "parsing sets")?;
    sets.sort_unstable();

    let winnings = sets
        .iter()
        .enumerate()
        .map(|(i, set)| {
            let rank = sets.len() - i;
            set.bid * rank
        })
        .reduce(|a, b| a + b)
        .with_context(|| "totalling winnings")?;

    Ok(winnings)
}

#[derive(Debug, PartialEq, Eq)]
struct Set {
    hand: Hand,
    bid: usize,
}

impl PartialOrd for Set {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Set {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.hand.cmp(&other.hand)
    }
}

impl Set {
    fn parse(s: &str) -> Result<Set> {
        let (hand, bid) = s.split_once(' ').with_context(|| "splitting line")?;
        let hand = Hand::parse(hand).with_context(|| "parsing hand")?;
        let bid = bid.parse().with_context(|| "parsing bid")?;
        Ok(Set { hand, bid })
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: [Card; 5],
    hand_type: Type,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let hand_type = self.hand_type.cmp(&other.hand_type);
        if hand_type != Ordering::Equal {
            return hand_type;
        }

        self.cards.cmp(&other.cards)
    }
}

impl Hand {
    fn parse(s: &str) -> Result<Hand> {
        if s.len() != 5 {
            bail!("hand length must be equal to 5: {s}");
        }

        let mut cards: [Card; 5] = Default::default();
        let mut hash: HashMap<char, u8> = HashMap::new();

        for (idx, label) in s.char_indices() {
            cards[idx] = Card::new(label)?;
            hash.entry(label).and_modify(|c| *c += 1).or_insert(1);
        }

        let values: Vec<u8> = hash.into_values().collect();

        let hand_type = match values.len() {
            1 => Type::FiveOfAKind,
            2 => match values.contains(&4) {
                true => Type::FourOfAKind,
                false => Type::ThreeOfAKind,
            },
            3 => match values.contains(&3) {
                true => Type::FullHouse,
                false => Type::TwoPair,
            },
            4 => Type::OnePair,
            5 => Type::HighCard,
            _ => bail!("bad hand length"),
        };

        Ok(Hand { cards, hand_type })
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
struct Card {
    label: char,
    strength: usize,
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.strength.cmp(&other.strength) {
            Ordering::Less => Ordering::Greater,
            Ordering::Equal => Ordering::Equal,
            Ordering::Greater => Ordering::Less,
        }
    }
}

impl Card {
    const LABEL_STRENGTH: [char; 13] = [
        'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
    ];

    fn new(label: char) -> Result<Card> {
        let strength = match Self::LABEL_STRENGTH.contains(&label) {
            true => Self::LABEL_STRENGTH.partition_point(|l| l == &label),
            _ => bail!("invalid label: {label}"),
        };

        Ok(Card { label, strength })
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Type {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483

";

    #[test]
    fn test_play() {
        let expected = 6440;
        assert_eq!(play(INPUT).unwrap(), expected);
    }
}
