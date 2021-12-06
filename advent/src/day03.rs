/// Calculate the power rating of the submarine.
///
/// The result is encoded as _gamma × epsilon_.
pub fn part1(input: &[String]) -> u32 {
    let mut ones = [0i32; 16];

    for line in input {
        for (i, bit) in line.chars().enumerate() {
            if bit == '1' {
                ones[i] += 1;
            } else if bit == '0' {
                ones[i] -= 1;
            }
        }
    }

    let mut gamma = 0;
    let mut epsilon = 0;
    let bits = input[0].len();

    for n in ones.iter().take(bits) {
        gamma *= 2;
        epsilon *= 2;
        if *n > 0 {
            gamma += 1;
        } else {
            epsilon += 1;
        }
    }

    gamma * epsilon
}

/// Calculate the life support rating of the submarine.
///
/// The result is encoded as _O₂ generator rating × CO₂ scrubber rating_.
pub fn part2(input: &[String]) -> u32 {
    use Rating::*;
    rate(O2, input) * rate(CO2, input)
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
/// The type of rating to calculate.
pub enum Rating {
    /// Calculate O₂ rating using the most common bit.
    O2,
    /// Calcualte CO₂ rating using least common bit.
    CO2,
}

/// Calculate O₂ or CO₂ rating.
///
/// For each bit position, find the most common bit.
///   * To find the O₂ rating, keep only the inputs whose bit at that
///     position matches the most common bit.
///   * To find the CO₂ rating, keep only the inputs whose bit at that
///     position does _not_ match the most common bit.
///
/// Repeat until you run out of bit positions or there is only one input
/// left. The last input is the rating.
///
/// # Examples
///
/// ```rust
/// use advent::day03::*;
/// let input: Vec<String> = ["01", "10", "10"]
///     .iter()
///     .map(|s| String::from(*s))
///     .collect();
/// assert_eq!(rate(Rating::O2, &input), 0b10);
/// assert_eq!(rate(Rating::CO2, &input), 0b01);
/// ```
pub fn rate(rating: Rating, input: &[String]) -> u32 {
    use Rating::*;
    let (one, zero) = match rating {
        O2 => (b'1', b'0'),
        CO2 => (b'0', b'1'),
    };

    let mut lines = Vec::from(input);
    let bits = input[0].len();

    for i in 0..bits {
        if lines.len() == 1 {
            break;
        }

        let mut ones = 0i32;
        for line in lines.iter() {
            let bit = line.as_bytes()[i];
            if bit == b'1' {
                ones += 1
            } else {
                ones -= 1
            }
        }

        let bit = if ones >= 0 { one } else { zero };
        lines.retain(|line| line.as_bytes()[i] == bit);
    }

    u32::from_str_radix(&lines[0], 2).unwrap()
}

check!(ex 1 = 198, ex 2 = 230, part 1 = 775304, part 2 = 1370737);
bench!(part 1, part 2);
