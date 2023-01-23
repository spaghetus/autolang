use std::ops::Deref;

use rand::{seq::SliceRandom, thread_rng};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum Token {
	Literal(String),
	From(usize),
	To(usize),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Node {
	Leaf(Token),
	Branches(Vec<Node>),
}

impl Node {
	fn frequency(&self, map: &Mapping) -> usize {
		match self {
			Node::Leaf(token) => map.freq(token),
			Node::Branches(children) => children.iter().map(|child| child.frequency(map)).sum(),
		}
	}
	fn translate(&self, from: &Token, map: &Mapping) -> Option<Vec<Token>> {
		match self {
			Node::Leaf(this) => {
				if this == from {
					Some(vec![])
				} else {
					None
				}
			}
			Node::Branches(children) => children
				.iter()
				.enumerate()
				.map(|(n, node)| (n, node.translate(from, map)))
				.flat_map(|(n, path)| path.map(|path| (n, path)))
				.map(|(n, mut path)| {
					let mut v = vec![Token::To(n)];
					v.append(&mut path);
					v
				})
				.next(),
		}
	}

	fn translate_reverse(&self, from: &[Token], map: &Mapping) -> Option<Token> {
		match self {
			Node::Leaf(leaf) => Some(leaf.clone()),
			Node::Branches(children) => {
				if from.len() == 0 {
					None
				} else if let Token::To(index) = from[0] {
					children[index].translate_reverse(&from[1..], map)
				} else {
					None
				}
			}
		}
	}
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Mapping {
	pub tree: Vec<Node>,
	pub from: Vec<FromSymbol>,
	pub to: Vec<ToSymbol>,
}

impl Mapping {
	pub fn build(&mut self) {
		while self.tree.len() > 1 {
			// Sort the tree by frequency
			{
				let map = self.clone();
				self.tree.sort_by_cached_key(|node| node.frequency(&map));
			}
			// Take the least significant to.len() nodes
			let mut new_children = (0..self.to.len())
				.flat_map(|_| {
					if self.tree.len() > 0 {
						Some(self.tree.remove(0))
					} else {
						None
					}
				})
				.collect::<Vec<_>>();
			new_children.shuffle(&mut thread_rng());
			// Add a new node with those least significant nodes
			self.tree.push(Node::Branches(new_children))
		}
	}

	pub fn translate(&self, symbols: &[Token]) -> Vec<Token> {
		symbols
			.iter()
			.flat_map(|token| match token {
				Token::Literal(s) => vec![Token::Literal(s.to_string())],
				Token::To(_) => unreachable!(),
				from => self.tree[0].translate(from, self).unwrap_or_default(),
			})
			.collect()
	}

	pub fn translate_reverse(&self, symbols: &[Token]) -> Vec<Token> {
		let mut buffer = vec![];
		let mut results = vec![];

		for symbol in symbols {
			match symbol {
				Token::From(_) => unreachable!(),
				Token::Literal(s) => results.push(Token::Literal(s.to_string())),
				to => {
					buffer.push(symbol.clone());
					if let Some(token) = self.tree[0].translate_reverse(&buffer, self) {
						results.push(token);
						buffer.clear();
					}
				}
			}
		}

		results
	}

	pub fn freq(&self, token: &Token) -> usize {
		match token {
			Token::From(index) => self.from[*index].frequency,
			_ => 0,
		}
	}

	pub fn to_text(&self, token: &Token) -> String {
		match token {
			Token::Literal(string) => string.clone(),
			Token::From(index) => self.from[*index].text.clone(),
			Token::To(index) => self.to[*index].text.clone(),
		}
	}

	pub fn from_text(&self, text: String) -> Vec<Token> {
		text.split_ascii_whitespace()
			.map(|word| {
				for (index, token) in self.from.iter().enumerate() {
					if token.text == word {
						return Token::From(index);
					}
				}
				Token::Literal(word.to_string())
			})
			.collect()
	}

	pub fn from_text_reverse(&self, text: String) -> Vec<Token> {
		text.split_ascii_whitespace()
			.map(|word| {
				for (index, token) in self.to.iter().enumerate() {
					if token.text == word {
						return Token::To(index);
					}
				}
				Token::Literal(word.to_string())
			})
			.collect()
	}

	pub fn into_text(&self, tokens: &[Token]) -> String {
		tokens
			.iter()
			.map(|token| self.to_text(token))
			.collect::<Vec<_>>()
			.join(" ")
	}
}

#[derive(Serialize, Deserialize, Clone)]
pub struct FromSymbol {
	pub text: String,
	pub frequency: usize,
}

impl Deref for FromSymbol {
	type Target = str;

	fn deref(&self) -> &Self::Target {
		&self.text
	}
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ToSymbol {
	pub text: String,
}

impl Deref for ToSymbol {
	type Target = str;

	fn deref(&self) -> &Self::Target {
		&self.text
	}
}
