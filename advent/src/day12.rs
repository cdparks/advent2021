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
pub fn count_paths<'a>(graph: &'a Graph<'a>, mode: Mode) -> usize {
    let mut path = Path::new();
    let twice = match mode {
        Mode::Once => true,
        Mode::Twice => false,
    };
    walk(graph, &mut path, twice, graph.start)
}

fn walk<'a>(graph: &'a Graph<'a>, path: &mut Path, mut twice: bool, node: usize) -> usize {
    let kind = graph.kind(node);
    if kind == Kind::End {
        return 1;
    }

    if kind == Kind::Small && path.occurrences(node) > 0 {
        if twice {
            return 0;
        }
        twice = true;
    }

    path.push(node);

    let paths = graph
        .neighbors(node)
        .filter(|neighbor| graph.kind(**neighbor) != Kind::Start)
        .map(|neighbor| walk(graph, path, twice, *neighbor))
        .sum();

    path.pop();
    paths
}

#[derive(Debug)]
/// Specialized graph for this input.
pub struct Graph<'a> {
    len: usize,
    start: usize,
    end: usize,
    verts: [&'a str; 32],
    edges: [Vec<usize>; 32],
    kinds: [Kind; 32],
}

impl<'a> Graph<'a> {
    /// Create empty graph.
    pub fn new() -> Self {
        const EMPTY: Vec<usize> = Vec::new();
        Self {
            len: 0,
            start: 0,
            end: 0,
            verts: [""; 32],
            edges: [EMPTY; 32],
            kinds: [Kind::End; 32],
        }
    }

    /// Retrieve a vertex's index, adding it to the graph if necessary.
    pub fn vert(&mut self, key: &'a str) -> usize {
        match self.verts.iter().position(|vert| *vert == key) {
            Some(i) => i,
            None => {
                let i = self.len;
                self.verts[i] = key;
                self.len += 1;
                let kind = Kind::new(key);
                if kind == Kind::Start {
                    self.start = i;
                }
                if kind == Kind::End {
                    self.end = i;
                }
                self.kinds[i] = kind;
                i
            }
        }
    }

    /// Create an undirected edge between the two vertex indices.
    pub fn edge(&mut self, lhs: usize, rhs: usize) {
        self.edges[lhs].push(rhs);
        self.edges[rhs].push(lhs);
    }

    #[inline]
    /// Return the kind of vertex at this index.
    pub fn kind(&self, key: usize) -> Kind {
        self.kinds[key]
    }

    /// Yield a vertex's neighbors.
    pub fn neighbors(&'a self, key: usize) -> impl Iterator<Item = &'a usize> {
        self.edges[key].iter()
    }
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
/// The kind of node at an index in the graph.
pub enum Kind {
    /// Start node.
    Start,
    /// End node.
    End,
    /// Small "cave", one whose key was all lower-case.
    Small,
    /// Big "cave", one whose key was not all lower-case.
    Big,
}

impl Kind {
    /// Determine a node's kind.
    pub fn new(s: &str) -> Self {
        match s {
            "start" => Kind::Start,
            "end" => Kind::End,
            _ if s.to_lowercase() == s => Kind::Small,
            _ => Kind::Big,
        }
    }
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
pub struct Path {
    len: usize,
    path: [usize; 32],
}

impl Path {
    /// Create an empty path.
    pub fn new() -> Self {
        Self {
            len: 0,
            path: [0; 32],
        }
    }

    #[inline]
    /// Push a node onto the end of the path.
    fn push(&mut self, key: usize) {
        self.path[self.len] = key;
        self.len += 1;
    }

    #[inline]
    /// Pop the last node from the end of the path.
    fn pop(&mut self) {
        self.len -= 1;
    }

    #[inline]
    /// Count the number of times a node appears in the path.
    fn occurrences(&self, key: usize) -> usize {
        self.path[0..self.len]
            .iter()
            .filter(|node| **node == key)
            .count()
    }
}

fn parse_graph<'a>(lines: &'a [String]) -> Graph<'a> {
    lines.iter().flat_map(|line| line.split_once('-')).fold(
        Graph::new(),
        |mut graph, (lhs, rhs)| {
            let lhs = graph.vert(lhs);
            let rhs = graph.vert(rhs);
            graph.edge(lhs, rhs);
            graph
        },
    )
}

check!(ex 1 = 10, ex 2 = 36, part 1 = 3761, part 2 = 99138);
bench!(part 1, part 2);
