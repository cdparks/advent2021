use itertools::Itertools;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

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

        while let Some((Reverse(cost), node @ (row, col))) = queue.pop() {
            if node == target {
                return cost;
            }

            if cost > costs[&node] {
                continue;
            }

            for (dy, dx) in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
                let neighbor = (row + dy, col + dx);
                if let Some(neighbor_cost) = self.graph.get(&neighbor) {
                    let cost = neighbor_cost + cost;
                    if cost < costs[&neighbor] {
                        queue.push((Reverse(cost), neighbor));
                        costs.insert(neighbor, cost);
                    }
                }
            }
        }
        usize::MAX
    }
}

check!(ex 1 = 40, ex 2 = 315, part 1 = 589, part 2 = 2885);
bench!(part 1, part 2);
