#[doc(hidden)]
#[cfg(test)]
pub fn parse_lines<T: std::str::FromStr>(text: &str) -> Vec<T> {
     text.lines().flat_map(|line| line.parse()).collect::<Vec<T>>()
}

#[macro_export]
macro_rules! check {
    () => {};

    // `ex <n> = <expected>` generates a test using parsed EXAMPLE
    (, ex $n:literal = $expected:literal $($tail:tt)*) => {
        paste::paste! {
            #[test]
            pub fn [<example $n>]() {
                let parsed = crate::macros::parse_lines(EXAMPLE);
                assert_eq!(super::[<part $n>](&parsed), $expected)
            }

            check!($($tail)*);
        }
    };

    // `part <n> = <expected>` generates a test and benchmark using INPUT
    (, part $n:literal = $expected:literal$($tail:tt)*) => {
        paste::paste! {
            #[test]
            pub fn [<part $n>] () {
                let parsed = crate::macros::parse_lines(INPUT);
                assert_eq!(super::[<part $n>](&parsed), $expected)
            }

            #[bench]
            pub fn [<bench_ part $n>](b: &mut test::Bencher) {
                let parsed = crate::macros::parse_lines(INPUT);
                b.iter(|| {
                    super::[<part $n>](&parsed);
                })
            }

            check!($($tail)*);
        }
    };

    // `"XX" ...` generates a test module using inputs in "../inputs/XX/"
    ($day:literal $($tail:tt)+) => {
        #[cfg(test)]
        pub mod tests {
            const EXAMPLE: &str = include_str!(concat!("../inputs/", $day, "/example"));
            const INPUT: &str = include_str!(concat!("../inputs/", $day, "/input"));

            check!($($tail)*);
        }
    };
}
