#[macro_export]
macro_rules! solve {
    () => {};

    (, ex $n:literal = $expected:literal $($tail:tt)*) => {
        paste::paste! {
            #[test]
            pub fn [<example $n>]() {
                assert_eq!(super::[<part $n>](EXAMPLE), $expected)
            }
            solve!($($tail)*);
        }
    };

    (, part $n:literal = $expected:literal$($tail:tt)*) => {
        paste::paste! {
            #[test]
            pub fn [<part $n>] () {
                assert_eq!(super::[<part $n>](INPUT), $expected)
            }
            solve!($($tail)*);
        }
    };

    ($day:literal $($tail:tt)+) => {
        #[cfg(test)]
        pub mod tests {
            const EXAMPLE: &str = include_str!(concat!("../inputs/", $day, "/example"));
            const INPUT: &str = include_str!(concat!("../inputs/", $day, "/input"));
            solve!($($tail)*);
        }
    };
}
