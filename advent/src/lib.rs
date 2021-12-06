//! Solutions to Advent of Code 2021

#![allow(dead_code)]
#![deny(missing_debug_implementations)]
#![deny(missing_docs)]
#![feature(never_type)]
#![feature(test)]
extern crate test;

#[macro_use]
/// Macros for generating tests
pub mod check;

#[macro_use]
/// Macros for generating benchmarks
pub mod bench;

/// Solutions for day 1
pub mod day01;
/// Solutions for day 2
pub mod day02;
/// Solutions for day 3
pub mod day03;
/// Solutions for day 4
pub mod day04;
/// Solutions for day 5
pub mod day05;
/// Solutions for day 6
pub mod day06;
