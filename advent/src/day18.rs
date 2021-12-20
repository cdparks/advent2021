use itertools::Itertools;
use std::iter::Peekable;
use std::ops::Add;
use std::str::Chars;
use std::str::FromStr;

/// Compute the total sum of the snailfish numbers.
pub fn part1(nodes: &[Node]) -> u64 {
    let node = nodes
        .iter()
        .cloned()
        .reduce(|lhs, rhs| lhs.add(rhs))
        .unwrap();
    node.magnitude()
}

/// Find the largest magnitude computable by adding any two snailfish numbers.
pub fn part2(nodes: &[Node]) -> u64 {
    nodes
        .iter()
        .cloned()
        .cartesian_product(nodes.iter().cloned())
        .fold(0, |acc, (lhs, rhs)| {
            let x = lhs.clone() + rhs.clone();
            let y = rhs + lhs;
            acc.max(x.magnitude()).max(y.magnitude())
        })
}

#[derive(Debug, Clone)]
/// Binary tree with values at the leaves only.
pub enum Node {
    /// Leaf is a non-negative number.
    Number(u64),
    /// Branching node.
    Pair(Box<Node>, Box<Node>),
}

impl Add for Node {
    type Output = Node;
    /// Add two snailfish numbers together and reduce the result.
    fn add(self, other: Node) -> Self {
        let mut node = Node::Pair(Box::new(self), Box::new(other));
        while node.explode() || node.split() {}
        node
    }
}

impl Node {
    /// Compute the total magnitude of the snailfish number.
    pub fn magnitude(&self) -> u64 {
        match self {
            Node::Number(i) => *i,
            Node::Pair(lhs, rhs) => 3 * lhs.magnitude() + 2 * rhs.magnitude(),
        }
    }

    /// Attempt to interpret node as a number.
    pub fn as_number(&self) -> Option<u64> {
        match *self {
            Node::Number(i) => Some(i),
            Node::Pair(_, _) => None,
        }
    }

    /// Return true if any number exploded
    pub fn explode(&mut self) -> bool {
        self._explode(0).is_some()
    }

    /// Return the pair of next deepest numbers that exploded.
    fn _explode(&mut self, depth: usize) -> Option<(u64, u64)> {
        match self {
            Node::Number(_) => None,
            Node::Pair(lhs, rhs) => {
                if depth == 4 {
                    let x = lhs.as_number()?;
                    let y = rhs.as_number()?;
                    *self = Node::Number(0);
                    Some((x, y))
                } else if let Some((x, y)) = lhs._explode(depth + 1) {
                    rhs.add_leftmost(y);
                    Some((x, 0))
                } else if let Some((x, y)) = rhs._explode(depth + 1) {
                    lhs.add_rightmost(x);
                    Some((0, y))
                } else {
                    None
                }
            }
        }
    }

    /// Split numbers _>= 10_ into a pair.
    pub fn split(&mut self) -> bool {
        match self {
            Node::Number(i) if *i >= 10 => {
                let div2 = *i as f64 / 2.0;
                let lhs = Box::new(Node::Number(div2.floor() as u64));
                let rhs = Box::new(Node::Number(div2.ceil() as u64));
                *self = Node::Pair(lhs, rhs);
                true
            }
            Node::Number(_) => false,
            Node::Pair(lhs, rhs) => lhs.split() || rhs.split(),
        }
    }

    /// Add the value to the leftmost leaf.
    fn add_leftmost(&mut self, x: u64) {
        match self {
            Node::Number(n) => *n += x,
            Node::Pair(lhs, _) => lhs.add_leftmost(x),
        }
    }

    /// Add the value to the rightmost leaf.
    fn add_rightmost(&mut self, x: u64) {
        match self {
            Node::Number(n) => *n += x,
            Node::Pair(_, rhs) => rhs.add_rightmost(x),
        }
    }
}

impl FromStr for Node {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(parse(&mut Stream::new(s)).unwrap())
    }
}

/// Parse a Node from a Stream.
pub fn parse(stream: &mut Stream<'_>) -> Option<Node> {
    let c = stream.peek()?;
    if c == '[' {
        parse_node(stream)
    } else if c.is_digit(10) {
        parse_num(stream)
    } else {
        None
    }
}

/// Parse a Node from a Stream.
pub fn parse_node(stream: &mut Stream<'_>) -> Option<Node> {
    stream.expect('[')?;
    let lhs = parse(stream)?;
    stream.expect(',')?;
    let rhs = parse(stream)?;
    stream.expect(']')?;
    Some(Node::Pair(Box::new(lhs), Box::new(rhs)))
}

/// Parse a number from a Stream.
pub fn parse_num(stream: &mut Stream<'_>) -> Option<Node> {
    let mut n = 0;
    while let Some(digit) = stream.digit() {
        n = n * 10 + digit;
    }
    Some(Node::Number(n))
}

#[derive(Debug)]
/// Ergonomic wrapper for a peekable character iterator.
pub struct Stream<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> Stream<'a> {
    /// Create a new Stream from a string.
    pub fn new(s: &'a str) -> Self {
        Self {
            chars: s.chars().peekable(),
        }
    }

    /// Peek the next character.
    pub fn peek(&mut self) -> Option<char> {
        let &c = self.chars.peek()?;
        Some(c)
    }

    /// Consume the next character.
    pub fn drop(&mut self) -> Option<()> {
        self.chars.next()?;
        Some(())
    }

    /// Consume the matching character or fail.
    pub fn expect(&mut self, c: char) -> Option<()> {
        let x = self.chars.next()?;
        guard(x == c)
    }

    /// Parse a digit or fail.
    pub fn digit(&mut self) -> Option<u64> {
        let c = self.peek()?;
        guard(c.is_digit(10))?;
        self.drop()?;
        Some(c.to_digit(10)? as u64)
    }
}

/// Continue the computation if the condition holds.
pub fn guard(cond: bool) -> Option<()> {
    if cond {
        Some(())
    } else {
        None
    }
}

check!(ex 1 = 4140, ex 2 = 3993, part 1 = 4480, part 2 = 4676);
bench!(part 1, part 2);
