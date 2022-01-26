use super::{matrix, Model, DFA};
use crate::common::parse_array0;
use ndarray::{ArrayView1, ArrayViewMut1};
use std::io::{BufRead, Error, ErrorKind, Result, Write};
use std::mem::MaybeUninit;

fn parse_array1<R: BufRead>(
	reader: &mut R,
	line: &mut String,
	array: &mut ArrayViewMut1<MaybeUninit<usize>>,
	value_limit: usize,
) -> Result<()> {
	reader.read_line(line)?;
	let mut iter = line.split_whitespace();
	for (index, cell) in array.iter_mut().enumerate() {
		let value = match iter.next() {
			Some(value) => value,
			None => {
				return Err(Error::new(
					ErrorKind::InvalidData,
					format!("expected {} columns, found {}", array.len(), index),
				))
			}
		}
		.parse()
		.map_err(|err| Error::new(ErrorKind::InvalidData, err))?;
		if value >= value_limit {
			return Err(Error::new(
				ErrorKind::InvalidData,
				format!(
					"expected any non-negative integer below {}, found {}",
					value_limit, value
				),
			));
		}
		*cell = MaybeUninit::new(value);
	}
	line.clear();
	Ok(())
}

fn format_array1<W: Write>(writer: &mut W, array: &ArrayView1<usize>) -> Result<()> {
	for cell in array {
		write!(writer, "{} ", cell)?;
	}
	writeln!(writer)
}

impl DFA {
	pub fn load_from_simple_text<R: BufRead>(reader: &mut R) -> Result<Self> {
		let mut line = String::new();
		let model = parse_array0(reader, &mut line)?;
		let n_states = parse_array0(reader, &mut line)?;
		let n_inputs = parse_array0(reader, &mut line)?;
		let n_outputs = parse_array0(reader, &mut line)?;
		let mut output_matrix = matrix::uninit_output(model, n_states, n_inputs);
		let mut state_matrix = matrix::uninit_state(n_states, n_inputs);
		if model == Model::Mealy {
			for (mut states, mut outputs) in state_matrix
				.columns_mut()
				.into_iter()
				.zip(output_matrix.columns_mut())
			{
				parse_array1(reader, &mut line, &mut states, n_states)?;
				parse_array1(reader, &mut line, &mut outputs, n_outputs)?;
			}
		} else {
			parse_array1(
				reader,
				&mut line,
				&mut output_matrix.column_mut(0),
				n_outputs,
			)?;
			for mut states in state_matrix.columns_mut() {
				parse_array1(reader, &mut line, &mut states, n_states)?;
			}
		}
		unsafe {
			Ok(Self {
				model,
				n_states,
				n_inputs,
				n_outputs,
				output_matrix: output_matrix.assume_init(),
				state_matrix: state_matrix.assume_init(),
			})
		}
	}

	pub fn store_as_simple_text<W: Write>(&self, writer: &mut W) -> Result<()> {
		writeln!(writer, "{}", self.model)?;
		writeln!(writer, "{}", self.n_states)?;
		writeln!(writer, "{}", self.n_inputs)?;
		writeln!(writer, "{}", self.n_outputs)?;
		if self.model == Model::Mealy {
			for (states, outputs) in self
				.state_matrix
				.columns()
				.into_iter()
				.zip(self.output_matrix.columns())
			{
				format_array1(writer, &states)?;
				format_array1(writer, &outputs)?;
			}
		} else {
			format_array1(writer, &self.output_matrix.column(0))?;
			for states in self.state_matrix.columns() {
				format_array1(writer, &states)?;
			}
		}
		Ok(())
	}
}
