extern crate day;

#[macro_export]
/// Generate tests using the solution's example and input files.
///
/// # Examples
///
/// In a file named `path/to/day00.rs`, the following invocation:
///
/// ```rust,no_run
/// advent::check!(ex 1 = "hello", part 1 = "world", ex 1 = DIFF);
/// ```
///
/// expands to
///
/// ```rust,no_run
/// #[cfg(test)]
/// pub mod tests {
///     pub (crate) const EXAMPLE: &str = include_str!("../inputs/00/example");
///     pub (crate) const INPUT: &str = include_str!("../inputs/00/input");
///
///     #[test]
///     pub fn example1() {
///         let parsed = crate::check::parse_lines(EXAMPLE);
///         assert_eq!(super::part1(&parsed), "hello")
///     }
///
///     #[test]
///     pub fn part1() {
///         let parsed = crate::check::parse_lines(INPUT);
///         assert_eq!(super::part1(&parsed), "world")
///     }
///
///     #[test]
///     pub fn diff_example1() {
///         let expected: &str = include_str!("../outputs/00/example");
///         let parsed = crate::check::parse_lines(EXAMPLE);
///         let result = super::part1(&parsed);
///         if result != expected {
///             panic!("\nResult:\n{}\ndoes not match expected:\n{}\n", result, expected);
///         } else {
///             // See output with `cargo test -- --nocapture`
///             println!("\n{}", result);
///         }
///     }
/// }
/// ```
macro_rules! check {
    ($($tail:tt)+) => {
        #[cfg(test)]
        pub mod tests {
            pub (crate) const EXAMPLE: &str = include_str!(concat!("../inputs/", day::day!(), "/example"));
            pub (crate) const INPUT: &str = include_str!(concat!("../inputs/", day::day!(), "/input"));
            gen!($($tail)+);
        }
    };
}

#[doc(hidden)]
#[cfg(test)]
macro_rules! gen {
    (diff, $input:ident, $prefix:ident, $n:literal, $filename:expr) => {
        paste::paste! {
            #[test]
            pub fn [<diff_ $prefix $n>]() {
                let expected: &str = include_str!(concat!("../outputs/", day::day!(), "/", $filename));
                let parsed = crate::check::parse_lines($input);
                let result = super::[<part $n>](&parsed);
                if result != expected {
                    panic!("\nResult:\n{}\ndoes not match expected:\n{}\n", result, expected);
                } else {
                    // See output with `cargo test -- --nocapture`
                    println!("\n{}", result);
                }
            }
        }
    };

    (test, $input:ident, $prefix:ident, $n:literal, $expected:literal) => {
        paste::paste! {
            #[test]
            pub fn [<$prefix $n>]() {
                let parsed = crate::check::parse_lines($input);
                let now = std::time::Instant::now();
                let result = super::[<part $n>](&parsed);
                let elapsed = now.elapsed();
                assert_eq!(result, $expected, "elapsed: {:?}", elapsed)
            }
        }
    };

    (ex $n:literal = DIFF) => {
        gen!(diff, EXAMPLE, example, $n, "example");
    };

    (ex $n:literal = DIFF, $($tail:tt)+) => {
        gen!(ex $n = DIFF);
        gen!($($tail)+);
    };

    (ex $n:literal = $expected:expr) => {
        gen!(test, EXAMPLE, example, $n, $expected);
    };

    (ex $n:literal = $expected:expr, $($tail:tt)+) => {
        gen!(ex $n = $expected);
        gen!($($tail)+);
    };

    (part $n:literal = DIFF) => {
        gen!(diff, INPUT, part, $n, "output");
    };

    (part $n:literal = DIFF, $($tail:tt)+) => {
        gen!(part $n = DIFF);
        gen!($($tail)+);
    };

    (part $n:literal = $expected:expr) => {
        gen!(test, INPUT, part, $n, $expected);
    };

    (part $n:literal = $expected:expr, $($tail:tt)+) => {
        gen!(part $n = $expected);
        gen!($($tail)+);
    };
}

#[doc(hidden)]
#[cfg(test)]
pub(crate) fn parse_lines<T: std::str::FromStr>(text: &str) -> Vec<T> {
    text.lines().flat_map(str::parse).collect::<Vec<T>>()
}
