mod dot;
mod matrix;
mod minimize;
mod model;
mod simple_text;
use model::Model;
use ndarray::Array2;

pub struct DFA {
	model: Model,
	n_states: usize,
	n_inputs: usize,
	n_outputs: usize,
	output_matrix: Array2<usize>,
	state_matrix: Array2<usize>,
}
