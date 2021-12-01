#[allow(dead_code)]
fn part1(input: &[i32]) -> usize {
    input
      .windows(2)
      .filter(|window| match window {
        [x, y] => y > x,
        _ => false
      })
      .count()
}

#[allow(dead_code)]
fn part2(input: &[i32]) -> usize {
    input
      .windows(3)
      .filter_map(|window| match window {
        [x, y, z] => Some(x + y + z),
        _ => None
      })
      .collect::<Vec<i32>>()
      .windows(2)
      .filter(|window| match window {
        [x, y] => y > x,
        _ => false
      })
      .count()
}

solve!("01", ex 1 = 7, ex 2 = 5, part 1 = 1288, part 2 = 1311);
