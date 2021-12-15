use itertools::Itertools;
use std::collections::HashMap;
use std::collections::VecDeque;

/// Evolve space for 100 steps and count the number of flashes
pub fn part1(lines: &[String]) -> usize {
    let mut space = Space::parse(lines);
    (0..100).map(|_| space.step()).sum()
}

/// Find the first step where every octopus flashes together
pub fn part2(lines: &[String]) -> usize {
    let mut space = Space::parse(lines);
    (1..)
        .map(|i| (i, space.step()))
        .skip_while(|&(_, x)| x < 100)
        .map(|(i, _)| i)
        .next()
        .unwrap()
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct Space {
    space: HashMap<(usize, usize), u8>,
}

impl Space {
    fn new(space: HashMap<(usize, usize), u8>) -> Self {
        Self { space }
    }

    fn parse(lines: &[String]) -> Self {
        let space = lines
            .iter()
            .enumerate()
            .flat_map(|(i, line)| {
                line.bytes()
                    .enumerate()
                    .map(|(j, byte)| ((i, j), byte - b'0'))
                    .collect_vec()
            })
            .collect();

        Self::new(space)
    }

    fn step(&mut self) -> usize {
        let mut queue: VecDeque<(usize, usize)> = (0..10).cartesian_product(0..10).collect();
        let mut flashed: Vec<(usize, usize)> = Vec::new();

        while let Some(point) = queue.pop_front() {
            if self.space[&point] > 9 {
                continue;
            }

            self.space.entry(point).and_modify(|level| *level += 1);

            if self.space[&point] > 9 {
                flashed.push(point);
                for neighbor in neighbors(point) {
                    queue.push_back(neighbor)
                }
            }
        }

        let flashes = flashed.len();
        for point in flashed {
            self.space.insert(point, 0);
        }
        flashes
    }
}

fn neighbors(point: (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
    let range = 0..10;
    let row = point.0 as isize;
    let col = point.1 as isize;
    (-1..=1)
        .cartesian_product(-1..=1)
        .filter(|&(dy, dx)| dy != 0 || dx != 0)
        .filter_map(move |(dy, dx)| {
            let point = (row + dy, col + dx);
            if range.contains(&point.0) && range.contains(&point.1) {
                return Some((point.0 as usize, point.1 as usize));
            }
            None
        })
}

check!(ex 1 = 1656, ex 2 = 195, part 1 = 1613, part 2 = 510);
bench!(part 1, part 2);
