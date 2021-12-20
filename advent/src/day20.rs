use itertools::{iterate, Itertools};
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::convert::TryInto;

/// Iterate the image enhancement algorithm twice.
pub fn part1(lines: &[String]) -> usize {
    solve(lines, 2)
}

/// Iterate the image enhancement algorithm fifty times.
pub fn part2(lines: &[String]) -> usize {
    solve(lines, 50)
}

/// Iterate the image enhancement algorithm.
pub fn solve(lines: &[String], steps: usize) -> usize {
    let palette = Palette::parse(&lines[0]);
    let image = Image::parse(&lines[2..]);

    iterate(image, |image| image.enhance(&palette))
        .nth(steps)
        .unwrap()
        .lit_pixels()
}

#[derive(Debug)]
/// Table of pixel values. Pixels are either on (true) or off (false).
pub struct Palette {
    pixels: [bool; 512],
}

impl Palette {
    /// Parse palette from a string.
    pub fn parse(text: &str) -> Self {
        assert_eq!(text.len(), 512);
        let mut pixels = [false; 512];
        text.chars()
            .enumerate()
            .for_each(|(i, c)| pixels[i] = c == '#');
        Self { pixels }
    }

    /// Get the pixel at this index.
    pub fn pixel(&self, index: i64) -> bool {
        self.pixels[index as usize]
    }
}

#[derive(Debug, Copy, Clone)]
/// A point in a 2D coordinate system
pub struct Point {
    x: i64,
    y: i64,
}

impl Point {
    /// Create a new point.
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Copy, Clone)]
/// Rectangular bounds represented as the origin and bottom-right corner.
pub struct Bounds {
    origin: Point,
    extent: Point,
}

impl Bounds {
    /// Create new bounds.
    fn new(xmin: i64, ymin: i64, xmax: i64, ymax: i64) -> Self {
        Self {
            origin: Point::new(xmin, ymin),
            extent: Point::new(xmax, ymax),
        }
    }

    /// Expand the bounds one unit in every direction.
    fn pad(&self) -> Self {
        let xmin = self.origin.x - 1;
        let ymin = self.origin.y - 1;
        let xmax = self.extent.x + 1;
        let ymax = self.extent.y + 1;
        Self::new(xmin, ymin, xmax, ymax)
    }
}

#[derive(Debug)]
/// Image to enhance.
pub struct Image {
    pixels: HashMap<(i64, i64), bool>,
    default: bool,
    bounds: Bounds,
}

impl Image {
    /// Parse an image from a sequence of strings.
    pub fn parse(lines: &[String]) -> Self {
        let height = lines.len() as i64;
        let width = lines[0].len() as i64;
        let bounds = Bounds::new(0, 0, width - 1, height - 1);

        let pixels = lines
            .iter()
            .enumerate()
            .flat_map(|(i, line)| {
                line.chars()
                    .enumerate()
                    .map(move |(j, c)| ((i as i64, j as i64), c == '#'))
            })
            .collect();

        Self {
            pixels,
            default: false,
            bounds,
        }
    }

    /// Sample each pixel and its nine neighbors to create a new image
    pub fn enhance(&self, palette: &Palette) -> Self {
        let bounds = self.bounds.pad();
        let (ymin, ymax) = (bounds.origin.y, bounds.extent.y);
        let (xmin, xmax) = (bounds.origin.x, bounds.extent.x);

        let pixels = (ymin..=ymax)
            .cartesian_product(xmin..=xmax)
            .map(|(i, j)| {
                let index = self.sample(i, j).fold(0, |n, lit| n << 1 | (lit as i64));
                ((i, j), palette.pixel(index))
            })
            .collect();

        let default = if self.default {
            palette.pixel(0b111111111 - 1)
        } else {
            palette.pixel(0)
        };

        Self {
            pixels,
            bounds,
            default,
        }
    }

    /// Yield each of the nine pixels in the pixel's neighborhood.
    pub fn sample(&self, i: i64, j: i64) -> impl Iterator<Item = bool> + '_ {
        SAMPLE
            .iter()
            .map(move |(di, dj)| self.pixels.get(&(i + di, j + dj)).unwrap_or(&self.default))
            .copied()
    }

    /// Count the number of lit pixels
    pub fn lit_pixels(&self) -> usize {
        self.pixels.values().filter(|&&lit| lit).count()
    }
}

lazy_static! {
    /// Area around a point to sample.
    pub static ref SAMPLE: [(i64, i64); 9] = (-1..=1)
        .cartesian_product(-1..=1)
        .collect_vec()
        .try_into()
        .unwrap();
}

check!(ex 1 = 35, ex 2 = 3351, part 1 = 5231, part 2 = 14279);
