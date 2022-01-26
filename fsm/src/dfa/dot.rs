use super::{Model, DFA};
use std::io::{Result, Write};

impl DFA {
	pub fn store_as_dot<W: Write>(&self, writer: &mut W) -> Result<()> {
		writer.write_all(
			b"digraph {\n\trankdir=LR\n\tnode [shape=circle]\n\tstart [shape=point]\n",
		)?;
		if self.model == Model::Mealy {
			if self.n_states > 0 {
				writer.write_all(b"\tstart -> q0\n")?;
			}
			for state in 0..self.n_states {
				for input in 0..self.n_inputs {
					writeln!(
						writer,
						"\tq{} -> q{} [label=\"x{}/y{}\"]",
						state,
						self.state_matrix[(state, input)],
						input,
						self.output_matrix[(state, input)]
					)?;
				}
			}
		} else {
			if self.n_states > 0 {
				writeln!(writer, "\tstart -> \"q0/y{}\"", self.output_matrix[(0, 0)])?;
			}
			for state in 0..self.n_states {
				for input in 0..self.n_inputs {
					let next_state = self.state_matrix[(state, input)];
					writeln!(
						writer,
						"\t\"q{}/y{}\" -> \"q{}/y{}\" [label=x{}]",
						state,
						self.output_matrix[(state, 0)],
						next_state,
						self.output_matrix[(next_state, 0)],
						input
					)?;
				}
			}
		}
		writer.write_all(b"}")?;
		Ok(())
	}
}
