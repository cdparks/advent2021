use itertools::Itertools;
use std::collections::HashSet;
use std::str::FromStr;

use crate::gif::{self, Frame, Image};

/// Fold paper along first line.
pub fn part1(lines: &[String]) -> usize {
    let (points, folds) = parse(lines);
    fold(points, &folds[0])
        .last()
        .unwrap()
        .iter()
        .unique()
        .count()
}

/// Completely fold paper and render result.
pub fn part2(lines: &[String]) -> String {
    let (points, folds) = parse(lines);
    let mut width = points.iter().map(|point| point.x).max().unwrap_or(0) + 1;
    let mut height = points.iter().map(|point| point.y).max().unwrap_or(0) + 1;
    let global_bounds = (width, height);

    let mut frames = Vec::new();
    let points = folds.iter().fold(points, |points, f| {
        let before = points.clone();
        let changes = fold(points, &f);
        let sample = if changes.len() > 16 {
            changes.len() / 16
        } else {
            1
        };
        println!("gen {} changes, sample % {}", changes.len(), sample);
        for (i, change) in changes.iter().enumerate() {
            if i == changes.len() - 1 || i % sample == 0 {
                frames.push(paint(
                    global_bounds,
                    (width, height),
                    &before,
                    &change,
                    Some(*f),
                ));
            }
        }
        if changes.len() == 12 {
            println!("last gen, adding more frames");
            let frame = frames.last().unwrap().clone();
            for _ in 0..20 {
                frames.push(frame.clone());
            }
        }
        match f {
            Fold::X(x) => width = *x + 1,
            Fold::Y(y) => height = *y + 1,
        }
        changes.last().unwrap().iter().unique().copied().collect()
    });

    let final_width = points.iter().map(|point| point.x).max().unwrap_or(0) + 1;
    let final_height = points.iter().map(|point| point.y).max().unwrap_or(0) + 1;
    let frame = paint(
        global_bounds,
        (final_width, final_height),
        &points,
        &points,
        None,
    );
    for _ in 0..20 {
        frames.push(frame.clone())
    }

    println!("{} {} x {} frames", frames.len(), width, height);
    gif::write("day13.gif", frames);
    render(points)
}

fn fold(mut points: Vec<Point>, fold: &Fold) -> Vec<Vec<Point>> {
    let mut changed = true;
    let mut changes: Vec<Vec<Point>> = Vec::new();
    let targets: Vec<Point> = points.iter().map(|point| point.fold(fold)).collect();

    while changed {
        changed = false;
        points = points
            .iter()
            .enumerate()
            .map(|(i, point)| {
                let target = targets[i];
                if point.x != target.x {
                    changed = true;
                    Point::new(point.x - 1, point.y)
                } else if point.y != target.y {
                    changed = true;
                    Point::new(point.x, point.y - 1)
                } else {
                    *point
                }
            })
            .collect();
        if changed {
            changes.push(points.clone())
        }
    }
    changes
}

fn paint(
    global_bounds: (usize, usize),
    current_bounds: (usize, usize),
    points: &Vec<Point>,
    changes: &Vec<Point>,
    fold: Option<Fold>,
) -> Frame {
    let (width, height) = current_bounds;
    let mut image = Image::new(width, height);
    let axis: HashSet<Point> = match fold {
        Some(Fold::X(x)) => (0..height).map(|y| Point::new(x, y as usize)).collect(),
        Some(Fold::Y(y)) => (0..width).map(|x| Point::new(x as usize, y)).collect(),
        None => HashSet::new(),
    };
    for y in 0..height {
        for x in 0..width {
            let point = Point::new(x as usize, y as usize);
            if let Some(i) = changes.iter().position(|&x| x == point) {
                if points[i] != point {
                    image.set(x, y, [255, 255, 0]);
                } else {
                    image.set(x, y, [0, 255, 255]);
                }
            } else if axis.contains(&point) {
                image.set(x, y, [255, 0, 255]);
            } else {
                image.black(x, y);
            }
        }
    }
    if global_bounds.0 != width || global_bounds.1 != height {
        image = image.resize(global_bounds.0, global_bounds.1);
    }

    image.frame(100)
}

fn render(points: Vec<Point>) -> String {
    let max_x = points.iter().map(|point| point.x).max().unwrap_or(0);
    let max_y = points.iter().map(|point| point.y).max().unwrap_or(0);
    let mut out = String::new();
    for y in 0..=max_y {
        for x in 0..=max_x {
            if points.contains(&Point::new(x, y)) {
                out.push('█');
            } else {
                out.push(' ');
            }
        }
        out.push('\n');
    }
    out
}

fn parse(lines: &[String]) -> (Vec<Point>, Vec<Fold>) {
    let i = lines.iter().position(|line| line.is_empty()).unwrap();
    let points: Vec<Point> = lines[0..i]
        .iter()
        .flat_map(|line| str::parse(line))
        .collect();
    let folds: Vec<Fold> = lines[i + 1..]
        .iter()
        .flat_map(|line| str::parse(line))
        .collect();
    (points, folds)
}

#[derive(PartialEq, Eq, Debug, Hash, Copy, Clone)]
/// A point in a 2D coordinate system
pub struct Point {
    x: usize,
    y: usize,
}

impl Point {
    /// Create a point from 2 scalars.
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    /// Transpose a point across the specified axis or return the same point.
    pub fn fold(self, fold: &Fold) -> Self {
        match *fold {
            Fold::X(x) if self.x > x => Self::new(x - (self.x - x), self.y),
            Fold::Y(y) if self.y > y => Self::new(self.x, y - (self.y - y)),
            _ => self,
        }
    }
}

impl FromStr for Point {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let go = || {
            let (lhs, rhs) = s.split_once(",")?;
            let x = lhs.parse().ok()?;
            let y = rhs.parse().ok()?;
            Some(Point::new(x, y))
        };
        Ok(go().unwrap())
    }
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
/// Represents a fold across an axis.
pub enum Fold {
    /// Fold over vertical line x=n.
    X(usize),
    /// Fold over horizontal line y=n.
    Y(usize),
}

impl FromStr for Fold {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let go = || {
            let (lhs, rhs) = s.split_once("=")?;
            let axis = lhs.chars().last()?;
            let i = str::parse(rhs).ok()?;
            match axis {
                'x' => Some(Self::X(i)),
                'y' => Some(Self::Y(i)),
                _ => None,
            }
        };
        Ok(go().unwrap())
    }
}

check!(ex 1 = 17, ex 2 = DIFF, part 1 = 631, part 2 = DIFF);
bench!(part 1, part 2);
