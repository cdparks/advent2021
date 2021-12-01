#[macro_export]
macro_rules! bench {
    (part $n:literal) => {
        paste::paste! {
            #[bench]
            pub fn [<part $n>](b: &mut test::Bencher) {
                let parsed = test::black_box(crate::bench::parse_lines(INPUT));
                b.iter(|| super::[<part $n>](&parsed))
            }
        }
    };

    (part $n:literal, $($tail:tt)+) => {
        bench!(part $n);
        bench!($($tail)+);
    };


    ($day:literal, $($tail:tt)+) => {
        #[cfg(test)]
        pub mod bench {
            const INPUT: &str = include_str!(concat!("../inputs/", $day, "/input"));

            bench!($($tail)+);
        }
    };
}

#[doc(hidden)]
#[cfg(test)]
pub fn parse_lines<T: std::str::FromStr>(text: &str) -> Vec<T> {
    text.lines()
        .flat_map(|line| line.parse())
        .collect::<Vec<T>>()
}
