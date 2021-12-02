/// Count the number of times a depth measurement increases from the
/// previous measurement.
pub fn part1(input: &[i32]) -> usize {
    count(2, input)
}

/// Count the number of times the sum of measurements in this sliding
/// window increases from the previous sum.
pub fn part2(input: &[i32]) -> usize {
    count(4, input)
}

#[inline]
/// Count windows of `size` in the `input` that sum to a value larger
/// than the previous window.
///
/// # Examples
///
/// ```rust
/// assert_eq!(advent::day01::count(2, &[1, 2, 1, 3, 5]), 3);
/// ```
///
/// # Panics
///
/// Panics if the window size is zero.
///
/// ```rust,should_panic
/// advent::day01::count(0, &[1, 2, 3]);
/// ```
pub fn count(size: usize, input: &[i32]) -> usize {
    input
        .windows(size)
        .filter(|window| window.last() > window.first())
        .count()
}

check!(ex 1 = 7, ex 2 = 5, part 1 = 1288, part 2 = 1311);
bench!(part 1, part 2);
