use itertools::Itertools;

/// Find minimum fuel cost when each step uses 1 unit of fuel.
pub fn part1(lines: &[String]) -> i64 {
    min_fuel_cost(lines, |i, j| (j - i).abs())
}

/// Find minimum fuel cost when each step uses 1 unit of fuel more than the previous step.
pub fn part2(lines: &[String]) -> i64 {
    min_fuel_cost(lines, |i, j| {
        let n = (j - i).abs();
        n * (n + 1) / 2
    })
}

/// Find minimum fuel cost by applying cost function to each position and possible distance.
pub fn min_fuel_cost<F>(lines: &[String], cost: F) -> i64
where
    F: Fn(i64, i64) -> i64,
{
    let positions: Vec<i64> = lines[0].split(',').flat_map(str::parse).collect();
    let (&min, &max) = positions.iter().minmax().into_option().unwrap();
    (min..=max)
        .map(|i| positions.iter().map(|&j| cost(i, j)).sum())
        .min()
        .unwrap()
}

check!(ex 1 = 37, ex 2 = 168, part 1 = 335330, part 2 = 92439766);
bench!(part 1, part 2);
