use std::{
	fs::File,
	io::{stdin, BufReader},
	path::PathBuf,
};

use autolang::Mapping;
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
	mapping: PathBuf,
	#[arg(short, long)]
	reverse: bool,
}

fn main() {
	let args = Args::parse();

	let map: Mapping =
		serde_json::from_reader(File::open(args.mapping).expect("Failed to open mapping file"))
			.expect("Failed to parse mapping");

	for line in stdin().lines().flatten() {
		let tokenized = if args.reverse {
			map.from_text_reverse(line)
		} else {
			map.from_text(line)
		};
		let translated = if args.reverse {
			map.translate_reverse(&tokenized)
		} else {
			map.translate(&tokenized)
		};
		let text = map.into_text(&translated);
		println!("{}", text);
	}
}
