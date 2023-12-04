use std::ops::Range;

#[derive(Debug, Default)]
pub struct Engine {
    parts: Vec<Part>,
    symbols: Vec<Symbol>,
}

impl Engine {
    pub fn parse(s: &str) -> anyhow::Result<Engine> {
        let mut engine = Engine::default();

        for (l, line) in s.lines().enumerate() {
            for (c, char) in line.char_indices() {
                if char.is_ascii_punctuation() && char != '.' {
                    let location = Location::from_usize(l, c);
                    let symbol = Symbol::new(char, location);
                    engine.symbols.push(symbol)
                }
            }
        }

        for (l, line) in s.lines().enumerate() {
            let mut chars = line.char_indices().peekable();
            while let Some((c, char)) = chars.next() {
                if char.is_ascii_digit() {
                    let mut location = Location::from_usize(l, c);

                    let adjacent_to_symbol = engine
                        .symbols
                        .iter()
                        .any(|l| l.location.adjacent(&location));

                    if !adjacent_to_symbol {
                        continue;
                    }

                    let mut part = char.to_string();

                    if let Some(left) = line.get(0..c) {
                        let i = left
                            .rfind(|c: char| !c.is_ascii_digit())
                            .map(|i| i + 1)
                            .unwrap_or_default();
                        line.get(i..c).map(|s| part.insert_str(0, s));
                        location.range.start = i;
                    }

                    while let Some((_, char)) = chars.next_if(|(_, c)| c.is_ascii_digit()) {
                        part.push(char);
                        location.range.end += 1;
                    }

                    engine.parts.push(Part::new(part.parse()?, location))
                }
            }
        }

        Ok(engine)
    }

    pub fn sum_parts(&self) -> usize {
        self.parts.iter().fold(0, |a, b| a + b.id)
    }

    pub fn sum_gear_ratios(&self) -> usize {
        let mut sum = 0;

        for symbol in &self.symbols {
            if symbol.char != '*' {
                continue;
            }

            let adjacent_numbers = self
                .parts
                .iter()
                .filter(|p| p.location.adjacent(&symbol.location))
                .map(|p| p.id)
                .collect::<Vec<usize>>();

            if adjacent_numbers.len() != 2 {
                continue;
            }

            sum += adjacent_numbers[0] * adjacent_numbers[1]
        }

        sum
    }
}

#[derive(Debug, Default, Eq, PartialEq)]
struct Part {
    id: usize,
    location: Location,
}

impl Part {
    fn new(id: usize, location: Location) -> Part {
        Part { id, location }
    }
}

#[derive(Debug, Default, Eq, PartialEq)]
struct Symbol {
    char: char,
    location: Location,
}

impl Symbol {
    fn new(char: char, location: Location) -> Symbol {
        Symbol { char, location }
    }
}

#[derive(Debug, Default, Eq, PartialEq)]
struct Location {
    line: usize,
    range: Range<usize>,
}

impl Location {
    fn from_usize(line: usize, index: usize) -> Location {
        let range = index..index + 1;
        Location { line, range }
    }

    fn adjacent(&self, other: &Location) -> bool {
        let line_adjacent = self.line.abs_diff(other.line) <= 1;
        let char_adjacent = ranges_adjacent(&self.range, &other.range);
        self != other && line_adjacent && char_adjacent
    }
}

fn ranges_adjacent(one: &Range<usize>, two: &Range<usize>) -> bool {
    let start = one.start.checked_sub(1).unwrap_or(one.start);
    let end = one.end.checked_add(1).unwrap_or(one.end);
    (start..end).any(|i| two.contains(&i))
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use super::*;

    #[test]
    fn test_engine_parse() -> Result<()> {
        let input = "
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
            .trim();
        let expected_symbols = vec![
            Symbol::new('*', Location::from_usize(1, 3)),
            Symbol::new('#', Location::from_usize(3, 6)),
            Symbol::new('*', Location::from_usize(4, 3)),
            Symbol::new('+', Location::from_usize(5, 5)),
            Symbol::new('$', Location::from_usize(8, 3)),
            Symbol::new('*', Location::from_usize(8, 5)),
        ];
        let expected_parts = vec![
            Part {
                id: 467,
                location: Location::from_range_usize(0, 0..3),
            },
            Part {
                id: 35,
                location: Location::from_range_usize(2, 2..4),
            },
            Part {
                id: 633,
                location: Location::from_range_usize(2, 6..9),
            },
            Part {
                id: 617,
                location: Location::from_range_usize(4, 0..3),
            },
            Part {
                id: 592,
                location: Location::from_range_usize(6, 2..5),
            },
            Part {
                id: 755,
                location: Location::from_range_usize(7, 6..9),
            },
            Part {
                id: 664,
                location: Location::from_range_usize(9, 1..4),
            },
            Part {
                id: 598,
                location: Location::from_range_usize(9, 5..8),
            },
        ];
        let expected_sum = 4361;
        let expected_ratio = 467835;

        let engine = Engine::parse(input)?;

        assert_eq!(expected_symbols, engine.symbols, "checking symbols");
        assert_eq!(expected_parts, engine.parts, "checking parts");
        assert_eq!(expected_sum, engine.sum_parts(), "checking sum of parts");
        assert_eq!(
            expected_ratio,
            engine.sum_gear_ratios(),
            "checking gear ratios"
        );

        Ok(())
    }
}
