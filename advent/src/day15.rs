use itertools::Itertools;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};

use image::codecs::gif::GifEncoder;
use image::{Delay, Frame, Rgba, RgbaImage};
use std::fs::File;
use std::time::Duration;

/// Find the cost of the optimal path from starting point to the bottom-right point.
pub fn part1(lines: &[String]) -> usize {
    Graph::parse(lines).dijkstra()
}

/// Find the cost of the optimal path from starting point to the bottom-right point in the larger map.
pub fn part2(lines: &[String]) -> usize {
    Graph::parse(lines).expand(5).dijkstra()
}

/// A point in a 2D coordinate system.
pub type Point = (i64, i64);

#[derive(PartialEq, Eq, Debug)]
/// A mapping from points in 2D space to the cost of traversing them.
pub struct Graph {
    graph: HashMap<Point, usize>,
}

impl Graph {
    /// Parse a graph from the input.
    pub fn parse(lines: &[String]) -> Self {
        let graph = lines
            .iter()
            .enumerate()
            .flat_map(|(row, line)| {
                line.bytes()
                    .enumerate()
                    .map(move |(col, byte)| ((row as i64, col as i64), (byte - b'0') as usize))
            })
            .collect();
        Self { graph }
    }

    /// Find the bottom-right point.
    pub fn target(&self) -> Point {
        let row = self.graph.keys().map(|&(row, _)| row).max().unwrap();
        let col = self.graph.keys().map(|&(_, col)| col).max().unwrap();
        (row, col)
    }

    /// Get the dimensions of the graph as _height × width_.
    pub fn dimensions(&self) -> Point {
        let (row, col) = self.target();
        (row + 1, col + 1)
    }

    /// Expand the graph down and to the right, increasing each point's
    /// cost every time. If the cost of a point would go over 9, it
    /// wraps back to 1.
    pub fn expand(self, n: usize) -> Self {
        let n = n as i64;
        let (height, width) = self.dimensions();
        let graph = (0..n)
            .cartesian_product(0..n)
            .flat_map(|(y, x)| {
                self.graph.iter().map(move |((row, col), &cost)| {
                    let row = row + height * y;
                    let col = col + width * x;
                    let cost = ((cost as i64 + x + y) - 1) % 9 + 1;
                    ((row, col), cost as usize)
                })
            })
            .collect();
        Self { graph }
    }

    /// Use Dijkstra's algorithm to find the cost of the optimal path
    /// from the top-left point to the bottom-right point.
    pub fn dijkstra(&self) -> usize {
        let target = self.target();

        // Map point to lowest cost
        let mut costs: HashMap<_, _> = self
            .graph
            .iter()
            .map(|(&point, _)| (point, usize::MAX))
            .collect();
        costs.insert((0, 0), 0);

        // Minimum-cost priority queue. BinaryHeap is a max-heap, so
        // use Reverse to flip comparisons.
        let mut queue = BinaryHeap::new();
        queue.push((Reverse(0), (0, 0)));

        let mut parents: HashMap<Point, Point> = HashMap::new();
        let mut frames = Vec::new();

        let mut steps = 0;
        while let Some((Reverse(cost), node @ (row, col))) = queue.pop() {
            if node == target {
                frames.push(self.to_frame(
                    &costs,
                    queue.iter().map(|(_, p)| p).copied().collect(),
                    node,
                    &parents,
                ));
                let gif = File::create("day15.gif").unwrap();
                let mut encoder = GifEncoder::new(gif);
                encoder.encode_frames(frames).unwrap();
                return cost;
            }

            if cost > costs[&node] {
                continue;
            }

            steps += 1;
            if steps % 1000 == 0 {
                frames.push(self.to_frame(
                    &costs,
                    queue.iter().map(|(_, p)| p).copied().collect(),
                    node,
                    &parents,
                ));
            }

            for (dy, dx) in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
                let neighbor = (row + dy, col + dx);
                if let Some(neighbor_cost) = self.graph.get(&neighbor) {
                    let cost = neighbor_cost + cost;
                    if cost < costs[&neighbor] {
                        queue.push((Reverse(cost), neighbor));
                        parents.insert((row + dy, col + dx), (row, col));
                        costs.insert(neighbor, cost);
                    }
                }
            }
        }
        usize::MAX
    }

    fn to_frame(
        &self,
        costs: &HashMap<Point, usize>,
        fringe: HashSet<Point>,
        mut node: Point,
        parents: &HashMap<Point, Point>,
    ) -> Frame {
        let (height, width) = self.dimensions();
        let height = height as u32;
        let width = width as u32;
        let mut buffer = RgbaImage::new(width, height);

        let mut path = HashSet::from([node]);
        while let Some(&parent) = parents.get(&node) {
            path.insert(parent);
            node = parent;
        }

        for y in 0..height {
            for x in 0..width {
                let i = y as i64;
                let j = x as i64;
                let point = (i, j);
                if path.contains(&point) {
                    buffer.put_pixel(x, y, Rgba([255, 255, 255, 1]));
                } else if fringe.contains(&point) {
                    buffer.put_pixel(x, y, Rgba([0, 255, 255, 1]));
                } else if costs.get(&point) == Some(&usize::MAX) {
                    buffer.put_pixel(x, y, Rgba([0, 0, 0, 1]));
                } else {
                    let cost = *self.graph.get(&point).unwrap();
                    let b = ((cost as f64 / 9.0) * 255.0).floor() as u8;
                    buffer.put_pixel(x, y, Rgba([0, 0, b, 1]));
                }
            }
        }
        Frame::from_parts(
            buffer,
            0,
            0,
            Delay::from_saturating_duration(Duration::from_millis(1)),
        )
    }
}

check!(ex 1 = 40, ex 2 = 315, part 1 = 589, part 2 = 2885);
bench!(part 1, part 2);
