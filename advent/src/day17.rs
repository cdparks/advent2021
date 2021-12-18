use itertools::Itertools;
use sscanf::scanf;

/// Find highest _y_ along a path that intersects the target area.
pub fn part1(lines: &[String]) -> i64 {
    let area = Area::parse(&lines.iter().join(""));
    velocities(area).map(|(_, y)| y).max().unwrap()
}

/// Count the distinct velocities that will launch a probe into the target area.
pub fn part2(lines: &[String]) -> usize {
    let area = Area::parse(&lines.iter().join(""));
    velocities(area).count()
}

/// Calculate the sequence of velocities that will launch a probe into the target area.
pub fn velocities(area: Area) -> impl Iterator<Item = (Velocity, i64)> {
    // Beyond target area after 1 step
    let dxmax = area.xmax;
    let dymin = area.ymin;
    let dymax = -area.ymin;

    (1..=dxmax)
        .cartesian_product(dymin..=dymax)
        .filter_map(move |(dx, dy)| {
            let velocity = Point::new(dx, dy);
            simulate(area, velocity).map(|y| (velocity, y))
        })
}

/// Simulate a probe being launched at the target area using the starting velocity.
///
/// Returns the highest _y_ if the target area was hit.
pub fn simulate(area: Area, mut velocity: Velocity) -> Option<i64> {
    let mut point = Point::origin();
    let mut max_y = point.y;
    while !point.beyond(area) {
        max_y = max_y.max(point.y);
        if area.contains(point) {
            return Some(max_y);
        }
        step(&mut point, &mut velocity);
    }
    None
}

/// Compute one step of the simulation.
///
/// Apply the velocity to the point, then apply drag and gravity to
/// the velocity.
fn step(point: &mut Point, velocity: &mut Velocity) {
    point.x += velocity.x;
    point.y += velocity.y;
    velocity.x -= velocity.x.signum();
    velocity.y -= 1;
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
/// A point in a 2D coordinate system.
pub struct Point {
    x: i64,
    y: i64,
}

/// Reuse Point as Velocity.
pub type Velocity = Point;

impl Point {
    /// Create a point a 2D coordinate system
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    /// Origin in a 2D coordinate system
    fn origin() -> Self {
        Self::new(0, 0)
    }

    /// Is this point beyond the target area?
    fn beyond(&self, area: Area) -> bool {
        self.x > area.xmax || self.y < area.ymin
    }
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
/// Target area defined by its bottom-left and top-right corners.
pub struct Area {
    xmin: i64,
    xmax: i64,
    ymin: i64,
    ymax: i64,
}

impl Area {
    /// Parse the target area from a string.
    pub fn parse(input: &str) -> Self {
        let (xmin, xmax, ymin, ymax) =
            scanf!(input, "target area: x={}..{}, y={}..{}", i64, i64, i64, i64).unwrap();
        Self {
            xmin,
            xmax,
            ymin,
            ymax,
        }
    }

    /// Does the target area contain this point?
    pub fn contains(&self, point: Point) -> bool {
        self.xmin <= point.x && point.x <= self.xmax && self.ymin <= point.y && point.y <= self.ymax
    }
}

check!(ex 1 = 45, ex 2 = 112, part 1 = 2628, part 2 = 1334);
bench!(part 1, part 2);
