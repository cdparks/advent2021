use itertools::Itertools;
use sscanf::scanf;
use std::ops::RangeInclusive;

/// Volume of enabled prisms within a 100-unit cube.
pub fn part1(lines: &[String]) -> usize {
    let bounds = Prism::cube(false, -50..=50);
    solve(&parse(lines, &Some(bounds)))
}

/// Volume of all enabled prisms.
pub fn part2(lines: &[String]) -> usize {
    solve(&parse(lines, &None))
}

/// For each enabled prism compute its volume minus the volume of any
/// intersections with that prism. Igore the disabled prisms. Both enabled
/// and disabled prisms have the same effect recursively (their volume
/// is subtracted out from the enclosing prism).
pub fn solve(prisms: &[Prism]) -> usize {
    let mut total = 0;
    for (i, prism) in prisms.iter().enumerate() {
        if prism.on {
            total += prism.volume_on(&prisms[i + 1..]);
        }
    }
    total
}

/// Add some interval-like functionality to a type
pub trait Interval {
    /// Result of intersect
    type Intersection;

    /// Length of interval.
    fn length(&self) -> usize;

    /// Compute the intersection of an interval
    fn intersect(&self, other: &Self) -> Self::Intersection;
}

impl Interval for RangeInclusive<i64> {
    type Intersection = Option<Self>;

    fn length(&self) -> usize {
        (self.end() - self.start() + 1).abs() as usize
    }

    fn intersect(&self, other: &Self) -> Self::Intersection {
        if other.end() < self.start() || self.end() < other.start() {
            return None;
        }
        let start = *self.start().max(other.start());
        let end = *self.end().min(other.end());
        Some(start..=end)
    }
}

#[derive(Debug, Clone)]
/// Axis-aligned rectangular prism
pub struct Prism {
    on: bool,
    x: RangeInclusive<i64>,
    y: RangeInclusive<i64>,
    z: RangeInclusive<i64>,
}

impl Prism {
    /// Parse a prism from a string, optionally restricting the result
    /// to be within some boundaries.
    pub fn parse(line: &str, bounds: &Option<Prism>) -> Option<Self> {
        let (on, xmin, xmax, ymin, ymax, zmin, zmax) = scanf!(
            line,
            "{/on|off/} x={}..{},y={}..{},z={}..{}",
            String,
            i64,
            i64,
            i64,
            i64,
            i64,
            i64
        )?;

        let prism = Self {
            on: on == "on",
            x: xmin..=xmax,
            y: ymin..=ymax,
            z: zmin..=zmax,
        };

        if let Some(bounds) = bounds {
            return prism.intersect(bounds);
        }
        Some(prism)
    }

    /// Create a prism with equal-length sides
    pub fn cube(on: bool, interval: RangeInclusive<i64>) -> Self {
        Self {
            on,
            x: interval.clone(),
            y: interval.clone(),
            z: interval,
        }
    }

    /// Compute the intersection of two prisms if it exists.
    pub fn intersect(&self, other: &Prism) -> Option<Self> {
        Some(Self {
            on: self.on,
            x: self.x.intersect(&other.x)?,
            y: self.y.intersect(&other.y)?,
            z: self.z.intersect(&other.z)?,
        })
    }

    /// Compute the volume of the prism.
    fn volume(&self) -> usize {
        self.x.length() * self.y.length() * self.z.length()
    }

    /// Compute the volume of the prism without counting the volume of
    /// any intersections from other prisms.
    fn volume_on(&self, others: &[Prism]) -> usize {
        let overlapping = others
            .iter()
            .filter_map(|other| self.intersect(other))
            .collect_vec();

        let mut volume = self.volume();
        for (i, other) in overlapping.iter().enumerate() {
            volume -= other.volume_on(&overlapping[i + 1..]);
        }
        volume
    }
}

fn parse(lines: &[String], bounds: &Option<Prism>) -> Vec<Prism> {
    lines
        .iter()
        .filter_map(|line| Prism::parse(line, bounds))
        .collect_vec()
}

check!(ex 1 = 474140, ex 2 = 2758514936282235, part 1 = 591365, part 2 = 1211172281877240);
bench!(part 1, part 2);
