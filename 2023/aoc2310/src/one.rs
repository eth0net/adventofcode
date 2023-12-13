pub fn distance_to_end(s: &str) -> usize {
    let grid: Vec<Vec<char>> = s
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect();

    let mut start = ((0, 0), char::default());
    'outer: for l in 0..grid.len() {
        for c in 0..grid[l].len() {
            if grid[l][c] != 'S' {
                continue;
            }

            let north = match l.checked_sub(1).map(|l| grid.get(l).map(|l| l.get(c))) {
                Some(p) if p.is_some_and(|p| p.is_some()) => {
                    ['|', '7', 'F'].contains(p.unwrap().unwrap())
                }
                _ => false,
            };
            let south = match grid.get(l + 1).map(|l| l.get(c)) {
                Some(p) if p.is_some() => ['|', 'L', 'J'].contains(p.unwrap()),
                _ => false,
            };
            let east = match grid.get(l).map(|l| l.get(c + 1)) {
                Some(p) if p.is_some() => ['-', '7', 'J'].contains(p.unwrap()),
                _ => false,
            };
            let west = match c.checked_sub(1).map(|c| grid.get(l).map(|l| l.get(c))) {
                Some(p) if p.is_some_and(|p| p.is_some()) => {
                    ['-', 'L', 'F'].contains(p.unwrap().unwrap())
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

            start = ((l, c), pipe);

            break 'outer;
        }
    }

    let next = connected(start.1, start.0);
    let mut one = (start, (next.0, grid[next.0 .0][next.0 .1]));
    let mut two = (start, (next.1, grid[next.1 .0][next.1 .1]));
    let mut steps = 1;

    while one.1 != two.1 {
        let next_one = next_point(one.0 .0, one.1);
        let next_two = next_point(two.0 .0, two.1);

        one = (one.1, (next_one, grid[next_one.0][next_one.1]));
        two = (two.1, (next_two, grid[next_two.0][next_two.1]));

        steps += 1;
    }

    steps
}

fn next_point(last: (usize, usize), current: ((usize, usize), char)) -> (usize, usize) {
    let connected = connected(current.1, current.0);
    match connected.0 == last {
        true => connected.1,
        false => connected.0,
    }
}

fn connected(p: char, loc: (usize, usize)) -> ((usize, usize), (usize, usize)) {
    let (l, c) = loc;
    match p {
        '|' => ((l - 1, c), (l + 1, c)),
        '-' => ((l, c - 1), (l, c + 1)),
        'L' => ((l - 1, c), (l, c + 1)),
        'J' => ((l - 1, c), (l, c - 1)),
        '7' => ((l, c - 1), (l + 1, c)),
        'F' => ((l, c + 1), (l + 1, c)),
        _ => panic!("bad pipe {}", p),
    }
}

#[cfg(test)]
mod tests {
    use test_case::case;

    use super::*;

    const EXAMPLE_ONE: &str = ".....
.S-7.
.|.|.
.L-J.
.....

";

    const EXAMPLE_TWO: &str = "
..F7.
.FJ|.
SJ.L7
|F--J
LJ...

";

    #[case(EXAMPLE_ONE, 4 ; "example one")]
    #[case(EXAMPLE_TWO, 8 ; "example two")]
    fn test_distance_to_end(input: &str, expected: usize) {
        assert_eq!(distance_to_end(input), expected);
    }
}
