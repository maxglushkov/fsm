use super::{Automaton, StateType};
use crate::common::{parse_array0, Symbols};
use std::collections::BTreeSet;
use std::io::{BufRead, Error, ErrorKind, Result};
use std::str::FromStr;

#[derive(Clone, Copy)]
enum GrammarType {
	RightRegular,
	LeftRegular,
}

#[inline]
fn set_start_rule(state_type: &mut StateType, grammar_type: GrammarType) {
	*state_type |= match grammar_type {
		GrammarType::RightRegular => StateType::INITIAL,
		GrammarType::LeftRegular => StateType::FINAL,
	};
}

#[inline]
fn set_final_rule(state_type: &mut StateType, grammar_type: GrammarType) {
	*state_type |= match grammar_type {
		GrammarType::RightRegular => StateType::FINAL,
		GrammarType::LeftRegular => StateType::INITIAL,
	};
}

impl Automaton {
	pub fn load_from_regular_grammar<R: BufRead>(reader: &mut R) -> Result<Self> {
		let mut line = String::new();
		let grammar_type = parse_array0(reader, &mut line)?;
		let mut states = Symbols::new();
		let mut inputs = Symbols::new();
		let mut transitions = BTreeSet::new();
		let mut final_rules = BTreeSet::new();
		for _ in 0..parse_array0(reader, &mut line)? {
			reader.read_line(&mut line)?;
			let mut tokens = line.split_whitespace();
			let first_state = states.get_or_create_id(
				tokens
					.next()
					.ok_or_else(|| Error::new(ErrorKind::InvalidData, "no state name specified"))?
					.to_string(),
			);
			for token in tokens.collect::<Vec<&str>>().join("").split('|') {
				let mut token = token.chars();
				match match grammar_type {
					GrammarType::RightRegular => token.next(),
					GrammarType::LeftRegular => token.next_back(),
				} {
					Some(input) => {
						let input = inputs.get_or_create_id(input);
						let second_state = states.get_or_create_id(token.as_str().to_string());
						transitions.insert(match grammar_type {
							GrammarType::RightRegular => (first_state, input, second_state),
							GrammarType::LeftRegular => (second_state, input, first_state),
						});
					}
					None => {
						final_rules.insert(first_state);
					}
				}
			}
			line.clear();
		}
		let mut state_types = vec![StateType::default(); states.len()];
		if let Some(start_rule) = state_types.get_mut(0) {
			set_start_rule(start_rule, grammar_type);
		}
		if let Some(&final_rule) = states.get_id("") {
			set_final_rule(&mut state_types[final_rule], grammar_type);
		}
		for rule in final_rules {
			set_final_rule(&mut state_types[rule], grammar_type);
		}
		let mut automaton = Self {
			states: states.into_table(),
			inputs: inputs.into_table(),
			transitions,
		};
		for (state_type, state) in state_types.into_iter().zip(&mut automaton.states) {
			state.push(state_type.name());
		}
		Ok(automaton)
	}
}

impl FromStr for GrammarType {
	type Err = Error;

	fn from_str(s: &str) -> Result<Self> {
		match s {
			"R" => Ok(GrammarType::RightRegular),
			"L" => Ok(GrammarType::LeftRegular),
			_ => Err(Error::new(
				ErrorKind::InvalidData,
				"grammar should be (R)ight- of (L)eft-regular",
			)),
		}
	}
}
