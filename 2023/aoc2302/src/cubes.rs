use anyhow::{bail, Context, Result};

#[derive(Debug, Default)]
pub struct Game {
    pub id: usize,
    pub rounds: Vec<Set>,
}

impl Game {
    pub(crate) fn parse(s: &str) -> Result<Game> {
        let (game, rounds_str) = s.split_once(':').with_context(|| "splitting game line")?;

        let id = game
            .strip_prefix("Game ")
            .with_context(|| "stripping game prefix")?
            .parse()
            .with_context(|| "parsing game id")?;

        let mut rounds = vec![];
        for round in rounds_str.split("; ") {
            rounds.push(Set::parse(round).with_context(|| "parsing round")?)
        }

        Ok(Game { id, rounds })
    }

    pub fn possible(&self, bag: &Set) -> bool {
        self.rounds.iter().all(|round| round.possible(bag))
    }
}

#[derive(Debug, Default)]
pub struct Set {
    pub red: usize,
    pub green: usize,
    pub blue: usize,
}

impl Set {
    pub fn new(red: usize, green: usize, blue: usize) -> Set {
        Set { red, green, blue }
    }

    fn parse(s: &str) -> Result<Set> {
        let mut round = Set::default();

        for mut cube in s.split(", ").map(|f| f.split_ascii_whitespace()) {
            let count = cube
                .next()
                .with_context(|| "getting count from iterator")?
                .parse()
                .with_context(|| "parsing count")?;

            let colour = cube
                .next()
                .with_context(|| "getting colour from iterator")?;

            match colour {
                "red" => round.red = count,
                "green" => round.green = count,
                "blue" => round.blue = count,
                _ => bail!("invalid colour: {}", colour),
            }
        }

        Ok(round)
    }

    fn possible(&self, bag: &Set) -> bool {
        self.red <= bag.red && self.green <= bag.green && self.blue <= bag.blue
    }

    pub fn power(&self) -> usize {
        self.red * self.green * self.blue
    }
}
