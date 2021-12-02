#[macro_export]
macro_rules! bench {
    ($($tail:tt)+) => {
        #[cfg(test)]
        pub mod bench {
            gen_bench!($($tail)+);
        }
    };
}

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
