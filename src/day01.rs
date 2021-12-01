#[allow(dead_code)]
fn part1(input: &[i32]) -> usize {
    count(2, input)
}

#[allow(dead_code)]
fn part2(input: &[i32]) -> usize {
    count(4, input)
}

#[inline]
fn count(size: usize, input: &[i32]) -> usize {
    input
        .windows(size)
        .filter(|window| window.last() > window.first())
        .count()
}

check!("01", ex 1 = 7, ex 2 = 5, part 1 = 1288, part 2 = 1311);
bench!("01", part 1, part 2);
