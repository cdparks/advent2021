use itertools::Itertools;
use std::str::FromStr;

/// Fold paper along first line.
pub fn part1(lines: &[String]) -> usize {
    let (points, folds) = parse(lines);
    fold(points, &folds[0]).len()
}

/// Completely fold paper and render result.
pub fn part2(lines: &[String]) -> String {
    let (points, folds) = parse(lines);
    render(folds.iter().fold(points, fold))
}

fn fold(points: Vec<Point>, fold: &Fold) -> Vec<Point> {
    points
        .iter()
        .map(|point| point.fold(fold))
        .unique()
        .collect()
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

#[derive(PartialEq, Eq, Debug)]
/// Represents a fold across an axis.
pub enum Fold {
    /// Fold over vertical line x=n.
    X(usize),
    /// Fold over horizontal lin y=n.
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
