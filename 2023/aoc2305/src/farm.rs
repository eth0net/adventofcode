use anyhow::{bail, Context, Result};
use std::{cmp::Ordering, iter::Peekable, num::ParseIntError, ops::Range, str::Lines};
#[cfg(concurrency)]
use std::{sync::mpsc::channel, sync::Arc, thread::spawn};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
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

impl PartialOrd for RangeMap {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for RangeMap {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let start = self.source.start.cmp(&other.source.start);
        if start != Ordering::Equal {
            return start;
        }

        let end = self.source.end.cmp(&other.source.end);
        if end != Ordering::Equal {
            return end;
        }

        self.shift.cmp(&other.shift)
    }
}

#[derive(Clone, Debug, Default)]
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
    pub fn with_seed_list(s: &str) -> Result<Almanac> {
        Almanac::parse(s, parse_isize_vec)
    }

    pub fn with_seed_ranges(s: &str) -> Result<Almanac> {
        Almanac::parse(s, parse_isize_range_seq)
    }

    fn parse(s: &str, seed_parser: fn(&str) -> Result<Vec<isize>>) -> Result<Almanac> {
        println!("starting almanac parse");
        let mut almanac = Almanac::default();
        let mut lines = s.lines().peekable();
        while let Some(line) = lines.next() {
            match line {
                l if l.starts_with("seeds: ") => {
                    println!("found seeds");
                    let (_, seeds) = l.split_once(": ").unwrap();
                    almanac.seeds = seed_parser(seeds).with_context(|| "parsing seeds")?;
                }
                "seed-to-soil map:" => {
                    println!("found seed to soil map");
                    almanac.seed_soil = parse_mappers(&mut lines)?;
                }
                "soil-to-fertilizer map:" => {
                    println!("found soil to fertilizer map");
                    almanac.soil_fertilizer = parse_mappers(&mut lines)?;
                }
                "fertilizer-to-water map:" => {
                    println!("found fertilizer to water map");
                    almanac.fertilizer_water = parse_mappers(&mut lines)?;
                }
                "water-to-light map:" => {
                    println!("found water to light map");
                    almanac.water_light = parse_mappers(&mut lines)?;
                }
                "light-to-temperature map:" => {
                    println!("found light to temperature map");
                    almanac.light_temperature = parse_mappers(&mut lines)?;
                }
                "temperature-to-humidity map:" => {
                    println!("found temperature to humidity map");
                    almanac.temperature_humidity = parse_mappers(&mut lines)?;
                }
                "humidity-to-location map:" => {
                    println!("found humidity to location map");
                    almanac.humidity_location = parse_mappers(&mut lines)?;
                }
                _ => {}
            }
        }
        Ok(almanac)
    }

    #[cfg(concurrency)]
    pub fn closest(&self) -> Option<isize> {
        let (tx, rx) = channel();

        let almanac = Arc::new(self.clone());

        println!("Seed count: {}", self.seeds.len());

        for (count, seed) in self.seeds.iter().enumerate() {
            let tx = tx.clone();
            let count = count.to_owned();
            let seed = seed.to_owned();
            let almanac = Arc::clone(&almanac);
            spawn(move || -> Result<()> {
                let soil = with_mappers(&almanac.seed_soil, &seed);
                let fert = with_mappers(&almanac.soil_fertilizer, &soil);
                let water = with_mappers(&almanac.fertilizer_water, &fert);
                let light = with_mappers(&almanac.water_light, &water);
                let temp = with_mappers(&almanac.light_temperature, &light);
                let hum = with_mappers(&almanac.temperature_humidity, &temp);
                let loc = with_mappers(&almanac.humidity_location, &hum);
                tx.send(loc).with_context(|| "transmitting thread data")?;
                println!("thread {} complete", count);
                Ok(())
            });
        }
        drop(tx);

        let mut locations: Vec<isize> = rx.iter().collect();

        locations.sort_unstable();
        Some(*locations.first()?)
    }

    #[cfg(not(concurrency))]
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
    // let (tx, rx) = channel();
    // for map in maps {
    //     let map = map.to_owned();
    //     let value = value.to_owned();
    //     let tx = tx.clone();
    //     spawn(move || -> Result<()> {
    //         if let Some(value) = map.map_value(&value) {
    //             tx.send(value)?;
    //         }
    //         Ok(())
    //     });
    // }
    // drop(tx);
    // rx.recv().unwrap_or(*value)

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

fn parse_isize_range_seq(s: &str) -> Result<Vec<isize>, anyhow::Error> {
    let split = parse_isize_vec(s)?;
    if split.len() % 2 != 0 {
        bail!("invalid seed range sequence")
    }
    let seeds = split
        .chunks_exact(2)
        .flat_map(|chunk| chunk[0]..(chunk[0] + chunk[1]))
        .collect();
    Ok(seeds)
}

fn parse_mappers(lines: &mut Peekable<Lines<'_>>) -> Result<Vec<RangeMap>> {
    let mut mappers = vec![];
    while let Some(line) = lines.next_if(|l| !l.is_empty()) {
        mappers.push(RangeMap::parse(line)?);
    }
    mappers.sort_unstable();
    Ok(mappers)
}

#[cfg(test)]
mod tests {
    use super::*;

    use anyhow::Result;

    const INPUT: &str = "
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

    #[test]
    fn test_almanac_seed_list_closest() -> Result<()> {
        let expected = Some(35);
        assert_eq!(Almanac::with_seed_list(INPUT)?.closest(), expected);
        Ok(())
    }

    #[test]
    fn test_almanace_seed_range_closest() -> Result<()> {
        let expected = Some(46);
        assert_eq!(Almanac::with_seed_ranges(INPUT)?.closest(), expected);
        Ok(())
    }
}
