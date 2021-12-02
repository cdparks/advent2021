extern crate day;

#[macro_export]
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
