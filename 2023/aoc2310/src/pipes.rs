// find S
// check 4 directions to determine S shape
// follow pipe in both directions from S counting steps
// find the largest distance

#[derive(Debug)]
struct Map<'a> {
    start: (usize, usize),
    tiles: &'a [&'a [Tile]],
}

#[derive(Debug)]
struct Tile {
    char: char,
    pipe: Option<Pipe>,
}

#[derive(Debug, Default)]
struct Pipe {
    shape: char,
    north: bool,
    south: bool,
    east: bool,
    west: bool,
}

impl Pipe {
    fn from_directions(north: bool, south: bool, east: bool, west: bool) -> Option<Pipe> {
        let shape = match [north, south, east, west] {
            [true, true, false, false] => '|',
            [false, false, true, true] => '-',
            [true, false, true, false] => 'L',
            [true, false, false, true] => 'J',
            [false, true, false, true] => '7',
            [false, true, true, false] => 'F',
            _ => panic!("can't happen"),
        };
        Some(Pipe {
            shape,
            north,
            south,
            east,
            west,
        })
    }

    fn try_new(c: char) -> Option<Pipe> {
        let mut pipe = Pipe::default();
        match c {
            '|' => Pipe {},
        }
    }
}
