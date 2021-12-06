#![feature(test)]
extern crate test;

mod input;
mod opts;
mod solver;

pub use input::get_input;
pub use opts::Opts;
pub use solver::{bench_solver, test_solver, Solver, SolverOutput};
