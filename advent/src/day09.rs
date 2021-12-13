use itertools::Itertools;
use std::collections::HashSet;
use std::collections::VecDeque;

/// Find low points and calculate total risk.
pub fn part1(lines: &[String]) -> usize {
    let map = parse_map(lines);
    low_points(&map).map(|height| 1 + height as usize).sum()
}

/// Find 3 largest basins and multiply their sizes together.
pub fn part2(lines: &[String]) -> u64 {
    let map = parse_map(lines);
    low_point_coords(&map)
        .map(|(i, j)| basin_area(&map, i, j))
        .sorted()
        .rev()
        .take(3)
        .product()
}

/// Breadth-first-search from lowest point.
///
/// Stops at boundaries of map or points of height 9.
pub fn basin_area(map: &[Vec<u8>], row: usize, col: usize) -> u64 {
    let mut queue = VecDeque::from([(row, col)]);
    let mut seen = HashSet::new();
    let mut area = 0;

    while let Some((i, j)) = queue.pop_front() {
        if seen.contains(&(i, j)) || map[i][j] == 9 {
            continue;
        }

        area += 1;
        seen.insert((i, j));
        queue.extend(neighbor_coords(map, i, j))
    }

    area
}

/// Find lowest points
pub fn low_points(map: &[Vec<u8>]) -> impl Iterator<Item = u8> + '_ {
    low_point_impl(map).map(|(height, _)| height)
}

/// Find lowest point coordinates.
pub fn low_point_coords(map: &[Vec<u8>]) -> impl Iterator<Item = (usize, usize)> + '_ {
    low_point_impl(map).map(|(_, point)| point)
}

/// Find lowest point heights and coordinates.
pub fn low_point_impl(map: &[Vec<u8>]) -> impl Iterator<Item = (u8, (usize, usize))> + '_ {
    map.iter().enumerate().flat_map(move |(i, row)| {
        row.iter().enumerate().filter_map(move |(j, &height)| {
            let min = neighbors(map, i, j).min().unwrap();
            if height < min {
                return Some((height, (i, j)));
            }
            None
        })
    })
}

/// Find point's neighbors in each cardinal direction.
pub fn neighbors(map: &[Vec<u8>], row: usize, col: usize) -> impl Iterator<Item = u8> + '_ {
    neighbor_coords(map, row, col).map(move |(i, j)| map[i][j])
}

/// Find coordinates of point's neighbors in cardinal directions.
pub fn neighbor_coords(
    map: &[Vec<u8>],
    row: usize,
    col: usize,
) -> impl Iterator<Item = (usize, usize)> {
    let rows = 0..map.len() as isize;
    let cols = 0..map[0].len() as isize;
    [(-1, 0), (0, -1), (1, 0), (0, 1)]
        .iter()
        .filter_map(move |(dy, dx)| {
            let point = (row as isize + dy, col as isize + dx);
            if rows.contains(&point.0) && cols.contains(&point.1) {
                return Some((point.0 as usize, point.1 as usize));
            }
            None
        })
}

fn parse_map(lines: &[String]) -> Vec<Vec<u8>> {
    lines
        .iter()
        .map(|line| line.bytes().map(|byte| byte - b'0').collect())
        .collect()
}

check!(ex 1 = 15, ex 2 = 1134, part 1 = 498, part 2 = 1071000);
bench!(part 1, part 2);
