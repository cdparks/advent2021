use std::collections::HashMap;

/// Count paths through caves. Small caves can only be visited once.
pub fn part1(lines: &[String]) -> usize {
    let graph = parse_graph(lines);
    count_paths(&graph, Mode::Once)
}

/// Count paths through caves. One small cave may be visited twice; all
/// other small caves can only be visited once.
pub fn part2(lines: &[String]) -> usize {
    let graph = parse_graph(lines);
    count_paths(&graph, Mode::Twice)
}

/// Count paths through graph from `start` to `end`. Final argument indicates
/// whether a single small cave can be visited twice or not.
pub fn count_paths(graph: &HashMap<&str, Vec<&str>>, mode: Mode) -> usize {
    let mut path = Path::new();
    let twice = match mode {
        Mode::Once => true,
        Mode::Twice => false,
    };
    walk(graph, &mut path, twice, "start")
}

fn walk<'a>(
    graph: &HashMap<&str, Vec<&'a str>>,
    path: &mut Path<'a>,
    mut twice: bool,
    node: &'a str,
) -> usize {
    if node == "end" {
        return 1;
    }

    if node == node.to_lowercase() && path.occurrences(node) > 0 {
        if twice {
            return 0;
        }
        twice = true;
    }

    path.push(node);

    let paths = graph[node]
        .iter()
        .filter(|&&child| child != "start")
        .map(|child| walk(graph, path, twice, child))
        .sum();

    path.pop();
    paths
}

#[derive(PartialEq, Eq, Debug)]
/// How to visit small caves.
pub enum Mode {
    /// Each small cave can be visited once.
    Once,
    /// A single small cave can be visited twice.
    Twice,
}

#[derive(PartialEq, Eq, Debug)]
/// A path through the graph.
///
/// Provides stack-oriented modification with _O(1)_ membership testing.
pub struct Path<'a> {
    path: Vec<&'a str>,
    counts: HashMap<&'a str, usize>,
}

impl<'a> Path<'a> {
    /// Create an empty path.
    pub fn new() -> Self {
        Self {
            path: Vec::new(),
            counts: HashMap::new(),
        }
    }

    /// Push a node onto the end of the path.
    fn push(&mut self, key: &'a str) {
        self.path.push(key);
        *self.counts.entry(key).or_insert(0) += 1;
    }

    /// Pop the last node from the end of the path.
    fn pop(&mut self) {
        if let Some(key) = self.path.pop() {
            self.counts.entry(key).and_modify(|n| *n -= 1);
        }
    }

    #[inline]
    /// Count the number of times a node appears in the path.
    fn occurrences(&self, key: &'a str) -> usize {
        *self.counts.get(key).unwrap_or(&0)
    }
}

fn parse_graph<'a>(lines: &'a [String]) -> HashMap<&'a str, Vec<&'a str>> {
    lines
        .iter()
        .flat_map(|line| line.split_once('-'))
        .flat_map(|(lhs, rhs)| [(lhs, rhs), (rhs, lhs)])
        .fold(HashMap::new(), |mut graph, (lhs, rhs)| {
            graph.entry(lhs).or_default().push(rhs);
            graph
        })
}

check!(ex 1 = 10, ex 2 = 36, part 1 = 3761, part 2 = 99138);
