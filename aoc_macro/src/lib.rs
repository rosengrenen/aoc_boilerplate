use std::{collections::HashSet, fs, io};

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse::Parse, parse_macro_input, Ident, LitInt};

struct Year {
	year: i64,
}

impl Parse for Year {
	fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
		Ok(Self {
			year: input.parse::<LitInt>()?.base10_parse()?,
		})
	}
}

#[proc_macro]
pub fn include_days(stream: TokenStream) -> TokenStream {
	let Year { year } = parse_macro_input!(stream as Year);
	let days = get_days().expect("Could not get days");

	let mut modules = quote! {};
	let mut solvers = quote! {};
	let mut tests = quote! {};
	let mut benches = quote! {};

	for day in days {
		let module_name = format!("day{:02}", day);
		let module_name_ident = Ident::new(&module_name, Span::call_site());
		let struct_name_ident = Ident::new(&format!("Day{}", day), Span::call_site());

		modules = quote! {
			#modules
			mod #module_name_ident;
		};

		solvers = quote! {
			#solvers
			#day => Some(Box::new(#module_name_ident::#struct_name_ident::default())),
		};

		let tests_meta =
			get_tests(day).unwrap_or_else(|_| panic!("Could not get tests for day {}", day));
		if !tests_meta.is_empty() {
			let mut day_tests = quote! {};
			for (part, name) in tests_meta {
				let (part_one, test_name) = match part {
					1 => (true, format!("part_one_{}", name)),
					2 => (false, format!("part_two_{}", name)),
					_ => continue,
				};
				let test_name_ident = Ident::new(&test_name, Span::call_site());

				day_tests = quote! {
					#day_tests

					#[test]
					fn #test_name_ident() {
						let solver: Box<dyn Solver> = Box::new(#struct_name_ident::default());
						test_solver(&solver, #day, #part_one, #name);
					}
				};
			}

			tests = quote! {
				#tests

				mod #module_name_ident {
					use aoc_util::{test_solver, Solver};
					use crate::#module_name_ident::#struct_name_ident;

					#day_tests
				}
			}
		}

		benches = quote! {
			#benches

			mod #module_name_ident {
				use aoc_util::{bench_solver, Solver};
				use test::Bencher;

				use crate::#module_name_ident::#struct_name_ident;

				#[bench]
				fn part_one(bencher: &mut Bencher) {
					let solver: Box<dyn Solver> = Box::new(#struct_name_ident::default());
					bench_solver(bencher, &solver, #year, #day, true);
				}

				#[bench]
				fn part_two(bencher: &mut Bencher) {
					let solver: Box<dyn Solver> = Box::new(#struct_name_ident::default());
					bench_solver(bencher, &solver, #year, #day, false);
				}
			}
		}
	}

	let out = quote! {
		#modules

		use aoc_util::{get_input, Opts, Solver, SolverOutput};

		fn get_solver(day: i64) -> Option<Box<dyn Solver>> {
			match day {
				#solvers
				_ => None,
			}
		}

		fn format_answer(day: i64, part: i64, answer: &SolverOutput) {
			println!("Day #{:02} part {}:\n{}", day, part, answer);
		}

		fn run_day(day: i64, part: Option<i64>) {
			let solver = match get_solver(day) {
				Some(solver) => solver,
				None => return,
			};

			let input = get_input(#year, day).unwrap_or_else(|_| panic!("Could not get input for day {}", day));

			match part {
				None => {
					let answer = solver.part_one(&input);
					format_answer(day, 1, &answer);
					let answer = solver.part_two(&input);
					format_answer(day, 2, &answer);
				}
				Some(0) => {
					let answer = solver.part_one(&input);
					format_answer(day, 1, &answer);
				}
				Some(1) => {
					let answer = solver.part_two(&input);
					format_answer(day, 2, &answer);
				}
				_ => return,
			}
		}

		fn main() {
			let opts = Opts::parse();
			if let Some(day) = opts.day {
				run_day(day, opts.part);
			} else {
				for day in 0..=25 {
					run_day(day, opts.part);
				}
			}
		}

		#[cfg(test)]
		mod tests {
			#tests
		}

		#[cfg(test)]
		mod benches {
			#benches
		}
	};

	out.into()
}

fn get_days() -> io::Result<Vec<i64>> {
	let mut days: Vec<_> = fs::read_dir("src")?
		.filter_map(|entry| {
			let entry = match entry {
				Ok(entry) => entry,
				Err(_) => return None,
			};

			if !entry.path().is_dir() {
				return None;
			}

			let name = entry.file_name();
			let name = name.to_string_lossy().to_string();

			if let Some(day) = name.strip_prefix("day") {
				match day.parse() {
					Ok(day) => Some(day),
					Err(_) => None,
				}
			} else {
				None
			}
		})
		.collect();
	days.sort();
	Ok(days)
}

fn get_tests(day: i64) -> io::Result<Vec<(i64, String)>> {
	let mut tests = HashSet::new();
	for entry in fs::read_dir(format!("src/day{:02}", day))? {
		let entry = entry?;

		if !entry.path().is_file() {
			continue;
		}

		let name = entry.file_name();
		let name = name.to_string_lossy().to_string();

		if name.ends_with(".txt") {
			let parts: Vec<_> = name.split('.').collect();
			if parts.len() != 3 {
				continue;
			}

			let part_num = parts[0];
			let test_name = parts[1];

			let part_num = match part_num.strip_prefix("part") {
				Some(part_num) => match part_num {
					"1" => 1,
					"2" => 2,
					_ => continue,
				},
				None => continue,
			};

			tests.insert((part_num, test_name.to_string()));
		}
	}

	Ok(tests.into_iter().collect())
}
