use std::convert::TryInto;
use std::ops::{BitAnd, BitOr, Index, IndexMut};
use std::str::FromStr;

/// Count number of outputs uniquely identified by their segment length.
pub fn part1(entries: &[Entry]) -> usize {
    entries
        .iter()
        .flat_map(|entry| entry.outputs.iter().cloned())
        .filter(|pattern| [2, 4, 3, 7].contains(&pattern.len()))
        .count()
}

/// Deduce each display's outputs and sum the results.
pub fn part2(entries: &[Entry]) -> u64 {
    entries.iter().map(solve_entry).sum()
}

/// Deduce a single display's output.
pub fn solve_entry(entry: &Entry) -> u64 {
    let mut digits = [Pattern::default(); 10];

    // Find simple cases first
    entry
        .patterns
        .iter()
        .for_each(|&pattern| match pattern.len() {
            2 => digits[1] = pattern,
            3 => digits[7] = pattern,
            4 => digits[4] = pattern,
            7 => digits[8] = pattern,
            _ => {}
        });

    // Deduce remaining cases
    entry
        .patterns
        .iter()
        .for_each(|&pattern| match pattern.len() {
            5 if (pattern & digits[1]) == digits[1] => digits[3] = pattern,
            5 if (pattern | digits[4]) == digits[8] => digits[2] = pattern,
            5 => digits[5] = pattern,
            6 if (pattern & digits[4]) == digits[4] => digits[9] = pattern,
            6 if (pattern & digits[1]) == digits[1] => digits[0] = pattern,
            6 => digits[6] = pattern,
            _ => {}
        });

    let mut patterns = [0; 256];
    digits
        .iter()
        .enumerate()
        .for_each(|(i, &pattern)| patterns[pattern] = i as u8);

    // Match output to known patterns
    entry
        .outputs
        .iter()
        .fold(0, |acc, &pattern| acc * 10 + patterns[pattern] as u64)
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
/// A single entry in the input representing a 7-segment display.
pub struct Entry {
    patterns: [Pattern; 10],
    outputs: [Pattern; 4],
}

impl FromStr for Entry {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (lhs, rhs) = s.split_once(" | ").unwrap();
        let lhs: Vec<Pattern> = lhs.split_whitespace().flat_map(str::parse).collect();
        let rhs: Vec<Pattern> = rhs.split_whitespace().flat_map(str::parse).collect();
        Ok(Self {
            patterns: lhs.try_into().unwrap(),
            outputs: rhs.try_into().unwrap(),
        })
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
/// A segment pattern representing some digit.
///
/// The pattern is a bitfield where a 1 bit represents a segment used
/// in the digit. For example, the number 4 could be represented by
/// `"bcdf"`:
/// ```plaintext
///  ....
/// b    c
/// b    c
///  dddd
/// .    f
/// .    f
///  ....
///  ```
///
/// This would be encoded as follows:
///
/// ```plaintext
///   gfedcba
/// 0b0101110
/// ```
pub struct Pattern {
    byte: u8,
}

impl Pattern {
    #[inline]
    /// Create a new pattern from a byte.
    pub fn new(byte: u8) -> Self {
        Self { byte }
    }

    #[inline]
    /// Return the pattern's length.
    ///
    /// The pattern's length is actually its population count.
    pub fn len(&self) -> u8 {
        self.byte.count_ones() as u8
    }
}

impl Default for Pattern {
    #[inline]
    fn default() -> Self {
        Pattern::new(0)
    }
}

impl BitAnd for Pattern {
    type Output = Pattern;

    #[inline]
    fn bitand(self, rhs: Self) -> Self::Output {
        Pattern::new(self.byte & rhs.byte)
    }
}

impl BitOr for Pattern {
    type Output = Pattern;

    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Pattern::new(self.byte | rhs.byte)
    }
}

impl<T> Index<Pattern> for [T] {
    type Output = T;

    #[inline]
    fn index(&self, pattern: Pattern) -> &Self::Output {
        self.index(pattern.byte as usize)
    }
}

impl<T> IndexMut<Pattern> for [T] {
    #[inline]
    fn index_mut(&mut self, pattern: Pattern) -> &mut Self::Output {
        self.index_mut(pattern.byte as usize)
    }
}

impl FromStr for Pattern {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut byte = 0;
        for c in s.bytes() {
            assert!(b"abcdefg".contains(&c));
            byte |= 1 << (c - b'a');
        }
        Ok(Pattern::new(byte))
    }
}

check!(ex 1 = 26, ex 2 = 61229, part 1 = 342, part 2 = 1068933);
bench!(part 1, part 2);
