# aoc_boilerplate

Macro and utils to reduce boilerplate for advent of code.

## Macro usage

### `Cargo.toml`

> `Cargo.toml`
>
> ```toml
> [dependencies]
> aoc_macro = { git = "https://github.com/rosengrenen/aoc_boilerplate" }
> aoc_util = { git = "https://github.com/rosengrenen/aoc_boilerplate" }
> ```

### `main.rs`

The macro takes the year as input

> `src/main.rs`
>
> ```rust
> #![feature(test)]
> extern crate test;
>
> aoc_macro::include_days!(2021);
> ```

### Solution structure

For each day, add `src/day{:02}` (e.g. `src/day01`) with a `mod.rs` in it containing the following (example for empty day 1):

> `src/day01/mod.rs`
>
> ```rust
> use aoc_util::{Solver, SolverOutput};
>
> #[derive(Default)]
> pub struct Day1;
>
> impl Solver for Day1 {
> 	fn part_one(&self, input: &str) -> SolverOutput {
> 		SolverOutput::Num(0)
> 	}
>
> 	fn part_two(&self, input: &str) -> SolverOutput {
> 		SolverOutput::Num(0)
> 	}
> }
> ```

### Puzzle input

The macro automatically fetches the puzzle input. To do this it needs the session cookie value. When logged in to adventofcode.com it can be found in the developer tools under cookies, named `session`. Add an environment variable name `AOC_SESSION` to your environment or to a `.env` file located in the root of the project.

> `.env`
>
> ```
> AOC_SESSION=<token>
> ```

### Automatic tests

To add automatic tests, create a file named `part(1|2).<test_name>.txt` containing input and output separated by `<<<>>>`. Below is an example of the first test of 2019 day 1 in this format.

> `part1.test1.txt`
>
> ```
> 12
> <<<>>>
> 2
> ```
