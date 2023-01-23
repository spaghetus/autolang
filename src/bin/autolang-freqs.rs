//! Automatically converts from a text file containing example writing in the source language,
//! to a CSV mapping from words to frequencies.

use std::{
	collections::{HashMap, HashSet},
	io::{stdin, stdout, BufRead, BufReader, Write},
	ops::RangeBounds,
	path::PathBuf,
};

use autolang::FromSymbol;
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
	#[arg(short, long)]
	from: Option<PathBuf>,
	#[arg(short, long)]
	to: Option<PathBuf>,

	/// If a word contains a non-ASCII character, it's probably an OCR error in the dataset.
	#[arg(long)]
	allow_non_ascii: bool,
	/// If a word contains a capital letter, it's probably a name.
	#[arg(long)]
	allow_capitals: bool,
	/// If a word contains punctuation, we should probably remove it.
	#[arg(long)]
	allow_punctuation: bool,
	/// Only allow words from a certain dictionary
	#[arg(long)]
	dictionary: Option<PathBuf>,
}

fn main() {
	let args = Args::parse();

	let input: Box<dyn BufRead> = if let Some(path) = args.from {
		let file = std::fs::File::open(path).expect("Failed to open input file");
		let buf = BufReader::new(file);
		Box::new(buf)
	} else {
		Box::new(stdin().lock())
	};

	let output: Box<dyn Write> = if let Some(path) = args.to {
		let file = std::fs::File::open(path).expect("Failed to open output file");
		Box::new(file)
	} else {
		Box::new(stdout().lock())
	};

	let dictionary = args
		.dictionary
		.and_then(|path| std::fs::read_to_string(path).ok())
		.map(|string| {
			string
				.lines()
				.map(|s| s.to_string())
				.collect::<HashSet<_>>()
		})
		.unwrap_or_default();

	let mut words = input
		.lines()
		.flatten()
		.flat_map(|l| {
			l.split_ascii_whitespace()
				.map(|v| v.to_string())
				.collect::<Vec<_>>()
		})
		.filter(|word| word.chars().all(|char| char.is_ascii()) || args.allow_non_ascii)
		.filter(|word| word.chars().all(|char| !char.is_ascii_uppercase()) || args.allow_capitals)
		.filter(|word| {
			word.chars()
				.all(|c| c.is_alphabetic() || args.allow_punctuation)
		})
		.filter(|word| dictionary.is_empty() || dictionary.contains(word))
		.collect::<Vec<_>>();
	words.sort();

	let mut max = 0;

	let words: HashMap<String, usize> = {
		let mut frequencies = HashMap::new();
		for word in words {
			let entry = frequencies.entry(word).or_insert(0);
			*entry += 1;
			max = *entry.max(&mut max);
		}
		frequencies
	};

	let mut output = csv::Writer::from_writer(output);

	for (word, frequency) in words {
		output
			.serialize(FromSymbol {
				text: word,
				frequency,
			})
			.expect("Failed to serialize");
	}

	output.flush().expect("Failed to flush output");
}
