use super::Automaton;
use crate::common::{parse_array0, Symbols};
use std::collections::BTreeSet;
use std::io::{BufRead, Error, ErrorKind, Result, Write};

impl Automaton {
	pub fn load_from_simple_text<R: BufRead>(reader: &mut R) -> Result<Self> {
		let mut line = String::new();
		let mut states = Symbols::with_capacity(parse_array0(reader, &mut line)?);
		let mut inputs = Symbols::with_capacity(parse_array0(reader, &mut line)?);
		let mut transitions = BTreeSet::new();
		while reader.read_line(&mut line)? > 0 {
			let tokens: Vec<&str> = line.split_whitespace().take(4).collect();
			if tokens.len() != 3 {
				return Err(Error::new(
					ErrorKind::InvalidData,
					"transition definition should contain 3 arguments",
				));
			}
			let on: Vec<char> = tokens[1].chars().take(2).collect();
			if on.len() != 1 {
				return Err(Error::new(
					ErrorKind::InvalidData,
					"input should be single character",
				));
			}
			let from = states.get_or_create_id(tokens[0].to_string());
			let on = inputs.get_or_create_id(on[0]);
			let into = states.get_or_create_id(tokens[2].to_string());
			transitions.insert((from, on, into));
			line.clear();
		}
		Ok(Self {
			states: states.into_table(),
			inputs: inputs.into_table(),
			transitions,
		})
	}

	pub fn store_as_simple_text<W: Write>(&self, writer: &mut W) -> Result<()> {
		writeln!(writer, "{}", self.states.len())?;
		writeln!(writer, "{}", self.inputs.len())?;
		for (from, on, into) in &self.transitions {
			writeln!(
				writer,
				"{} {} {}",
				self.states[*from], self.inputs[*on], self.states[*into]
			)?;
		}
		Ok(())
	}
}
