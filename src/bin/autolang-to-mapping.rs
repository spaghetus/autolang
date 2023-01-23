use std::{collections::HashMap, path::PathBuf};

use autolang::{FromSymbol, Mapping, Node, ToSymbol};
use clap::Parser;
use csv::Reader;

#[derive(Parser, Debug)]
struct Args {
	from_symbols: PathBuf,
	to_symbols: PathBuf,
	output: PathBuf,
}

fn main() {
	let args = Args::parse();

	let from_symbols = Reader::from_path(args.from_symbols)
		.expect("Failed to open from symbols")
		.deserialize::<FromSymbol>()
		.flatten()
		.collect::<Vec<_>>();

	let to_symbols = Reader::from_path(args.to_symbols)
		.expect("Failed to open from symbols")
		.deserialize::<ToSymbol>()
		.flatten()
		.collect::<Vec<_>>();

	let mut mapping = Mapping {
		tree: from_symbols
			.iter()
			.enumerate()
			.map(|(n, _)| Node::Leaf(autolang::Token::From(n)))
			.collect(),
		from: from_symbols,
		to: to_symbols,
	};
	mapping.build();

	let mapping = serde_json::to_string_pretty(&mapping).expect("Failed to serialize");
	std::fs::write(args.output, mapping).expect("Failed to write")
}
