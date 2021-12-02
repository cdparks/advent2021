extern crate day;

#[macro_export]
/// Generate tests against the solution's example and input files
///
/// # Examples
///
/// In a file named `path/to/day00.rs`, the following invocation:
///
/// ```rust,no_run
/// advent::check!(ex 1 = "hello", part 1 = "world");
/// ```
///
/// expands to
///
/// ```rust,no_run
/// #[cfg(test)]
/// pub mod tests {
///   pub (crate) const EXAMPLE: &str = include_str!("../inputs/00/example");
///   pub (crate) const INPUT: &str = include_str!("../inputs/00/input");
///
///   pub fn example1() {
///       let parsed = crate::check::parse_lines(EXAMPLE);
///       assert_eq!(super::part1(&parsed), "hello")
///   }
///
///   pub fn part1() {
///       let parsed = crate::check::parse_lines(INPUT);
///       assert_eq!(super::part1(&parsed), "world")
///   }
/// }
/// ```
macro_rules! check {
    ($($tail:tt)+) => {
        #[cfg(test)]
        pub mod tests {
            pub (crate) const EXAMPLE: &str = include_str!(concat!("../inputs/", day::day!(), "/example"));
            pub (crate) const INPUT: &str = include_str!(concat!("../inputs/", day::day!(), "/input"));
            gen_tests!($($tail)+);
        }
    };
}

#[doc(hidden)]
#[cfg(test)]
macro_rules! gen_tests {
    (gen $input:ident $prefix:ident $n:literal $expected:literal) => {
        paste::paste! {
            #[test]
            pub fn [<$prefix $n>]() {
                let parsed = crate::check::parse_lines($input);
                assert_eq!(super::[<part $n>](&parsed), $expected)
            }
        }
    };

    (ex $n:literal = $expected:literal) => {
        gen_tests!(gen EXAMPLE example $n $expected);
    };

    (ex $n:literal = $expected:literal, $($tail:tt)+) => {
        gen_tests!(ex $n = $expected);
        gen_tests!($($tail)+);
    };

    (part $n:literal = $expected:literal) => {
        gen_tests!(gen INPUT part $n $expected);
    };

    (part $n:literal = $expected:literal, $($tail:tt)+) => {
        gen_tests!(part $n = $expected);
        gen_tests!($($tail)+);
    };
}

#[doc(hidden)]
#[cfg(test)]
pub(crate) fn parse_lines<T: std::str::FromStr>(text: &str) -> Vec<T> {
    text.lines()
        .flat_map(str::parse)
        .collect::<Vec<T>>()
}
