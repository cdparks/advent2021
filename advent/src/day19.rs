use itertools::Itertools;
use lazy_static::lazy_static;
use sscanf::scanf;
use std::collections::{HashMap, HashSet, VecDeque};

/// Count the number of beacons.
pub fn part1(lines: &[String]) -> usize {
    solve(lines).0.beacons.len()
}

/// Find the maximum distance between any two scanners.
pub fn part2(lines: &[String]) -> i64 {
    solve(lines).1
}

/// Merge the scanners and calculate the maximum distance between any two scanners.
fn solve(lines: &[String]) -> (Scanner, i64) {
    let mut scanners = VecDeque::from(parse(lines));
    let mut base = scanners.pop_front().unwrap();
    let mut distances = Vec::new();
    while let Some(other) = scanners.pop_front() {
        if let Some((merged, distance)) = base.merge(&other) {
            base = merged;
            distances.push(distance);
        } else {
            scanners.push_back(other);
        }
    }

    let max_distance = distances
        .iter()
        .cartesian_product(distances.iter())
        .map(|(lhs, rhs)| lhs.manhattan(*rhs))
        .max()
        .unwrap();

    (base, max_distance)
}

/// Parse into sequence of Scanners.
fn parse(lines: &[String]) -> Vec<Scanner> {
    lines
        .split(String::is_empty)
        .flat_map(Scanner::parse)
        .collect()
}

#[derive(PartialEq, Eq, Debug, Copy, Clone, Hash)]
/// Point in 3D coordinate space
pub struct Point3 {
    x: i64,
    y: i64,
    z: i64,
}

/// Type alias for using a Point as a Vector
pub type Vec3 = Point3;

impl Point3 {
    /// Create a point in 3D space.
    pub fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }

    /// Parse a point from a string.
    pub fn parse(line: &str) -> Option<Self> {
        let (x, y, z) = scanf!(line, "{},{},{}", i64, i64, i64)?;
        Some(Self::new(x, y, z))
    }

    /// Translate a point by the vector.
    pub fn translate(&self, d: Vec3) -> Self {
        Self::new(self.x + d.x, self.y + d.y, self.z + d.z)
    }

    /// Calculate a vector from one point to another.
    pub fn dist(&self, other: Point3) -> Vec3 {
        Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }

    /// Dot product of two points.
    pub fn dot(&self, other: Point3) -> i64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Apply rotation matrix to point.
    pub fn rotate(&self, matrix: Matrix3) -> Self {
        Self::new(self.dot(matrix.x), self.dot(matrix.y), self.dot(matrix.z))
    }

    /// Manhattan distance between two points.
    pub fn manhattan(&self, other: Point3) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }
}

/// Convert a 3-element array to a point.
impl From<[i64; 3]> for Point3 {
    fn from(point: [i64; 3]) -> Self {
        Self::new(point[0], point[1], point[2])
    }
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
/// 3 x 3 matrix.
pub struct Matrix3 {
    x: Point3,
    y: Point3,
    z: Point3,
}

impl Matrix3 {
    /// Create a 3 x 3 matrix from 3 points.
    pub fn new(x: Point3, y: Point3, z: Point3) -> Self {
        Self { x, y, z }
    }
}

/// Convert a 3 x 3 nested array to a matrix.
impl From<[[i64; 3]; 3]> for Matrix3 {
    fn from(rows: [[i64; 3]; 3]) -> Self {
        Self::new(
            Point3::from(rows[0]),
            Point3::from(rows[1]),
            Point3::from(rows[2]),
        )
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
/// Set of beacons relative to this scanner.
pub struct Scanner {
    beacons: HashSet<Point3>,
}

impl Scanner {
    /// Create a new scanner.
    pub fn new(beacons: HashSet<Point3>) -> Self {
        Self { beacons }
    }

    /// Parse a scanner from a string.
    pub fn parse(lines: &[String]) -> Option<Self> {
        let beacons = lines[1..]
            .iter()
            .flat_map(|line| Point3::parse(line))
            .collect();
        Some(Self::new(beacons))
    }

    /// Attempt to align the second scanner to the first, adding both of
    /// their becaons to the result. Also return the vector from the
    /// first scanner to the second.
    pub fn merge(&self, other: &Scanner) -> Option<(Scanner, Vec3)> {
        for mut scanner in other.rotations() {
            if let Some(distance) = self.match_beacons(&scanner) {
                scanner.translate(distance);
                scanner.extend(&self.beacons);
                return Some((scanner, distance));
            }
        }
        None
    }

    /// Attempt to find a vector from the first scanner to the second.
    fn match_beacons(&self, other: &Scanner) -> Option<Vec3> {
        let distances: HashMap<Point3, usize> = self
            .beacons
            .iter()
            .cartesian_product(other.beacons.iter())
            .fold(HashMap::new(), |mut acc, (lhs, rhs)| {
                *acc.entry(lhs.dist(*rhs)).or_default() += 1;
                acc
            });

        let matches = distances
            .iter()
            .filter_map(|(d, n)| if *n >= 12 { Some(*d) } else { None })
            .collect_vec();

        matches.first().cloned()
    }

    /// Sequence of rotations of the input.
    pub fn rotations(&self) -> impl Iterator<Item = Scanner> + '_ {
        ROTATIONS.iter().map(move |matrix| {
            Scanner::new(
                self.beacons
                    .iter()
                    .map(|point| point.rotate(*matrix))
                    .collect(),
            )
        })
    }

    /// Translate every beacon in the scanner.
    pub fn translate(&mut self, distance: Vec3) {
        self.beacons = self
            .beacons
            .iter()
            .map(|point| point.translate(distance))
            .collect();
    }

    /// Add new beacons to the scanner.
    pub fn extend(&mut self, beacons: &HashSet<Point3>) {
        self.beacons.extend(beacons)
    }
}

lazy_static! {
    /// All 24 rotation matrixes, generated with a script.
    pub static ref ROTATIONS: [Matrix3; 24] = [
        [[0, 0, 1], [0, 1, 0], [-1, 0, 0]].into(),
        [[0, 0, 1], [0, -1, 0], [1, 0, 0]].into(),
        [[0, 0, -1], [0, 1, 0], [1, 0, 0]].into(),
        [[0, 0, -1], [0, -1, 0], [-1, 0, 0]].into(),
        [[0, 0, 1], [1, 0, 0], [0, 1, 0]].into(),
        [[0, 0, 1], [-1, 0, 0], [0, -1, 0]].into(),
        [[0, 0, -1], [1, 0, 0], [0, -1, 0]].into(),
        [[0, 0, -1], [-1, 0, 0], [0, 1, 0]].into(),
        [[0, 1, 0], [0, 0, 1], [1, 0, 0]].into(),
        [[0, 1, 0], [0, 0, -1], [-1, 0, 0]].into(),
        [[0, -1, 0], [0, 0, 1], [-1, 0, 0]].into(),
        [[0, -1, 0], [0, 0, -1], [1, 0, 0]].into(),
        [[0, 1, 0], [1, 0, 0], [0, 0, -1]].into(),
        [[0, 1, 0], [-1, 0, 0], [0, 0, 1]].into(),
        [[0, -1, 0], [1, 0, 0], [0, 0, 1]].into(),
        [[0, -1, 0], [-1, 0, 0], [0, 0, -1]].into(),
        [[1, 0, 0], [0, 0, 1], [0, -1, 0]].into(),
        [[1, 0, 0], [0, 0, -1], [0, 1, 0]].into(),
        [[-1, 0, 0], [0, 0, 1], [0, 1, 0]].into(),
        [[-1, 0, 0], [0, 0, -1], [0, -1, 0]].into(),
        [[1, 0, 0], [0, 1, 0], [0, 0, 1]].into(),
        [[1, 0, 0], [0, -1, 0], [0, 0, -1]].into(),
        [[-1, 0, 0], [0, 1, 0], [0, 0, -1]].into(),
        [[-1, 0, 0], [0, -1, 0], [0, 0, 1]].into(),
    ];
}

check!(ex 1 = 79, ex 2 = 3621, part 1 = 355, part 2 = 10842);
