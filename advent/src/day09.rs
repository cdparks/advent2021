use itertools::Itertools;
use std::collections::HashSet;
use std::collections::VecDeque;

/// Find low points and calculate total risk.
pub fn part1(lines: &[String]) -> usize {
    let map = parse_map(lines);
    low_points(&map)
        .iter()
        .map(|height| 1 + *height as usize)
        .sum()
}

/// Find 3 largest basins and multiply their sizes together.
pub fn part2(lines: &[String]) -> u64 {
    let map = parse_map(lines);
    low_point_coords(&map)
        .iter()
        .map(|(i, j)| basin_area(&map, *i, *j))
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
pub fn low_points(map: &[Vec<u8>]) -> Vec<u8> {
    low_point_impl(map)
        .iter()
        .map(|(height, _)| *height)
        .collect()
}

/// Find lowest point coordinates.
pub fn low_point_coords(map: &[Vec<u8>]) -> Vec<(usize, usize)> {
    low_point_impl(map)
        .iter()
        .map(|(_, point)| *point)
        .collect()
}

/// Find lowest point heights and coordinates.
pub fn low_point_impl(map: &[Vec<u8>]) -> Vec<(u8, (usize, usize))> {
    let mut points = Vec::new();
    for (i, row) in map.iter().enumerate() {
        for (j, &height) in row.iter().enumerate() {
            let min = neighbors(map, i, j).into_iter().min().unwrap();
            if height < min {
                points.push((height, (i, j)));
            }
        }
    }
    points
}

/// Find point's neighbors in cardinal directions.
pub fn neighbors(map: &[Vec<u8>], row: usize, col: usize) -> Vec<u8> {
    neighbor_coords(map, row, col)
        .iter()
        .map(|(i, j)| map[*i][*j])
        .collect()
}

/// Find coordinates of point's neighbors in cardinal directions.
pub fn neighbor_coords(map: &[Vec<u8>], row: usize, col: usize) -> Vec<(usize, usize)> {
    let max_row = map.len() - 1;
    let max_col = map[0].len() - 1;
    let mut coords = Vec::with_capacity(4);
    if row > 0 {
        coords.push((row - 1, col));
    }
    if col > 0 {
        coords.push((row, col - 1));
    }
    if row < max_row {
        coords.push((row + 1, col));
    }
    if col < max_col {
        coords.push((row, col + 1));
    }
    coords
}

fn parse_map(lines: &[String]) -> Vec<Vec<u8>> {
    lines
        .iter()
        .map(|line| line.bytes().map(|byte| byte - b'0').collect())
        .collect()
}

check!(ex 1 = 15, ex 2 = 1134, part 1 = 498, part 2 = 1071000);
bench!(part 1, part 2);
