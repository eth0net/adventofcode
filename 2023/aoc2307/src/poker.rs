use std::{cmp::Ordering, collections::HashMap};

use anyhow::{bail, Context, Result};

pub fn play_without_joker(s: &str) -> Result<usize> {
    parse(s, false)
}

pub fn play_with_joker(s: &str) -> Result<usize> {
    parse(s, true)
}

fn parse(s: &str, with_joker: bool) -> Result<usize> {
    let mut sets = s
        .lines()
        .filter(|l| !l.is_empty())
        .map(|s| Set::parse(s, with_joker))
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
    fn parse(s: &str, with_joker: bool) -> Result<Set> {
        let (hand, bid) = s.split_once(' ').with_context(|| "splitting line")?;
        let hand = Hand::parse(hand, with_joker).with_context(|| "parsing hand")?;
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
    fn parse(s: &str, with_joker: bool) -> Result<Hand> {
        if s.len() != 5 {
            bail!("hand length must be equal to 5: {s}");
        }

        let mut cards: [Card; 5] = Default::default();

        for (idx, label) in s.char_indices() {
            cards[idx] = match with_joker {
                true => Card::with_joker(label)?,
                false => Card::without_joker(label)?,
            };
        }

        let hand_type = match with_joker {
            true => Type::with_joker(&cards),
            false => Type::without_joker(&cards),
        };

        Ok(Hand { cards, hand_type })
    }
}

#[derive(Debug, Default, PartialEq, Eq, Hash)]
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
    fn without_joker(label: char) -> Result<Card> {
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

    fn with_joker(label: char) -> Result<Card> {
        let strength = match label {
            'A' => 0,
            'K' => 1,
            'Q' => 2,
            'T' => 3,
            '9' => 4,
            '8' => 5,
            '7' => 6,
            '6' => 7,
            '5' => 8,
            '4' => 9,
            '3' => 10,
            '2' => 11,
            'J' => 12,
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

impl Type {
    fn without_joker(cards: &[Card; 5]) -> Type {
        let mut hash = HashMap::new();
        for card in cards {
            hash.entry(card).and_modify(|c| *c += 1).or_insert(1);
        }

        let values: Vec<u8> = hash.into_values().collect();
        match values.len() {
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
            _ => panic!("cannot happen"),
        }
    }

    fn with_joker(cards: &[Card]) -> Type {
        let mut hash = HashMap::new();
        for card in cards {
            hash.entry(card).and_modify(|c| *c += 1).or_insert(1);
        }

        if let Some(joker_count) = hash.remove(&Card::with_joker('J').unwrap()) {
            if joker_count == 5 {
                return Type::FiveOfAKind;
            }
            let highest = hash
                .iter()
                .reduce(|high, other| match high.1.cmp(other.1) {
                    Ordering::Less => other,
                    Ordering::Equal => match high.0.cmp(other.0) {
                        Ordering::Less => other,
                        Ordering::Equal => panic!("cannot happen"),
                        Ordering::Greater => high,
                    },
                    Ordering::Greater => high,
                })
                .unwrap()
                .0;
            hash.entry(highest).and_modify(|c| *c += joker_count);
        }

        let values: Vec<u8> = hash.into_values().collect();
        match values.len() {
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
            _ => panic!("cannot happen"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_play_without_joker() {
        let input = "
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";
        let expected = 6440;
        assert_eq!(play_without_joker(input).unwrap(), expected);
    }

    #[test]
    fn test_play_without_joker_2() {
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
        assert_eq!(play_without_joker(input).unwrap(), expected);
    }

    #[test]
    fn test_play_with_joker() {
        let input = "
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";

        let expected = 5905;
        assert_eq!(play_with_joker(input).unwrap(), expected);
    }

    #[test]
    fn test_play_with_joker_2() {
        let input = "
2345A 1
Q2KJJ 13
Q2Q2Q 19
T3T3J 17
T3Q33 11
2345J 3
J345A 2
32T3K 5
T55J5 29
KK677 7
KTJJT 34
QQQJA 31
JJJJJ 37
JAAAA 43
AAAAJ 59
AAAAA 61
2AAAA 23
2JJJJ 53
JJJJ2 41
";

        let expected = 6839;
        assert_eq!(play_with_joker(input).unwrap(), expected);
    }

    #[test]
    fn test_set_ord() -> Result<()> {
        let mut input = [
            Set {
                hand: Hand::parse("32T3K", false)?,
                bid: 765,
            },
            Set {
                hand: Hand::parse("T55J5", false)?,
                bid: 684,
            },
            Set {
                hand: Hand::parse("KK677", false)?,
                bid: 28,
            },
            Set {
                hand: Hand::parse("KTJJT", false)?,
                bid: 220,
            },
            Set {
                hand: Hand::parse("QQQJA", false)?,
                bid: 483,
            },
        ];
        let expected = [
            Set {
                hand: Hand::parse("QQQJA", false)?,
                bid: 483,
            },
            Set {
                hand: Hand::parse("T55J5", false)?,
                bid: 684,
            },
            Set {
                hand: Hand::parse("KK677", false)?,
                bid: 28,
            },
            Set {
                hand: Hand::parse("KTJJT", false)?,
                bid: 220,
            },
            Set {
                hand: Hand::parse("32T3K", false)?,
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
            Hand::parse("32T3K", false)?,
            Hand::parse("T55J5", false)?,
            Hand::parse("KK677", false)?,
            Hand::parse("KTJJT", false)?,
            Hand::parse("QQQJA", false)?,
        ];
        let expected = [
            Hand::parse("QQQJA", false)?,
            Hand::parse("T55J5", false)?,
            Hand::parse("KK677", false)?,
            Hand::parse("KTJJT", false)?,
            Hand::parse("32T3K", false)?,
        ];
        input.sort_unstable();
        assert_eq!(input, expected);
        Ok(())
    }

    #[test]
    fn test_hand_ord_2() -> Result<()> {
        let mut input = [
            Hand::parse("23456", false)?,
            Hand::parse("AKQJT", false)?,
            Hand::parse("22345", false)?,
            Hand::parse("AAKQJ", false)?,
            Hand::parse("22334", false)?,
            Hand::parse("AAKKQ", false)?,
            Hand::parse("22234", false)?,
            Hand::parse("AAAKQ", false)?,
            Hand::parse("22233", false)?,
            Hand::parse("AAAKK", false)?,
            Hand::parse("22223", false)?,
            Hand::parse("AAAAK", false)?,
            Hand::parse("22222", false)?,
            Hand::parse("AAAAA", false)?,
        ];
        let expected = [
            Hand::parse("AAAAA", false)?,
            Hand::parse("22222", false)?,
            Hand::parse("AAAAK", false)?,
            Hand::parse("22223", false)?,
            Hand::parse("AAAKK", false)?,
            Hand::parse("22233", false)?,
            Hand::parse("AAAKQ", false)?,
            Hand::parse("22234", false)?,
            Hand::parse("AAKKQ", false)?,
            Hand::parse("22334", false)?,
            Hand::parse("AAKQJ", false)?,
            Hand::parse("22345", false)?,
            Hand::parse("AKQJT", false)?,
            Hand::parse("23456", false)?,
        ];
        input.sort_unstable();
        assert_eq!(input, expected);
        Ok(())
    }

    #[test]
    fn test_hand_ord_3() -> Result<()> {
        let mut input = [Hand::parse("33322", false)?, Hand::parse("KKKKQ", false)?];
        let expected = [Hand::parse("KKKKQ", false)?, Hand::parse("33322", false)?];
        input.sort_unstable();
        assert_eq!(input, expected);
        Ok(())
    }

    #[test]
    fn test_card_ord() -> Result<()> {
        let mut input = [
            Card::without_joker('2')?,
            Card::without_joker('3')?,
            Card::without_joker('4')?,
            Card::without_joker('5')?,
            Card::without_joker('6')?,
            Card::without_joker('7')?,
            Card::without_joker('8')?,
            Card::without_joker('9')?,
            Card::without_joker('T')?,
            Card::without_joker('J')?,
            Card::without_joker('Q')?,
            Card::without_joker('K')?,
            Card::without_joker('A')?,
        ];
        let expected = [
            Card::without_joker('A')?,
            Card::without_joker('K')?,
            Card::without_joker('Q')?,
            Card::without_joker('J')?,
            Card::without_joker('T')?,
            Card::without_joker('9')?,
            Card::without_joker('8')?,
            Card::without_joker('7')?,
            Card::without_joker('6')?,
            Card::without_joker('5')?,
            Card::without_joker('4')?,
            Card::without_joker('3')?,
            Card::without_joker('2')?,
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
