use std::{
	env,
	fs::{read_to_string, write},
};

use dotenv::dotenv;
use reqwest::{blocking::Client, header::COOKIE};

fn input_from_file(day: i64) -> anyhow::Result<String> {
	Ok(read_to_string(format!("src/day{:02}/input.txt", day))?)
}

fn input_from_remote(year: i64, day: i64) -> anyhow::Result<String> {
	dotenv().ok();
	let session = env::var("AOC_SESSION")
		.expect("AOC_SESSION variable missing, add to environment or to .env");
	let endpoint = format!("https://adventofcode.com/{}/day/{}/input", year, day);
	let client = Client::new();
	let response = client
		.get(&endpoint)
		.header(COOKIE, format!("session={}", session))
		.send()?;
	if response.status() != 200 {
		anyhow::bail!("Advent of Code request unsuccessful");
	}

	let input = response.text()?.trim().to_string();

	Ok(input)
}

pub fn get_input(year: i64, day: i64) -> anyhow::Result<String> {
	if let Ok(input) = input_from_file(day) {
		Ok(input)
	} else {
		let input = input_from_remote(year, day)?;

		let _ = write(format!("src/day{:02}/input.txt", day), &input);

		Ok(input)
	}
}
