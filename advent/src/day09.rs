use itertools::Itertools;
use std::collections::VecDeque;
use std::collections::{HashMap, HashSet};

use image::codecs::gif::GifEncoder;
use image::imageops::{resize, FilterType};
use image::{Delay, Frame, Rgba, RgbaImage};
use std::fs::File;
use std::time::Duration;

/// Find low points and calculate total risk.
pub fn part1(lines: &[String]) -> usize {
    let map = parse_map(lines);
    low_points(&map).map(|height| 1 + height as usize).sum()
}

#[derive(Debug, Clone, Copy)]
///
pub enum Mode {
    ///
    Gradient,
    ///
    Fill,
}

/// Find 3 largest basins and multiply their sizes together.
pub fn part2(lines: &[String]) -> u64 {
    let map = parse_map(lines);
    let mut images = Vec::new();
    let mut points = HashMap::new();
    const SCALE: u32 = 7;
    const SAMPLE: usize = 36;
    let mut areas = HashMap::new();
    let mut seen = HashMap::new();
    let result = low_point_coords(&map)
        .map(|(i, j)| {
            let area = basin_area(
                &map,
                &mut points,
                &mut HashMap::new(),
                i,
                j,
                &mut images,
                Mode::Gradient,
            );
            areas.insert(area, (i, j));
            area
        })
        .sorted()
        .rev()
        .take(3)
        .inspect(|area| {
            let (i, j) = areas.get(&area).unwrap();
            basin_area(
                &map,
                &mut points,
                &mut seen,
                *i,
                *j,
                &mut images,
                Mode::Fill,
            );
        })
        .product();
    let mut width = images[0].width();
    let mut height = images[0].width();
    println!("{} {} x {} images", images.len(), width, height,);
    width *= SCALE;
    height *= SCALE;
    let frames = images
        .iter()
        .rev()
        .enumerate()
        .filter(|(i, _)| *i == 0 || *i % SAMPLE == 0)
        .rev()
        .map(|(_, image)| {
            let resized = resize(image, width, height, FilterType::Nearest);
            Frame::from_parts(
                resized,
                0,
                0,
                Delay::from_saturating_duration(Duration::from_millis(1)),
            )
        })
        .collect_vec();
    println!(
        "{} {} x {} filtered resized frames",
        frames.len(),
        width,
        height
    );
    let gif = File::create("day09.gif").unwrap();
    let mut encoder = GifEncoder::new(gif);
    encoder.encode_frames(frames).unwrap();
    result
}

/// Breadth-first-search from lowest point.
///
/// Stops at boundaries of map or points of height 9.
pub fn basin_area(
    map: &[Vec<u8>],
    points: &mut HashMap<(usize, usize), u8>,
    seen: &mut HashMap<(usize, usize), u8>,
    row: usize,
    col: usize,
    images: &mut Vec<RgbaImage>,
    mode: Mode,
) -> u64 {
    let mut queue = VecDeque::from([(row, col)]);
    let mut area = 0;

    let bounds = (map[0].len(), map.len());
    while let Some((i, j)) = queue.pop_front() {
        if seen.contains_key(&(i, j)) || map[i][j] == 9 {
            continue;
        }

        area += 1;
        seen.insert((i, j), map[i][j]);
        points.insert((i, j), map[i][j]);
        queue.extend(neighbor_coords(map, i, j));
        images.push(paint(
            bounds,
            points,
            &seen,
            queue.iter().copied().collect(),
            mode,
        ));
    }

    images.push(paint(bounds, points, &seen, HashSet::new(), mode));
    area
}

fn paint(
    bounds: (usize, usize),
    points: &HashMap<(usize, usize), u8>,
    area: &HashMap<(usize, usize), u8>,
    fringe: HashSet<(usize, usize)>,
    mode: Mode,
) -> RgbaImage {
    let (width, height) = bounds;
    let height = height as u32;
    let width = width as u32;
    let mut buffer = RgbaImage::new(width, height);
    for y in 0..height {
        for x in 0..width {
            let i = y as usize;
            let j = x as usize;
            let point = (i, j);
            let height = points.get(&point);
            match mode {
                Mode::Gradient => {
                    if fringe.contains(&point) && height.is_none() {
                        buffer.put_pixel(x, y, Rgba([0, 255, 255, 1]));
                    } else if let Some(&height) = height {
                        let b = (((8 - height) as f64 / 8.0) * 255.0).floor() as u8;
                        buffer.put_pixel(x, y, Rgba([0, 0, b, 1]));
                    } else {
                        buffer.put_pixel(x, y, Rgba([0, 0, 0, 1]));
                    }
                }
                Mode::Fill => {
                    if area.contains_key(&point) {
                        let height = height.unwrap();
                        let b = (((8 - height) as f64 / 8.0) * 255.0).floor() as u8;
                        buffer.put_pixel(x, y, Rgba([b, 0, b, 1]));
                    } else if let Some(&height) = height {
                        let b = (((8 - height) as f64 / 8.0) * 255.0).floor() as u8;
                        buffer.put_pixel(x, y, Rgba([0, 0, b, 1]));
                    } else {
                        buffer.put_pixel(x, y, Rgba([0, 0, 0, 1]));
                    }
                }
            }
        }
    }
    buffer
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
