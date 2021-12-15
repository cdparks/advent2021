use itertools::{Itertools, MinMaxResult};
use std::collections::HashMap;

/// Evolve the polymer for 10 steps.
pub fn part1(lines: &[String]) -> usize {
    let (template, rules) = parse(lines);
    solve(10, &template, &rules)
}

/// Evolve the polymer for 40 steps.
pub fn part2(lines: &[String]) -> usize {
    let (template, rules) = parse(lines);
    solve(40, &template, &rules)
}

/// Evolve the polymer for _n_ steps.
pub fn solve(n: usize, template: &str, rules: &HashMap<(char, char), char>) -> usize {
    let counts = template.chars().tuple_windows().counts();
    let counts = (0..n).fold(counts, |counts, _| evolve(counts, rules));
    let last = template.chars().last().unwrap();
    diff(last, counts)
}

/// Find the difference in quantity between the most and least common element.
pub fn diff(last: char, counts: HashMap<(char, char), usize>) -> usize {
    let mut totals: HashMap<char, usize> =
        counts
            .iter()
            .fold(HashMap::new(), |mut totals, (&(lhs, _), n)| {
                *totals.entry(lhs).or_default() += n;
                totals
            });

    // Last character is off-by-one since we're counting pairs.
    *totals.entry(last).or_default() += 1;

    if let MinMaxResult::MinMax(least, most) = totals.values().minmax() {
        most - least
    } else {
        0
    }
}

/// Apply a rule for each pair in the input.
pub fn evolve(
    counts: HashMap<(char, char), usize>,
    rules: &HashMap<(char, char), char>,
) -> HashMap<(char, char), usize> {
    counts
        .iter()
        .fold(HashMap::new(), |mut counts, (&(lhs, rhs), n)| {
            let x = rules[&(lhs, rhs)];
            *counts.entry((lhs, x)).or_default() += n;
            *counts.entry((x, rhs)).or_default() += n;
            counts
        })
}

fn parse(lines: &[String]) -> (String, HashMap<(char, char), char>) {
    let template = lines[0].clone();
    let rules: HashMap<(char, char), char> = lines[2..]
        .iter()
        .flat_map(|line| line.split_once(" -> "))
        .flat_map(|(lhs, rhs)| lhs.chars().chain(rhs.chars()).collect_tuple())
        .map(|(a, b, c)| ((a, b), c))
        .collect();
    (template, rules)
}

check!(ex 1 = 1588, ex 2 = 2188189693529, part 1 = 2967, part 2 = 3692219987038);
bench!(part 1, part 2);
