#[macro_export]
/// Generate benchmarks for the solution.
///
/// # Examples
///
/// In a file named `path/to/day00.rs`, the following invocation:
///
/// ```rust,no_run
/// advent::bench!(part 1, part 2);
/// ```
///
/// expands to
///
/// ```rust,no_run
/// #[cfg(test)]
/// pub mod bench {
///     #[bench]
///     pub fn part1(b: &mut test::Bencher) {
///         let parsed = crate::check::parse_lines(super::tests::INPUT);
///         let arg = test::black_box(parsed);
///         b.iter(|| super::part1(&arg))
///     }
///
///     #[bench]
///     pub fn part2(b: &mut test::Bencher) {
///         let parsed = crate::check::parse_lines(super::tests::INPUT);
///         let arg = test::black_box(parsed);
///         b.iter(|| super::part2(&arg))
///     }
/// }
/// ```
macro_rules! bench {
    ($($tail:tt)+) => {
        #[cfg(test)]
        pub mod bench {
            gen_bench!($($tail)+);
        }
    };
}

#[doc(hidden)]
#[cfg(test)]
macro_rules! gen_bench {
    (part $n:literal) => {
        paste::paste! {
            #[bench]
            pub fn [<part $n>](b: &mut test::Bencher) {
                let parsed = crate::check::parse_lines(super::tests::INPUT);
                let arg = test::black_box(parsed);
                b.iter(|| super::[<part $n>](&arg))
            }
        }
    };

    (part $n:literal, $($tail:tt)+) => {
        gen_bench!(part $n);
        gen_bench!($($tail)+);
    };
}
