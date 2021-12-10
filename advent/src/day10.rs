use itertools::Itertools;

/// Score corrupted lines.
///
/// Corrupted lines have a mismatched closing bracket of some kind.
/// These can be identified by maintaining a stack of expected brackets
/// and comparing as they're popped off. Incomplete lines are ignored.
pub fn part1(lines: &[String]) -> u64 {
    fn score(c: char) -> u64 {
        match c {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => panic!("Unexpected character {:?}", c),
        }
    }

    lines
        .iter()
        .map(|line| {
            let mut stack = Vec::new();
            for c in line.chars() {
                if "([{<".contains(c) {
                    stack.push(close(c));
                    continue;
                }

                if let Some(expected) = stack.pop() {
                    if c != expected {
                        return score(c);
                    }
                }
            }
            0
        })
        .sum()
}

/// Score incomplete lines.
///
/// Incomplete lines are missing the final sequence of closing
/// brackets. These can be reconstructed by maintaining a stack of
/// expected brackets. Corrupted lines are ignored.
pub fn part2(lines: &[String]) -> u64 {
    fn score(c: char) -> u64 {
        match c {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => panic!("Unexpected character {:?}", c),
        }
    }

    let scores: Vec<u64> = lines
        .iter()
        .filter_map(|line| {
            let mut stack = Vec::new();
            for c in line.chars() {
                if "([{<".contains(c) {
                    stack.push(close(c));
                    continue;
                }

                if let Some(expected) = stack.pop() {
                    if c != expected {
                        return None;
                    }
                }
            }

            Some(stack.iter().rev().fold(0, |acc, &c| acc * 5 + score(c)))
        })
        .sorted()
        .collect();

    scores[scores.len() / 2]
}

fn close(c: char) -> char {
    match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => panic!("Unexpected character {:?}", c),
    }
}

check!(ex 1 = 26397, ex 2 = 288957, part 1 = 240123, part 2 = 3260812321);
bench!(part 1, part 2);
