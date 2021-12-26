/// Find the step where no more moves are possible.
pub fn part1(lines: &[String]) -> usize {
    solve(parse(lines))
}

/// Parse into a grid.
fn parse(lines: &[String]) -> Vec<Vec<Facing>> {
    lines
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '>' => Facing::East,
                    'v' => Facing::South,
                    _ => Facing::Empty,
                })
                .collect()
        })
        .collect()
}

/// Iterate the map until no more moves are possible.
fn solve(mut map: Vec<Vec<Facing>>) -> usize {
    let mut steps = 0;
    loop {
        let east = step(&mut map, Facing::East, (0, 1));
        let south = step(&mut map, Facing::South, (1, 0));
        steps += 1;
        if !east && !south {
            break;
        }
    }
    steps
}

/// Attempt to move each matching item as specified, returning whether
/// any moves took place.
fn step(map: &mut Vec<Vec<Facing>>, direction: Facing, change: (usize, usize)) -> bool {
    let (dy, dx) = change;
    let rows = map.len();
    let cols = map[0].len();
    let mut changes = Vec::new();

    for (i, row) in map.iter().enumerate() {
        for (j, &facing) in row.iter().enumerate() {
            let (r, c) = ((i + dy) % rows, (j + dx) % cols);
            if facing == direction && map[r][c] == Facing::Empty {
                changes.push(((i, j), (r, c)));
            }
        }
    }

    let changed = changes.len() > 0;
    for ((i, j), (r, c)) in changes.into_iter() {
        map[r][c] = direction;
        map[i][j] = Facing::Empty;
    }
    changed
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
/// Tiles are empty, facing east, or facing south.
pub enum Facing {
    /// Unoccupied tile.
    Empty,
    /// East facing sea cucumber.
    East,
    /// South facing sea cucumber.
    South,
}

check!(ex 1 = 58, part 1 = 532);
bench!(part 1);
