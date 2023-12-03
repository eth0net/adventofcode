#[derive(Debug, Default)]
pub struct Engine {
    missing_parts: Vec<usize>,
    symbols: Vec<Location>,
}

impl Engine {
    pub fn parse(s: &str) -> anyhow::Result<Engine> {
        let mut engine = Engine::default();

        for (l, line) in s.lines().enumerate() {
            for (c, char) in line.char_indices() {
                if char.is_ascii_punctuation() && char != '.' {
                    engine.symbols.push(Location::new(l, c))
                }
            }
        }

        for (l, line) in s.lines().enumerate() {
            let mut chars = line.char_indices().peekable();
            while let Some((c, char)) = chars.next() {
                if char.is_ascii_digit() {
                    let location = Location::new(l, c);

                    let adjacent_to_symbol = engine.symbols.iter().any(|l| l.adjacent(&location));

                    if !adjacent_to_symbol {
                        continue;
                    }

                    // run backwards and forwards in chars to collect full number
                    // then skip to end of number and continue chars iteration

                    let mut part = char.to_string();

                    if let Some(left) = line.get(0..c) {
                        let i = left
                            .rfind(|c: char| !c.is_ascii_digit())
                            .map(|i| i + 1)
                            .unwrap_or_default();
                        line.get(i..c).map(|s| part.insert_str(0, s));
                    }

                    while let Some((_, char)) = chars.next_if(|(_, c)| c.is_ascii_digit()) {
                        part.push(char)
                    }

                    engine.missing_parts.push(part.parse()?)
                }
            }
        }

        Ok(engine)
    }

    pub fn sum_missing_parts(&self) -> usize {
        self.missing_parts.iter().fold(0, |a, b| a + b)
    }
}

#[derive(Debug, Default, Eq, PartialEq)]
struct Location {
    line: usize,
    char: usize,
}

impl Location {
    fn new(line: usize, char: usize) -> Location {
        Location { line, char }
    }

    fn adjacent(&self, other: &Location) -> bool {
        let line_adjacent = self.line.abs_diff(other.line) <= 1;
        let char_adjacent = self.char.abs_diff(other.char) <= 1;
        self != other && line_adjacent && char_adjacent
    }
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
            Location::new(1, 3),
            Location::new(3, 6),
            Location::new(4, 3),
            Location::new(5, 5),
            Location::new(8, 3),
            Location::new(8, 5),
        ];
        let expected_parts = vec![467, 35, 633, 617, 592, 755, 664, 598];
        let expected_sum = 4361;

        let engine = Engine::parse(input)?;

        assert_eq!(expected_symbols, engine.symbols);
        assert_eq!(expected_parts, engine.missing_parts);
        assert_eq!(expected_sum, engine.sum_missing_parts());

        Ok(())
    }
}
