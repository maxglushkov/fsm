use super::{Automaton, StateType};
use std::io::{Result, Write};

impl Automaton {
	pub fn store_as_dot<W: Write>(&self, writer: &mut W) -> Result<()> {
		writer.write_all(
			b"digraph {\n\trankdir=LR\n\tnode [shape=circle]\n\tstart [shape=point]\n",
		)?;
		for state in &self.states {
			let state_type = StateType::from_last_char(state);
			if state_type & StateType::INITIAL {
				writeln!(writer, "\tstart -> \"{}\"", state)?;
			}
			if state_type & StateType::FINAL {
				writeln!(writer, "\t\"{}\" [shape=doublecircle]", state)?;
			}
		}
		for (from, on, into) in &self.transitions {
			writeln!(
				writer,
				"\t\"{}\" -> \"{}\" [label=\"{}\"]",
				self.states[*from], self.states[*into], self.inputs[*on]
			)?;
		}
		writer.write_all(b"}")?;
		Ok(())
	}
}
