/// Compute sum of all packet versions
pub fn part1(lines: &[String]) -> u64 {
    Bits::parse(lines).decode().version()
}

/// Evaluate the expression embedded in the packet.
pub fn part2(lines: &[String]) -> u64 {
    Bits::parse(lines).decode().eval()
}

#[derive(PartialEq, Eq, Debug)]
/// A packet in the Buoyancy Interchange Transmission System protocol.
pub struct Packet {
    version: u64,
    node: Node,
}

impl Packet {
    /// Create a new packet from a version and node.
    pub fn new(version: u64, node: Node) -> Self {
        Self { version, node }
    }

    /// The sum of a packet's version and its children's version sums.
    pub fn version(&self) -> u64 {
        self.version + self.node.version()
    }

    /// Evaluate the packet operations
    pub fn eval(&self) -> u64 {
        self.node.eval()
    }
}

#[derive(PartialEq, Eq, Debug)]
/// A literal or an operation to perform on the node's children.
pub enum Node {
    /// Literal unsigned integer.
    Literal(u64),
    /// Sum of the children's values.
    Sum(Vec<Packet>),
    /// Product of the children's values.
    Product(Vec<Packet>),
    /// Minimum of the children's values.
    Min(Vec<Packet>),
    /// Maximum of the children's values.
    Max(Vec<Packet>),
    /// 1 if _left-hand-side > right-hand-side_ else 0.
    GreaterThan(Box<Packet>, Box<Packet>),
    /// 1 if _left-hand-side < right-hand-side_ else 0.
    LessThan(Box<Packet>, Box<Packet>),
    /// 1 if _left-hand-side == right-hand-side_ else 0.
    EqualTo(Box<Packet>, Box<Packet>),
}

impl Node {
    /// The sum of a packet's version and its children's version sums.
    pub fn version(&self) -> u64 {
        match self {
            Self::Literal(_) => 0,
            Self::Sum(packets) => packets.iter().map(Packet::version).sum(),
            Self::Product(packets) => packets.iter().map(Packet::version).sum(),
            Self::Min(packets) => packets.iter().map(Packet::version).sum(),
            Self::Max(packets) => packets.iter().map(Packet::version).sum(),
            Self::GreaterThan(lhs, rhs) => lhs.version() + rhs.version(),
            Self::LessThan(lhs, rhs) => lhs.version() + rhs.version(),
            Self::EqualTo(lhs, rhs) => lhs.version() + rhs.version(),
        }
    }

    /// Evaluate the packet operations
    pub fn eval(&self) -> u64 {
        match self {
            Self::Literal(n) => *n,
            Self::Sum(packets) => packets.iter().map(Packet::eval).sum(),
            Self::Product(packets) => packets.iter().map(Packet::eval).product(),
            Self::Min(packets) => packets.iter().map(Packet::eval).min().unwrap(),
            Self::Max(packets) => packets.iter().map(Packet::eval).max().unwrap(),
            Self::GreaterThan(lhs, rhs) => (lhs.eval() > rhs.eval()) as u64,
            Self::LessThan(lhs, rhs) => (lhs.eval() < rhs.eval()) as u64,
            Self::EqualTo(lhs, rhs) => (lhs.eval() == rhs.eval()) as u64,
        }
    }
}

#[derive(Debug)]
/// Mutable stream of bits.
pub struct Bits {
    bits: Vec<bool>,
    index: usize,
}

const VERSION_ID: usize = 3;
const TYPE_ID: usize = 3;
const LITERAL_TYPE: u64 = 4;
const LENGTH_ID: usize = 1;
const BIT_LEN: usize = 15;
const NUM_PACKETS: usize = 11;
const STOP: usize = 1;
const CHUNK: usize = 4;
const SHIFT: u64 = 4;

impl Bits {
    /// Parse bit stream from a sequence of lines.
    pub fn parse(lines: &[String]) -> Self {
        let bits = lines
            .join("")
            .chars()
            .flat_map(|c| c.to_digit(16).map(|d| d as u8))
            .flat_map(|byte| [8, 4, 2, 1].map(|mask| byte & mask == mask))
            .collect();
        Self { bits, index: 0 }
    }

    /// Decode and consume bits to produce a (potentially nested) packet.
    pub fn decode(mut self) -> Packet {
        self.unpack()
    }

    /// Recursive procedure borrows self in decode.
    fn unpack(&mut self) -> Packet {
        let version = self.take(VERSION_ID);
        let node = match self.take(TYPE_ID) {
            0 => Node::Sum(self.subpackets()),
            1 => Node::Product(self.subpackets()),
            2 => Node::Min(self.subpackets()),
            3 => Node::Max(self.subpackets()),
            4 => Node::Literal(self.literal()),
            5 => binop(self.subpackets(), Node::GreaterThan),
            6 => binop(self.subpackets(), Node::LessThan),
            7 => binop(self.subpackets(), Node::EqualTo),
            type_id => panic!("Invalid type id: {}", type_id),
        };
        Packet::new(version, node)
    }

    /// Parse a variable length literal from the stream.
    fn literal(&mut self) -> u64 {
        let mut result = 0;
        loop {
            let stop = self.take(STOP);
            result = (result << SHIFT) + self.take(CHUNK);
            if stop == 0 {
                return result;
            }
        }
    }

    /// Parse a packet's subpackets from the stream.
    fn subpackets(&mut self) -> Vec<Packet> {
        let mut packets = Vec::new();
        if self.take(LENGTH_ID) == 0 {
            let bit_len = self.take(BIT_LEN);
            let mut n = 0;
            while n < bit_len {
                let start = self.remaining();
                packets.push(self.unpack());
                n += (start - self.remaining()) as u64;
            }
        } else {
            let num_packets = self.take(NUM_PACKETS);
            for _ in 0..num_packets {
                packets.push(self.unpack());
            }
        }
        packets
    }

    /// Parse _n_ bits from the stream into an unsigned integer.
    fn take(&mut self, n: usize) -> u64 {
        let len = self.index + n;
        let value = self.bits[self.index..len]
            .iter()
            .rev()
            .enumerate()
            .map(|(i, &bit)| (bit as u64) << i)
            .sum();
        self.index += n;
        value
    }

    /// Remaining number of bits left in the stream.
    fn remaining(&self) -> usize {
        self.bits.len() - self.index
    }
}

/// Construct a binary operation if given two operands
fn binop<F>(mut packets: Vec<Packet>, new: F) -> Node
where
    F: Fn(Box<Packet>, Box<Packet>) -> Node,
{
    assert_eq!(packets.len(), 2, "Binary operator without 2 operands");
    let rhs = packets.pop().unwrap();
    let lhs = packets.pop().unwrap();
    new(Box::new(lhs), Box::new(rhs))
}

check!(ex 1 = 31, ex 2 = 54, part 1 = 877, part 2 = 194435634456);
bench!(part 1, part 2);
