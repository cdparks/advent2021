#[macro_export]
macro_rules! check {
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
        check!(gen EXAMPLE example $n $expected);
    };

    (ex $n:literal = $expected:literal, $($tail:tt)+) => {
        check!(ex $n = $expected);
        check!($($tail)+);
    };

    (part $n:literal = $expected:literal) => {
        check!(gen INPUT part $n $expected);
    };

    (part $n:literal = $expected:literal, $($tail:tt)+) => {
        check!(part $n = $expected);
        check!($($tail)+);
    };

    ($day:literal, $($tail:tt)+) => {
        #[cfg(test)]
        pub mod tests {
            pub (crate) const EXAMPLE: &str = include_str!(concat!("../inputs/", $day, "/example"));
            pub (crate) const INPUT: &str = include_str!(concat!("../inputs/", $day, "/input"));

            check!($($tail)+);
        }
    };
}

#[doc(hidden)]
#[cfg(test)]
pub (crate) fn parse_lines<T: std::str::FromStr>(text: &str) -> Vec<T> {
    text.lines()
        .flat_map(|line| line.parse())
        .collect::<Vec<T>>()
}
