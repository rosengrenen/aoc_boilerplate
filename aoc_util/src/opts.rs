use clap::Parser;

#[derive(Parser)]
pub struct Opts {
	#[clap(short, long)]
	pub day: Option<i64>,

	#[clap(short, long)]
	pub part: Option<i64>,
}

impl Opts {
	pub fn parse() -> Self {
		Parser::parse()
	}
}
