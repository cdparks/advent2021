use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

/// Count the number of points where horizontal or vertical lines overlap
pub fn part1(lines: &[Line]) -> usize {
    count_overlapping(lines, |line| line.is_horizontal() || line.is_vertical())
}

/// Count the number of points where horizontal, vertical, or diagonal lines overlap
pub fn part2(lines: &[Line]) -> usize {
    count_overlapping(lines, |line| {
        line.is_horizontal() || line.is_vertical() || line.is_diagonal()
    })
}

/// Count the number of points overlapped by the filtered lines.
pub fn count_overlapping<F>(lines: &[Line], pred: F) -> usize
where
    F: Fn(&Line) -> bool,
{
    lines
        .iter()
        .filter_map(|&line| if pred(&line) { Some(line) } else { None })
        .flat_map(|line| line.points())
        .fold(HashMap::new(), |mut points, point| {
            *points.entry(point).or_insert(0) += 1;
            points
        })
        .into_values()
        .filter(|&n| n > 1)
        .count()
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone, Hash)]
/// A line segment defined by two points.
pub struct Line {
    lhs: Point,
    rhs: Point,
}

impl Line {
    /// Create a line from 2 points.
    pub fn new(lhs: Point, rhs: Point) -> Self {
        Self { lhs, rhs }
    }

    /// Determine if the line is horizontal.
    ///
    /// That is, if the _y_-coordinate never changes.
    pub fn is_horizontal(&self) -> bool {
        self.lhs.y == self.rhs.y
    }

    /// Determine if the line is vertical.
    ///
    /// That is, if the _x_-coordinate never changes.
    pub fn is_vertical(&self) -> bool {
        self.lhs.x == self.rhs.x
    }

    /// Determine if the line is on a 45° diagonal.
    ///
    /// That is, if _|x₂ - x₁| ≡ |y₂ - y₁|_.
    pub fn is_diagonal(&self) -> bool {
        (self.lhs.x - self.rhs.x).abs() == (self.lhs.y - self.rhs.y).abs()
    }

    /// Return sequence of points covered by a horizontal, vertical, or
    /// 45° diagonal line.
    ///
    /// # Panics
    ///
    /// Panics if the line is not a horizontal, vertical, or 45°
    /// diagonal line. For any other kind of line, we would have to
    /// interpolate points _à la_ [Bresenham's line algorithm][Bresenham].
    ///
    /// [Bresenham]: https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm
    pub fn points(&self) -> Vec<Point> {
        assert!(self.is_horizontal() || self.is_vertical() || self.is_diagonal());

        let x_dist = self.rhs.x - self.lhs.x;
        let y_dist = self.rhs.y - self.lhs.y;

        let dx = x_dist.signum();
        let dy = y_dist.signum();

        let (x, y) = (self.lhs.x, self.lhs.y);

        let n = usize::max(x_dist.abs() as usize, y_dist.abs() as usize);
        (0..=n)
            .map(|i| Point::new(x + dx * i as i32, y + dy * i as i32))
            .collect()
    }
}

impl FromStr for Line {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (lhs, rhs) = s.split_once(" -> ").unwrap();
        let lhs = lhs.parse().unwrap();
        let rhs = rhs.parse().unwrap();
        Ok(Line::new(lhs, rhs))
    }
}

impl Display for Line {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{} -> {}", self.lhs, self.rhs)
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone, Hash)]
/// A point in a 2D coordinate system
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    /// Create a point from 2 scalars.
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl FromStr for Point {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').unwrap();
        let x = x.parse().unwrap();
        let y = y.parse().unwrap();
        Ok(Point::new(x, y))
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{},{}", self.x, self.y)
    }
}

check!(ex 1 = 5, ex 2 = 12, part 1 = 5632, part 2 = 22213);
bench!(part 1, part 2);
