use cached::proc_macro::cached;
use sscanf::scanf;

/// Play the game with a deterministic 100-sided die. Return the loser's
/// score x the number of rolls made.
pub fn part1(lines: &[String]) -> usize {
    let mut players = parse(lines).unwrap();
    let mut scores = (0, 0);
    let mut rolls = 0;
    let mut die = (1..=100).cycle();

    loop {
        let steps: usize = (0..3).flat_map(|_| die.next()).sum();
        rolls += 3;

        players.0 = ((players.0 - 1) + steps) % 10 + 1;
        scores.0 += players.0;

        if scores.0 >= 1000 {
            return scores.1 * rolls;
        }

        players = swap(players);
        scores = swap(scores);
    }
}

/// Play the game with a quantum 3-sided die. Find the player that wins
/// in the most universes, and return that number of universes.
pub fn part2(lines: &[String]) -> usize {
    let players = parse(lines).unwrap();
    let universes = solve(players.0, players.1, 0, 0);
    universes.0.max(universes.1)
}

#[cached]
/// Memoized solver using 4 usizes as the key.
///
/// Base case: either score is _>= 21_, so count that universe for the
/// corresponding player.
///
/// Recursive case: move player 1 according to each possible step,
/// adjusting their score as necessary. Then rotate player2 to player1's
/// position and recurse with player1's new position and score. The
/// result is the number of universes in which each player won.
/// Multiply those counts by the number of times the steps at this
/// level can be rolled.
fn solve(player1: usize, player2: usize, score1: usize, score2: usize) -> (usize, usize) {
    if score1 >= 21 {
        return (1, 0);
    }

    if score2 >= 21 {
        return (0, 1);
    }

    STEPS.iter().fold((0, 0), |wins, (steps, n)| {
        let pos = ((player1 - 1) + steps) % 10 + 1;
        let universes = swap(solve(player2, pos, score2, score1 + pos));
        (wins.0 + n * universes.0, wins.1 + n * universes.1)
    })
}

/// Steps mapped to the number of ways those steps can be rolled with
/// 3 3-sided dice rolls. For example, there's only 1 way to roll 3
/// (1+1+1), but there are 3 ways to roll 4 (1+1+2, 1+ 2+1, 2+1+1).
const STEPS: [(usize, usize); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

fn swap<A, B>(pair: (A, B)) -> (B, A) {
    (pair.1, pair.0)
}

fn parse(lines: &[String]) -> Option<(usize, usize)> {
    let (_, player1) = scanf!(lines[0], "Player {} starting position: {}", usize, usize)?;
    let (_, player2) = scanf!(lines[1], "Player {} starting position: {}", usize, usize)?;
    Some((player1, player2))
}

check!(ex 1 = 739785, ex 2 = 444356092776315, part 1 = 671580, part 2 = 912857726749764);
bench!(part 1, part 2);
