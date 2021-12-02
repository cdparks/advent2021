use std::str::FromStr;

fn part1(input: &[Command]) -> usize {
    use Command::*;

    let mut dist = 0;
    let mut depth = 0;

    for command in input {
        match command {
            Forward(n) => dist += n,
            Down(n) => depth += n,
            Up(n) => depth -= n,
        }
    }

    dist * depth
}

fn part2(input: &[Command]) -> usize {
    use Command::*;

    let mut aim = 0;
    let mut dist = 0;
    let mut depth = 0;

    for command in input {
        match command {
            Forward(n) => {
                dist += n;
                depth += n * aim;
            }
            Down(n) => aim += n,
            Up(n) => aim -= n,
        }
    }

    dist * depth
}

#[derive(PartialEq, Eq, Debug)]
enum Command {
    Forward(usize),
    Down(usize),
    Up(usize),
}

impl FromStr for Command {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Command::*;

        let (command, n) = s.split_once(' ').unwrap();
        let n = n.parse().unwrap();

        Ok(match command {
            "forward" => Forward(n),
            "down" => Down(n),
            "up" => Up(n),
            _ => unreachable!(),
        })
    }
}

check!(ex 1 = 150, ex 2 = 900, part 1 = 1746616, part 2 = 1741971043);
bench!(part 1, part 2);
