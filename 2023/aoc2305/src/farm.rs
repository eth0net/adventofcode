use anyhow::{bail, Context, Result};
use std::{iter::Peekable, num::ParseIntError, ops::Range, str::Lines};

#[derive(Debug, Default)]
struct RangeMap {
    source: Range<isize>,
    shift: isize,
}

impl RangeMap {
    fn new(dst_start: isize, src_start: isize, length: isize) -> RangeMap {
        let source = src_start..src_start + length;
        let shift = dst_start - src_start;
        RangeMap { source, shift }
    }

    fn parse(s: &str) -> Result<RangeMap> {
        let values = parse_isize_vec(s)?;
        if values.len() != 3 {
            bail!("expected 3 values per mapper line")
        }
        Ok(RangeMap::new(values[0], values[1], values[2]))
    }

    fn map_value(&self, item: &isize) -> Option<isize> {
        match self.source.contains(item) {
            true => Some(item + self.shift),
            false => None,
        }
    }
}

#[derive(Debug, Default)]
pub struct Almanac {
    seeds: Vec<isize>,
    seed_soil: Vec<RangeMap>,
    soil_fertilizer: Vec<RangeMap>,
    fertilizer_water: Vec<RangeMap>,
    water_light: Vec<RangeMap>,
    light_temperature: Vec<RangeMap>,
    temperature_humidity: Vec<RangeMap>,
    humidity_location: Vec<RangeMap>,
}

impl Almanac {
    pub fn parse(s: &str) -> Result<Almanac> {
        let mut almanac = Almanac::default();
        let mut lines = s.lines().peekable();
        while let Some(line) = lines.next() {
            match line {
                l if l.starts_with("seeds: ") => {
                    let (_, seeds) = l.split_once(": ").unwrap();
                    almanac.seeds = parse_isize_vec(seeds).with_context(|| "parsing seeds")?;
                }
                "seed-to-soil map:" => {
                    almanac.seed_soil = parse_mappers(&mut lines)?;
                }
                "soil-to-fertilizer map:" => {
                    almanac.soil_fertilizer = parse_mappers(&mut lines)?;
                }
                "fertilizer-to-water map:" => {
                    almanac.fertilizer_water = parse_mappers(&mut lines)?;
                }
                "water-to-light map:" => {
                    almanac.water_light = parse_mappers(&mut lines)?;
                }
                "light-to-temperature map:" => {
                    almanac.light_temperature = parse_mappers(&mut lines)?;
                }
                "temperature-to-humidity map:" => {
                    almanac.temperature_humidity = parse_mappers(&mut lines)?;
                }
                "humidity-to-location map:" => {
                    almanac.humidity_location = parse_mappers(&mut lines)?;
                }
                _ => {}
            }
        }
        Ok(almanac)
    }

    pub fn closest(&self) -> Option<isize> {
        let mut locations: Vec<isize> = self
            .seeds
            .iter()
            .map(|s| with_mappers(&self.seed_soil, s))
            .map(|s| with_mappers(&self.soil_fertilizer, &s))
            .map(|f| with_mappers(&self.fertilizer_water, &f))
            .map(|w| with_mappers(&self.water_light, &w))
            .map(|l| with_mappers(&self.light_temperature, &l))
            .map(|t| with_mappers(&self.temperature_humidity, &t))
            .map(|h| with_mappers(&self.humidity_location, &h))
            .collect();
        locations.sort_unstable();
        Some(*locations.first()?)
    }
}

fn with_mappers(maps: &[RangeMap], value: &isize) -> isize {
    maps.iter()
        .find_map(|m| m.map_value(value))
        .unwrap_or(*value)
}

fn parse_isize_vec(s: &str) -> Result<Vec<isize>, anyhow::Error> {
    s.split_ascii_whitespace()
        .map(str::parse::<isize>)
        .collect::<Result<Vec<isize>, ParseIntError>>()
        .with_context(|| "parsing isize vec from string")
}

fn parse_mappers(lines: &mut Peekable<Lines<'_>>) -> Result<Vec<RangeMap>> {
    let mut mappers = vec![];
    while let Some(line) = lines.next_if(|l| !l.is_empty()) {
        mappers.push(RangeMap::parse(line)?);
    }
    Ok(mappers)
}

#[cfg(test)]
mod tests {
    use super::*;

    use anyhow::{anyhow, Result};

    #[test]
    fn test_almanac_parse_closest() -> Result<()> {
        let input = "
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4

";
        let expected = 35;
        assert_eq!(
            Almanac::parse(input)?
                .closest()
                .ok_or(anyhow!("no value for closest"))?,
            expected
        );
        Ok(())
    }
}
