pub fn enclosed(s: &str) -> usize {
    let rows: Vec<Vec<Tile>> = s
        .lines()
        .enumerate()
        .filter(|(_, line)| !line.is_empty())
        .map(|(l, line)| {
            line.char_indices()
                .map(|(c, char)| {
                    let mut tile = Tile::from(char);
                    tile.location = Location::new(l, c);
                    tile
                })
                .collect()
        })
        .collect();

    let mut grid = Grid::new(rows);

    'outer: for l in 0..grid.rows.len() {
        for c in 0..grid.rows[l].len() {
            let location = Location::new(l, c);
            if grid.tile(location).char != 'S' {
                continue;
            }

            let north = match l.checked_sub(1).map(|l| grid.rows.get(l).map(|l| l.get(c))) {
                Some(p) if p.is_some_and(|p| p.is_some()) => {
                    ['|', '7', 'F'].contains(&p.unwrap().unwrap().char)
                }
                _ => false,
            };
            let south = match grid.rows.get(l + 1).map(|l| l.get(c)) {
                Some(p) if p.is_some() => ['|', 'L', 'J'].contains(&p.unwrap().char),
                _ => false,
            };
            let east = match grid.rows.get(l).map(|l| l.get(c + 1)) {
                Some(p) if p.is_some() => ['-', '7', 'J'].contains(&p.unwrap().char),
                _ => false,
            };
            let west = match c.checked_sub(1).map(|c| grid.rows.get(l).map(|l| l.get(c))) {
                Some(p) if p.is_some_and(|p| p.is_some()) => {
                    ['-', 'L', 'F'].contains(&p.unwrap().unwrap().char)
                }
                _ => false,
            };

            let pipe = match [north, south, east, west] {
                [true, true, false, false] => '|',
                [false, false, true, true] => '-',
                [true, false, true, false] => 'L',
                [true, false, false, true] => 'J',
                [false, true, false, true] => '7',
                [false, true, true, false] => 'F',
                _ => continue,
            };

            grid.start = Tile::from(pipe);
            grid.start.location = Location::new(l, c);
            grid.start.main_loop = true;

            grid.tile_mut(location).main_loop = true;

            break 'outer;
        }
    }

    let start_location = grid.start.location.to_owned();

    let mut tile = &mut grid.start;
    let mut direction = tile.connections().0;
    let mut next_location = tile.location.next(&direction);

    let mut main_loop = vec![start_location];
    while next_location != start_location {
        main_loop.push(next_location);

        tile = &mut grid.rows[next_location.line][next_location.char];
        direction = tile.next_direction(&direction);
        next_location = tile.location.next(&direction);

        tile.main_loop = true;
    }

    for row in grid.rows {
        eprintln!("{}", row.iter().map(|c| c.char).collect::<String>());
    }

    for location in main_loop {}

    0
}

#[derive(Debug, Default)]
struct Grid {
    rows: Vec<Vec<Tile>>,
    start: Tile,
}

impl Grid {
    fn new(rows: Vec<Vec<Tile>>) -> Grid {
        Grid {
            rows,
            ..Default::default()
        }
    }

    fn tile(&self, location: Location) -> &Tile {
        &self.rows[location.line][location.char]
    }

    fn tile_mut(&mut self, location: Location) -> &mut Tile {
        &mut self.rows[location.line][location.char]
    }
}

#[derive(Debug)]
struct Row {
    tiles: Vec<Tile>,
}

#[derive(Debug, Default)]
struct Tile {
    char: char,
    main_loop: bool,
    location: Location,
}

impl Tile {
    fn connections(&self) -> (Direction, Direction) {
        match self.char {
            '|' => (Direction::North, Direction::South),
            '-' => (Direction::West, Direction::East),
            'L' => (Direction::North, Direction::East),
            'J' => (Direction::North, Direction::West),
            '7' => (Direction::West, Direction::South),
            'F' => (Direction::East, Direction::South),
            _ => panic!("bad pipe {}", self.char),
        }
    }

    fn next_direction(&self, last_move: &Direction) -> Direction {
        let directions = self.connections();
        match last_move.is_opposite(&directions.0) {
            true => directions.1,
            false => directions.0,
        }
    }
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        Tile {
            char: value,
            main_loop: bool::default(),
            location: Location::default(),
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
struct Location {
    line: usize,
    char: usize,
}

impl Location {
    fn new(line: usize, char: usize) -> Location {
        Location { line, char }
    }

    fn next(&self, direction: &Direction) -> Location {
        let mut location = self.to_owned();
        match direction {
            Direction::North => location.line -= 1,
            Direction::South => location.line += 1,
            Direction::East => location.char += 1,
            Direction::West => location.char -= 1,
        };
        location
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }

    fn is_opposite(&self, other: &Direction) -> bool {
        &self.opposite() == other
    }
}

#[cfg(test)]
mod tests {
    use test_case::case;

    use super::*;

    const EXAMPLE_ONE: &str = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........

";

    #[case(EXAMPLE_ONE, 4 ; "example one")]
    fn test_enclosed(input: &str, expected: usize) {
        assert_eq!(enclosed(input), expected);
    }
}
