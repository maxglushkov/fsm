use super::{Automaton, StateType};
use crate::common::Symbols;
use std::collections::BTreeSet;
use std::fmt;

struct Builder {
	n_states: usize,
	inputs: Symbols<char>,
	transitions: BTreeSet<(usize, usize, usize)>,
	expression: Vec<ExprPart>,
}

enum ExprPart {
	StartGroup(usize),
	Alternative,
	MachinePart(MachinePart),
}

struct MachinePart {
	is_final: bool,
	final_ids: BTreeSet<usize>,
	transitions: BTreeSet<(usize, usize)>,
}

pub struct ParseRegExpError {
	position: usize,
	kind: RegExpErrorKind,
}

enum RegExpErrorKind {
	MissingPostfixUnaryOpArg(char),
	UnmatchedParenthesis,
	InvalidEscapeSequence,
}

impl Automaton {
	pub fn from_regexp(regexp: &str) -> Result<Self, ParseRegExpError> {
		let mut builder = Builder::new();
		let mut position = 0;
		let mut escaped = false;
		for c in regexp.chars() {
			position += 1;
			if escaped {
				escaped = false;
				builder.push_char(c);
			} else {
				match c {
					'(' => builder.start_group(position),
					')' => builder.end_group().map_err(|()| ParseRegExpError {
						position,
						kind: RegExpErrorKind::UnmatchedParenthesis,
					})?,
					'*' => builder.repeat_top(false).map_err(|()| ParseRegExpError {
						position,
						kind: RegExpErrorKind::MissingPostfixUnaryOpArg('*'),
					})?,
					'+' => builder.repeat_top(true).map_err(|()| ParseRegExpError {
						position,
						kind: RegExpErrorKind::MissingPostfixUnaryOpArg('+'),
					})?,
					'\\' => escaped = true,
					'|' => builder.push_alternative(),
					_ => builder.push_char(c),
				}
			}
		}
		if escaped {
			return Err(ParseRegExpError {
				position,
				kind: RegExpErrorKind::InvalidEscapeSequence,
			});
		}
		builder.build()
	}
}

impl Builder {
	fn new() -> Self {
		Self {
			n_states: 0,
			inputs: Symbols::new(),
			transitions: BTreeSet::new(),
			expression: Vec::new(),
		}
	}

	fn build(mut self) -> Result<Automaton, ParseRegExpError> {
		if let Some(ExprPart::StartGroup(position)) = self
			.expression
			.iter()
			.find(|&part| ExprPart::is_start_group(part))
		{
			return Err(ParseRegExpError {
				position: *position,
				kind: RegExpErrorKind::UnmatchedParenthesis,
			});
		}
		let start = MachinePart::reduce(&mut self.expression, &mut self.transitions);
		Ok(Automaton {
			states: self
				.finalize(start)
				.iter()
				.enumerate()
				.map(|(state_id, state_type)| format!("S{}{}", state_id, state_type.name()))
				.collect(),
			inputs: self.inputs.into_table(),
			transitions: self.transitions,
		})
	}

	fn start_group(&mut self, position: usize) {
		self.expression.push(ExprPart::StartGroup(position));
	}

	fn end_group(&mut self) -> Result<(), ()> {
		let start_index = match self.expression.iter().rposition(ExprPart::is_start_group) {
			Some(index) => index,
			None => return Err(()),
		};
		let machine_part = MachinePart::reduce(
			self.expression.split_at_mut(start_index + 1).1,
			&mut self.transitions,
		);
		self.expression.truncate(start_index);
		self.expression.push(ExprPart::MachinePart(machine_part));
		Ok(())
	}

	fn push_alternative(&mut self) {
		self.expression.push(ExprPart::Alternative);
	}

	fn push_char(&mut self, c: char) {
		self.expression.push(ExprPart::MachinePart(MachinePart {
			is_final: false,
			final_ids: BTreeSet::from([self.n_states]),
			transitions: BTreeSet::from([(self.inputs.get_or_create_id(c), self.n_states)]),
		}));
		self.n_states += 1;
	}

	fn repeat_top(&mut self, at_least_once: bool) -> Result<(), ()> {
		let machine_part = match self.expression.last_mut() {
			Some(ExprPart::MachinePart(part)) => part,
			_ => return Err(()),
		};
		machine_part.connect(machine_part, &mut self.transitions);
		machine_part.is_final |= !at_least_once;
		Ok(())
	}

	fn finalize(&mut self, start: MachinePart) -> Vec<StateType> {
		let start_id = self.n_states;
		self.n_states += 1;
		self.transitions.extend(
			start
				.transitions
				.iter()
				.map(|&(on, into)| (start_id, on, into)),
		);
		let mut state_types = vec![StateType::default(); self.n_states];
		for final_id in start.final_ids {
			state_types[final_id] |= StateType::FINAL;
		}
		if start.is_final {
			state_types[start_id] |= StateType::FINAL;
		}
		state_types[start_id] |= StateType::INITIAL;
		state_types
	}
}

impl ExprPart {
	fn is_start_group(&self) -> bool {
		matches!(self, Self::StartGroup(_))
	}

	fn is_alternative(&self) -> bool {
		matches!(self, Self::Alternative)
	}
}

impl MachinePart {
	fn reduce(
		variants: &mut [ExprPart],
		transitions: &mut BTreeSet<(usize, usize, usize)>,
	) -> Self {
		Self::merge(variants.split_mut(ExprPart::is_alternative).map(|variant| {
			let mut parts = variant.iter_mut();
			let mut variant = Self {
				is_final: true,
				final_ids: BTreeSet::new(),
				transitions: BTreeSet::new(),
			};
			while let Some(ExprPart::MachinePart(part)) = parts.next() {
				variant.concat(part, transitions);
			}
			variant
		}))
	}

	fn merge<I: IntoIterator<Item = Self>>(parts: I) -> Self {
		let mut merged = Self {
			is_final: false,
			final_ids: BTreeSet::new(),
			transitions: BTreeSet::new(),
		};
		for mut part in parts {
			merged.is_final |= part.is_final;
			merged.final_ids.append(&mut part.final_ids);
			merged.transitions.append(&mut part.transitions);
		}
		merged
	}

	fn concat(&mut self, rhs: &Self, transitions: &mut BTreeSet<(usize, usize, usize)>) {
		if self.is_final {
			self.transitions.extend(&rhs.transitions);
		}
		self.connect(rhs, transitions);
		if !rhs.is_final {
			self.is_final = false;
			self.final_ids.clear();
		}
		self.final_ids.extend(&rhs.final_ids);
	}

	fn connect(&self, rhs: &Self, transitions: &mut BTreeSet<(usize, usize, usize)>) {
		for &final_id in &self.final_ids {
			transitions.extend(
				rhs.transitions
					.iter()
					.map(|&(on, into)| (final_id, on, into)),
			);
		}
	}
}

impl fmt::Display for ParseRegExpError {
	fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		write!(formatter, "{}: {}", self.position, self.kind)
	}
}

impl fmt::Display for RegExpErrorKind {
	fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::MissingPostfixUnaryOpArg(op) => {
				write!(formatter, "postfix operator '{}' takes 1 argument", op)
			}
			Self::UnmatchedParenthesis => formatter.write_str("unmatched parenthesis"),
			Self::InvalidEscapeSequence => formatter.write_str("invalid escape sequence"),
		}
	}
}
