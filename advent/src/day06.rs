use std::collections::VecDeque;

/// Simulate lanternfish for 80 days.
pub fn part1(lines: &[String]) -> u64 {
    simulate(80, lines)
}

/// Simulate lanternfish for 256 days.
pub fn part2(lines: &[String]) -> u64 {
    simulate(256, lines)
}

/// Simulate lanternfish for the specified number of days.
///
/// Group each fish by the number of days remaining before that fish
/// produces a new fish. The cycle is 7 days long, but new fish start
/// with a timer of 8, so we can use an array of size 9. For example,
/// with this input:
///
/// ```plaintext
/// 3,4,3,1,2
/// ```
///
/// We can group each fish into the following buckets:
/// ```plaintext
/// 0: 0 // 0 fish with 0 days left
/// 1: 1 // 1 fish with 1 day left
/// 2: 1 // etc.
/// 3: 2
/// 4: 1
/// 5: 0
/// 6: 0
/// 7: 0
/// 8: 0
/// ```
///
/// To add new fish to the end, we rotate our counters left, e.g. after 2 days:
///
/// ```plaintext
///    0 1 2
/// +-------> days
/// |  0
/// v  1 1
/// 0: 1 1 1
/// 1: 2 2 2
/// 2: 1 1 1
/// 3: 0 0 0
/// 4: 0 0 0
/// 5: 0 0 0
/// 6: 0 0 0
/// 7:   0 0
/// 8:     1
/// |
/// +-------> fish
///    5 5 …
/// ```
///
/// Then, to put our original fish back into the cycle, _add_ the number
/// of fish at the end (those just added) to index 6 (7 days left):
/// ```plaintext
///    0 1         2
/// +---------------> days
/// |  0
/// v  1 1
/// 0: 1 1 1       1
/// 1: 2 2 2       1
/// 2: 1 1 1       1
/// 3: 0 0 0       0
/// 4: 0 0 0       0
/// 5: 0 0 0       0
/// 6: 0 0 0 + 1 = 1
/// 7:   0 0   |   0
/// 8:     1 <-+   1
/// |
/// +---------------> fish
///    5 5         6
/// ```
///
/// After 2 more days:
/// ```plaintext
///    0 1         2         3         4
/// +-----------------------------------> days
/// |  0
/// |  1 1
/// |  1 1 1       1
/// v  2 2 2       2 2       2
/// 0: 1 1 1       1 1       1 1       1
/// 1: 0 0 0       0 0       0 0       0
/// 2: 0 0 0       0 0       0 0       0
/// 3: 0 0 0       0 0       0 0       0
/// 4: 0 0 0 + 1 = 1 1       1 1       1
/// 5:   0 0   |   0 0 + 1 = 1 1       1
/// 6:     1 <-+   1 1   |   1 1 + 2 = 3
/// 7:               1 <-+   1 1   |   1
/// 8:                         2 <-+   2
/// |
/// +-----------------------------------> fish
///    5 5         6         7         9
/// ```
///
/// After simulating for the specified number of days, return the sum
/// of the elements of the array, which is the total number of fish in
/// the system.
pub fn simulate(days: usize, lines: &[String]) -> u64 {
    let mut counters: VecDeque<u64> =
        lines[0]
            .split(',')
            .flat_map(str::parse)
            .fold(VecDeque::from([0; 9]), |mut counters, i| {
                counters[i] += 1;
                counters
            });

    for _ in 0..days {
        counters.rotate_left(1);
        counters[6] += counters[8];
    }

    counters.into_iter().sum()
}

check!(ex 1 = 5934, ex 2 = 26984457539, part 1 = 358214, part 2 = 1622533344325);
bench!(part 1, part 2);
