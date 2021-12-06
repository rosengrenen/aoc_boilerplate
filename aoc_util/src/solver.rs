use std::{fmt::Display, fs::read_to_string};

use test::{black_box, Bencher};

use crate::get_input;

#[derive(Clone, Debug, PartialEq)]
pub enum SolverOutput {
	Num(i64),
	String(String),
}

impl Display for SolverOutput {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Num(num) => write!(f, "{}", num),
			Self::String(string) => write!(f, "{}", string),
		}
	}
}

pub trait Solver {
	fn part_one(&self, input: &str) -> SolverOutput;
	fn part_two(&self, input: &str) -> SolverOutput;
}

pub fn test_solver(solver: &Box<dyn Solver>, day: i64, part_one: bool, name: &str) {
	let part = if part_one { 1 } else { 2 };
	let input = read_to_string(&format!("src/day{:02}/part{}.{}.txt", day, part, name))
		.expect("Could not get input for test");
	let (input, output) = input
		.split_once("<<<>>>")
		.expect("Could not get input/output for test, use <<<>>> to separate them");

	if part_one {
		assert_eq!(solver.part_one(input.trim()).to_string(), output.trim());
	} else {
		assert_eq!(solver.part_two(input.trim()).to_string(), output.trim());
	}
}

pub fn bench_solver(
	bencher: &mut Bencher,
	solver: &Box<dyn Solver>,
	year: i64,
	day: i64,
	part_one: bool,
) {
	let input = get_input(year, day).expect("Could not get input for bench");
	if part_one {
		bencher.iter(|| solver.part_one(black_box(&input)));
	} else {
		bencher.iter(|| solver.part_two(black_box(&input)));
	}
}
