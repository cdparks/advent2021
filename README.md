# Advent of Code 2021

Solutions for ✨[Advent of Code 2021]✨ in [Rust]

|  ❄️   |  🎁  |  ⛄  |  🔔  |  🎄  |
| :--: | :--: | :--: | :--: | :--: |
| 1 ⭐ | 2 ⭐ | 3 ⭐ | 4 ⭐ | 5 ⭐ |
| 6 ⭐ | 7 ⭐ | 8 ⭐ | 9 ⭐ | 10⭐ |
| 11⭐ | 12⭐ | 13⭐ | 14⭐ | 15⭐ |
| 16⭐ | 17⭐ | 18⭐ | 19⭐ | 20⭐ |
| 21⭐ | 22⭐ | 23⭐ | 24⭐ | 25⭐ |

Each solution is in `advent/src/dayXX.rs`. Run `cargo test --release`
to check solutions and `cargo bench` to run benchmarks.

## Visualizations

Generated from hacked up versions of the solutions in [this branch][cp/viz].

### Day 9

Finds the three largest contiguous basins. Brighter colors indicate
deeper points.

![Day 9][day09]

### Day 13

Folds the 2D space until the overlapping points produce a sequence of
(meaningless) letters.

![Day 13][day13]

### Day 15

Finds the least expensive path from the top-left corner to the bottom-
right corner. Brighter colors indicate more expensive points.

![Day 15][day15]

### Day 25

Red points move east. Blue points move south. Runs until no point can
move without colliding with another point.

![Day 25][day25]

[Advent of Code 2021]: https://adventofcode.com/2021
[Rust]: https://www.rust-lang.org/
[cp/viz]: https://github.com/cdparks/advent2021/tree/cp/viz
[day09]: https://github.com/cdparks/advent2021/blob/main/gifs/day09.gif?raw=true
[day13]: https://github.com/cdparks/advent2021/blob/main/gifs/day13.gif?raw=true
[day15]: https://github.com/cdparks/advent2021/blob/main/gifs/day15.gif?raw=true
[day25]: https://github.com/cdparks/advent2021/blob/main/gifs/day25.gif?raw=true
