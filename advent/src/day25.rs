use image::codecs::gif::GifEncoder;
use image::imageops::{resize, FilterType};
use image::{Delay, Frame, Rgba, RgbaImage};
use itertools::Itertools;
use std::fs::File;
use std::time::Duration;

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
    let mut images = Vec::new();
    loop {
        let east = step(&mut map, Facing::East, (0, 1));
        let south = step(&mut map, Facing::South, (1, 0));
        images.push(paint(&map));
        steps += 1;
        if !east && !south {
            break;
        }
    }
    const SCALE: u32 = 5;
    const SAMPLE: usize = 4;
    let mut width = map[0].len() as u32;
    let mut height = map.len() as u32;
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
    let gif = File::create("day25.gif").unwrap();
    let mut encoder = GifEncoder::new(gif);
    encoder.encode_frames(frames).unwrap();
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

fn paint(map: &Vec<Vec<Facing>>) -> RgbaImage {
    let width = map[0].len() as u32;
    let height = map.len() as u32;
    let mut buffer = RgbaImage::new(width, height);
    for y in 0..height {
        for x in 0..width {
            let i = y as usize;
            let j = x as usize;
            match map[i][j] {
                Facing::Empty => buffer.put_pixel(x, y, Rgba([0, 0, 0, 1])),
                Facing::East => buffer.put_pixel(x, y, Rgba([255, 0, 64, 1])),
                Facing::South => buffer.put_pixel(x, y, Rgba([0, 64, 255, 1])),
            }
        }
    }
    buffer
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
