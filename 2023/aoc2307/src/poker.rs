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
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Equal => self.cards.cmp(&other.cards),
            o => o,
        }
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
                false => Type::FullHouse,
            },
            3 => match values.contains(&3) {
                true => Type::ThreeOfAKind,
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
        self.strength.cmp(&other.strength)
    }
}

impl Card {
    fn new(label: char) -> Result<Card> {
        let strength = match label {
            'A' => 0,
            'K' => 1,
            'Q' => 2,
            'J' => 3,
            'T' => 4,
            '9' => 5,
            '8' => 6,
            '7' => 7,
            '6' => 8,
            '5' => 9,
            '4' => 10,
            '3' => 11,
            '2' => 12,
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

    #[test]
    fn test_play() {
        let input = "
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";
        let expected = 6440;
        assert_eq!(play(input).unwrap(), expected);
    }

    #[test]
    fn test_play_2() {
        let input = "
AAAAA 2
22222 3
AAAAK 5
22223 7
AAAKK 11
22233 13
AAAKQ 17
22234 19
AAKKQ 23
22334 29
AAKQJ 31
22345 37
AKQJT 41
23456 43
";

        let expected = 1343;
        assert_eq!(play(input).unwrap(), expected);
    }

    #[test]
    fn test_set_ord() -> Result<()> {
        let mut input = [
            Set {
                hand: Hand::parse("32T3K")?,
                bid: 765,
            },
            Set {
                hand: Hand::parse("T55J5")?,
                bid: 684,
            },
            Set {
                hand: Hand::parse("KK677")?,
                bid: 28,
            },
            Set {
                hand: Hand::parse("KTJJT")?,
                bid: 220,
            },
            Set {
                hand: Hand::parse("QQQJA")?,
                bid: 483,
            },
        ];
        let expected = [
            Set {
                hand: Hand::parse("QQQJA")?,
                bid: 483,
            },
            Set {
                hand: Hand::parse("T55J5")?,
                bid: 684,
            },
            Set {
                hand: Hand::parse("KK677")?,
                bid: 28,
            },
            Set {
                hand: Hand::parse("KTJJT")?,
                bid: 220,
            },
            Set {
                hand: Hand::parse("32T3K")?,
                bid: 765,
            },
        ];
        input.sort_unstable();
        assert_eq!(input, expected);
        Ok(())
    }

    #[test]
    fn test_hand_ord() -> Result<()> {
        let mut input = [
            Hand::parse("32T3K")?,
            Hand::parse("T55J5")?,
            Hand::parse("KK677")?,
            Hand::parse("KTJJT")?,
            Hand::parse("QQQJA")?,
        ];
        let expected = [
            Hand::parse("QQQJA")?,
            Hand::parse("T55J5")?,
            Hand::parse("KK677")?,
            Hand::parse("KTJJT")?,
            Hand::parse("32T3K")?,
        ];
        input.sort_unstable();
        assert_eq!(input, expected);
        Ok(())
    }

    #[test]
    fn test_hand_ord_2() -> Result<()> {
        let mut input = [
            Hand::parse("23456")?,
            Hand::parse("AKQJT")?,
            Hand::parse("22345")?,
            Hand::parse("AAKQJ")?,
            Hand::parse("22334")?,
            Hand::parse("AAKKQ")?,
            Hand::parse("22234")?,
            Hand::parse("AAAKQ")?,
            Hand::parse("22233")?,
            Hand::parse("AAAKK")?,
            Hand::parse("22223")?,
            Hand::parse("AAAAK")?,
            Hand::parse("22222")?,
            Hand::parse("AAAAA")?,
        ];
        let expected = [
            Hand::parse("AAAAA")?,
            Hand::parse("22222")?,
            Hand::parse("AAAAK")?,
            Hand::parse("22223")?,
            Hand::parse("AAAKK")?,
            Hand::parse("22233")?,
            Hand::parse("AAAKQ")?,
            Hand::parse("22234")?,
            Hand::parse("AAKKQ")?,
            Hand::parse("22334")?,
            Hand::parse("AAKQJ")?,
            Hand::parse("22345")?,
            Hand::parse("AKQJT")?,
            Hand::parse("23456")?,
        ];
        input.sort_unstable();
        assert_eq!(input, expected);
        Ok(())
    }

    #[test]
    fn test_hand_ord_3() -> Result<()> {
        let mut input = [Hand::parse("33322")?, Hand::parse("KKKKQ")?];
        let expected = [Hand::parse("KKKKQ")?, Hand::parse("33322")?];
        input.sort_unstable();
        assert_eq!(input, expected);
        Ok(())
    }

    #[test]
    fn test_card_ord() -> Result<()> {
        let mut input = [
            Card::new('2')?,
            Card::new('3')?,
            Card::new('4')?,
            Card::new('5')?,
            Card::new('6')?,
            Card::new('7')?,
            Card::new('8')?,
            Card::new('9')?,
            Card::new('T')?,
            Card::new('J')?,
            Card::new('Q')?,
            Card::new('K')?,
            Card::new('A')?,
        ];
        let expected = [
            Card::new('A')?,
            Card::new('K')?,
            Card::new('Q')?,
            Card::new('J')?,
            Card::new('T')?,
            Card::new('9')?,
            Card::new('8')?,
            Card::new('7')?,
            Card::new('6')?,
            Card::new('5')?,
            Card::new('4')?,
            Card::new('3')?,
            Card::new('2')?,
        ];
        input.sort_unstable();
        assert_eq!(input, expected);
        Ok(())
    }

    #[test]
    fn test_hand_type_ord() {
        let mut input = [
            Type::HighCard,
            Type::OnePair,
            Type::TwoPair,
            Type::ThreeOfAKind,
            Type::FullHouse,
            Type::FourOfAKind,
            Type::FiveOfAKind,
        ];
        let expected = [
            Type::FiveOfAKind,
            Type::FourOfAKind,
            Type::FullHouse,
            Type::ThreeOfAKind,
            Type::TwoPair,
            Type::OnePair,
            Type::HighCard,
        ];
        input.sort_unstable();
        assert_eq!(input, expected);
    }
}
